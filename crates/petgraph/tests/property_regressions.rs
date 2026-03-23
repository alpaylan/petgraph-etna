use std::{
    collections::{HashSet, hash_map::RandomState},
    panic::{AssertUnwindSafe, catch_unwind},
};

use petgraph::{
    Directed, Graph, Undirected,
    algo::{
        all_simple_paths, dominators::simple_fast, floyd_warshall, ford_fulkerson,
        is_isomorphic_matching, is_isomorphic_subgraph, spfa, steiner_tree,
        subgraph_isomorphisms_iter, tarjan_scc,
    },
    csr::Csr,
    graph::NodeIndex,
    graphmap::DiGraphMap,
    stable_graph::StableGraph,
    visit::{
        Dfs, EdgeFiltered, EdgeRef, GetAdjacencyMatrix, GraphProp, IntoEdgeReferences, IntoEdges,
        IntoEdgesDirected, IntoNodeIdentifiers, NodeFiltered, NodeIndexable, Reversed,
        UndirectedAdaptor, Walker,
    },
};

fn assert_adjacency_matrix_consistent<G>(g: G)
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoEdgeReferences + GraphProp,
{
    let matrix = g.adjacency_matrix();
    let node_ids: Vec<G::NodeId> = g.node_identifiers().collect();
    let edges: Vec<(G::NodeId, G::NodeId)> = g
        .edge_references()
        .map(|edge| (edge.source(), edge.target()))
        .collect();

    for &a in &node_ids {
        for &b in &node_ids {
            if edges.contains(&(a, b)) || (!g.is_directed() && edges.contains(&(b, a))) {
                assert!(g.is_adjacent(&matrix, a, b));
            } else {
                assert!(!g.is_adjacent(&matrix, a, b));
            }
        }
    }
}

fn build_queue_sensitive_spfa_graph(scale: i32) -> (Graph<(), i32>, Vec<NodeIndex>) {
    let mut g = Graph::new();
    let ns = (0..5).map(|_| g.add_node(())).collect::<Vec<_>>();
    g.add_edge(ns[0], ns[4], 1000 * scale);
    g.add_edge(ns[0], ns[3], 100 * scale);
    g.add_edge(ns[0], ns[2], 10 * scale);
    g.add_edge(ns[0], ns[1], 1 * scale);
    g.add_edge(ns[3], ns[4], 100 * scale);
    g.add_edge(ns[2], ns[4], 150 * scale);
    g.add_edge(ns[2], ns[3], 10 * scale);
    g.add_edge(ns[1], ns[4], 111 * scale);
    g.add_edge(ns[1], ns[3], 10 * scale);
    (g, ns)
}

#[test]
fn property_csr_add_node_preserves_existing_edges() {
    for extra_nodes in 0..16 {
        let mut g: Csr<(), (), Directed> = Csr::new();
        let a = g.add_node(());
        let b = g.add_node(());
        assert!(g.add_edge(a, b, ()));
        let c = g.add_node(());
        for _ in 0..extra_nodes {
            let n = g.add_node(());
            assert_eq!(g.neighbors_slice(n), &[]);
        }

        assert_eq!(g.neighbors_slice(a), &[b]);
        assert_eq!(g.neighbors_slice(b), &[]);
        assert_eq!(g.neighbors_slice(c), &[]);
        assert_eq!(g.edge_count(), 1);
    }
}

#[test]
fn property_csr_dfs_move_to_reaches_reconnected_component() {
    for _ in 0..8 {
        let mut g: Csr<(), (), Directed> = Csr::new();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        let n4 = g.add_node(());
        let n5 = g.add_node(());

        g.add_edge(n0, n1, ());
        g.add_edge(n0, n2, ());
        g.add_edge(n1, n0, ());
        g.add_edge(n1, n1, ());
        g.add_edge(n1, n3, ());
        g.add_edge(n2, n2, ());
        g.add_edge(n4, n4, ());
        g.add_edge(n4, n5, ());

        let before: HashSet<_> = Dfs::new(&g, n0).iter(&g).collect();
        assert_eq!(before.len(), 4);
        assert!(!before.contains(&n4));
        assert!(!before.contains(&n5));

        g.add_edge(n1, n4, ());
        let after: HashSet<_> = Dfs::new(&g, n0).iter(&g).collect();
        assert_eq!(after.len(), g.node_count());
    }
}

#[test]
fn property_adjacency_matrix_matches_graph_structure() {
    let cases: &[(usize, &[(usize, usize)])] = &[
        (0, &[]),
        (1, &[]),
        (2, &[(0, 0)]),
        (5, &[(0, 2), (0, 4), (1, 3), (3, 4)]),
        (6, &[(2, 3)]),
        (9, &[(1, 4), (2, 8), (3, 7), (4, 8), (5, 8)]),
    ];

    for (order, edges) in cases {
        let mut csr_directed: Csr<(), (), Directed> = Csr::new();
        let mut csr_undirected: Csr<(), (), Undirected> = Csr::new();
        let mut sg_directed: StableGraph<(), (), Directed> =
            StableGraph::with_capacity(*order, edges.len());
        let mut sg_undirected: StableGraph<(), (), Undirected> =
            StableGraph::with_capacity(*order, edges.len());

        for _ in 0..*order {
            csr_directed.add_node(());
            csr_undirected.add_node(());
            sg_directed.add_node(());
            sg_undirected.add_node(());
        }
        for &(a, b) in *edges {
            let a = NodeIndex::new(a);
            let b = NodeIndex::new(b);
            csr_directed.add_edge(a.index() as u32, b.index() as u32, ());
            csr_undirected.add_edge(a.index() as u32, b.index() as u32, ());
            sg_directed.add_edge(a, b, ());
            sg_undirected.add_edge(a, b, ());
        }

        assert_adjacency_matrix_consistent(&csr_directed);
        assert_adjacency_matrix_consistent(&csr_undirected);
        assert_adjacency_matrix_consistent(&sg_directed);
        assert_adjacency_matrix_consistent(&sg_undirected);
    }
}

#[test]
fn property_dominators_exclude_self_from_immediately_dominated_by() {
    for _ in 0..8 {
        let mut g = Graph::<(), (), Directed>::new();
        let root = g.add_node(());
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(root, a, ());
        g.add_edge(a, b, ());
        g.add_edge(root, b, ());
        g.add_edge(b, c, ());

        let doms = simple_fast(&g, root);
        for node in [root, a, b, c] {
            assert!(doms.immediately_dominated_by(node).all(|x| x != node));
        }
    }
}

#[test]
fn property_edgefiltered_reversed_reachability_is_stable() {
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum E {
        A,
        B,
    }

    for _ in 0..8 {
        let mut g = Graph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        let d = g.add_node("D");
        let e = g.add_node("E");
        let f = g.add_node("F");

        g.add_edge(a, b, E::A);
        g.add_edge(b, c, E::A);
        g.add_edge(c, d, E::B);
        g.add_edge(d, e, E::A);
        g.add_edge(e, f, E::A);

        let ef_a = EdgeFiltered::from_fn(&g, |edge| *edge.weight() == E::A);

        let mut dfs = Dfs::new(&Reversed(&ef_a), f);
        let mut reached = HashSet::new();
        while let Some(nx) = dfs.next(&Reversed(&ef_a)) {
            reached.insert(nx);
        }

        assert_eq!(reached, HashSet::from([d, e, f]));
    }
}

#[test]
fn property_steiner_tree_undirected_graph_no_panic_and_bounded_weight() {
    for scale in [1_i32, 2, 3, 4] {
        let mut graph = Graph::<(), i32, Undirected>::new_undirected();
        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());
        let e = graph.add_node(());
        let f = graph.add_node(());
        graph.extend_with_edges([
            (a, b, 7 * scale),
            (a, f, 6 * scale),
            (b, c, 1 * scale),
            (b, f, 5 * scale),
            (c, d, 1 * scale),
            (c, e, 3 * scale),
            (d, e, 1 * scale),
            (d, f, 4 * scale),
            (e, f, 10 * scale),
        ]);

        let terminals = vec![a, c, e, f];
        let tree = steiner_tree(&graph, &terminals);
        let total_weight: i32 = tree.edge_weights().copied().sum();
        assert!(total_weight <= 12 * scale);
        assert!(tree.edge_count() >= terminals.len() - 1);
    }
}

#[test]
fn property_floyd_warshall_distances_are_symmetric_for_undirected_graphs() {
    for scale in [1_i32, 2, 3] {
        let mut g = Graph::<(), i32, Undirected>::new_undirected();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        g.extend_with_edges([
            (n0, n1, 1 * scale),
            (n1, n2, 2 * scale),
            (n2, n3, 3 * scale),
            (n0, n3, 9 * scale),
        ]);

        let d = floyd_warshall(&g, |edge| *edge.weight()).expect("no negative cycle");
        for a in g.node_indices() {
            for b in g.node_indices() {
                assert_eq!(d.get(&(a, b)), d.get(&(b, a)));
            }
        }
    }
}

#[test]
fn property_graphmap_incoming_neighbors_include_self_loops() {
    for _ in 0..8 {
        let mut graph = DiGraphMap::<u32, ()>::new();
        graph.add_node(0);
        graph.add_edge(0, 0, ());
        let neighbors = graph
            .neighbors_directed(0, petgraph::Incoming)
            .collect::<Vec<_>>();
        assert_eq!(neighbors, vec![0]);
    }
}

#[test]
fn property_graphmap_remove_node_clears_all_incident_edges() {
    for n in 3_u32..12 {
        let mut graph = DiGraphMap::<u32, ()>::new();
        for i in 0..n {
            graph.add_node(i);
        }
        for i in 0..(n - 1) {
            graph.add_edge(i, i + 1, ());
            graph.add_edge(i + 1, i, ());
        }
        let removed = n / 2;
        graph.remove_node(removed);

        for (a, b, _) in graph.all_edges() {
            assert_ne!(a, removed);
            assert_ne!(b, removed);
        }
        for i in 0..n {
            if i == removed {
                continue;
            }
            assert!(!graph.neighbors(i).any(|x| x == removed));
        }
    }
}

#[test]
fn property_graphmap_repeated_add_edge_keeps_single_adjacency_entry() {
    for _ in 0..8 {
        let mut graph = DiGraphMap::<u32, i32>::new();
        graph.add_edge(1, 2, 10);
        graph.add_edge(1, 2, 20);
        graph.add_edge(1, 2, 30);

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(
            graph
                .neighbors_directed(1, petgraph::Direction::Outgoing)
                .collect::<Vec<_>>(),
            vec![2]
        );
        assert_eq!(
            graph
                .neighbors_directed(2, petgraph::Direction::Incoming)
                .collect::<Vec<_>>(),
            vec![1]
        );
    }
}

#[test]
fn property_graphmap_directed_self_loop_neighbor_is_emitted_once() {
    for _ in 0..8 {
        let mut graph = DiGraphMap::<u32, ()>::new();
        graph.add_edge(0, 0, ());
        assert_eq!(
            graph
                .neighbors_directed(0, petgraph::Direction::Outgoing)
                .collect::<Vec<_>>(),
            vec![0]
        );
        assert_eq!(
            graph
                .neighbors_directed(0, petgraph::Direction::Incoming)
                .collect::<Vec<_>>(),
            vec![0]
        );
    }
}

#[test]
fn property_subgraph_isomorphism_empty_pattern_yields_single_empty_mapping() {
    for nodes in 3..10 {
        let a = Graph::<(), ()>::new();
        let mut b = Graph::<(), ()>::new();
        let ids = (0..nodes).map(|_| b.add_node(())).collect::<Vec<_>>();
        for w in ids.windows(2) {
            b.add_edge(w[0], w[1], ());
        }

        let mut node_match = |x: &(), y: &()| x == y;
        let mut edge_match = |x: &(), y: &()| x == y;
        let a_ref = &a;
        let b_ref = &b;
        let mut iter =
            subgraph_isomorphisms_iter(&a_ref, &b_ref, &mut node_match, &mut edge_match).unwrap();
        assert_eq!(iter.next(), Some(vec![]));
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn property_subgraph_isomorphism_triangle_embeddings_exist() {
    for extra in 0..6 {
        let pattern = Graph::<(), ()>::from_edges([(0, 1), (1, 2), (2, 0)]);
        let mut target = Graph::<(), ()>::from_edges([(0, 1), (1, 2), (2, 0), (2, 3), (0, 4)]);
        for i in 0..extra {
            let a = target.add_node(());
            let b = target.add_node(());
            target.add_edge(a, b, ());
            target.add_edge(b, a, ());
            if i % 2 == 0 {
                target.add_edge(NodeIndex::new(0), a, ());
            }
        }

        assert!(is_isomorphic_subgraph(&pattern, &target));

        let mut node_match = |x: &(), y: &()| x == y;
        let mut edge_match = |x: &(), y: &()| x == y;
        let mappings =
            subgraph_isomorphisms_iter(&&pattern, &&target, &mut node_match, &mut edge_match)
                .unwrap()
                .collect::<Vec<_>>();
        assert!(!mappings.is_empty());
        assert!(mappings.iter().all(|m| m.len() == 3));
    }
}

#[test]
fn property_subgraph_isomorphism_preserves_small_labeled_path_embedding() {
    for extra in 0..6 {
        let mut g = Graph::<String, ()>::new();
        let l1 = g.add_node("l1".to_string());
        let l2 = g.add_node("l2".to_string());
        g.add_edge(l1, l2, ());
        let l3 = g.add_node("l3".to_string());
        g.add_edge(l2, l3, ());
        let l4 = g.add_node("l4".to_string());
        g.add_edge(l3, l4, ());

        for i in 0..extra {
            let noise = g.add_node(format!("noise_{i}"));
            g.add_edge(l1, noise, ());
        }

        let mut sub = Graph::<String, ()>::new();
        let s3 = sub.add_node("l3".to_string());
        let s4 = sub.add_node("l4".to_string());
        sub.add_edge(s3, s4, ());

        let mut node_match = |x: &String, y: &String| x == y;
        let mut edge_match = |x: &(), y: &()| x == y;
        let mappings = subgraph_isomorphisms_iter(&&sub, &&g, &mut node_match, &mut edge_match)
            .expect("subgraph iterator should exist")
            .collect::<Vec<_>>();
        assert_eq!(mappings, vec![vec![2, 3]]);
    }
}

#[test]
fn property_node_filtered_incoming_edges_respect_node_filter() {
    let mut gr = Graph::<(), (), Directed>::new();
    let nodes = (0..8).map(|_| gr.add_node(())).collect::<Vec<_>>();
    // Crossing edges between low and high index partitions.
    gr.extend_with_edges([
        (nodes[0], nodes[1]),
        (nodes[1], nodes[2]),
        (nodes[2], nodes[3]),
        (nodes[4], nodes[1]),
        (nodes[5], nodes[2]),
        (nodes[3], nodes[6]),
        (nodes[6], nodes[0]),
        (nodes[7], nodes[4]),
    ]);

    for cutoff in 2..7 {
        let filter = |node: NodeIndex| node.index() < cutoff;
        let filtered = NodeFiltered::from_fn(&gr, filter);
        for i in gr.node_indices() {
            let expected = gr
                .edges_directed(i, petgraph::Incoming)
                .filter(|edge| filter(edge.source()) && filter(edge.target()))
                .map(|edge| (edge.source(), edge.target()))
                .collect::<Vec<_>>();
            let actual = filtered
                .edges_directed(i, petgraph::Incoming)
                .map(|edge| (edge.source(), edge.target()))
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn property_simple_paths_node_to_itself_stays_empty_in_complete_digraph() {
    for n in 3..8 {
        let mut g = Graph::<(), (), Directed>::new();
        let nodes = (0..n).map(|_| g.add_node(())).collect::<Vec<_>>();
        for &a in &nodes {
            for &b in &nodes {
                if a != b {
                    g.add_edge(a, b, ());
                }
            }
        }

        let paths: Vec<Vec<_>> =
            all_simple_paths::<Vec<_>, _, RandomState>(&g, nodes[0], nodes[0], 0, None).collect();
        assert!(paths.is_empty());
    }
}

#[test]
fn property_spfa_uses_queue_order_for_shortest_paths() {
    for scale in [1, 2, 3, 5] {
        let (g, ns) = build_queue_sensitive_spfa_graph(scale);
        let spfa_res = spfa(&g, ns[0], |edge| *edge.weight()).expect("spfa should succeed");
        let idx = g.to_index(ns[4]);
        assert_eq!(spfa_res.distances[idx], 111 * scale);
    }
}

#[test]
fn property_undirected_adaptor_traverses_incoming_edges() {
    let linear_edges = [(0_u32, 1_u32), (1, 2), (2, 3), (3, 4), (4, 5)];
    for _ in 0..8 {
        let graph = petgraph::graph::DiGraph::<(), ()>::from_edges(linear_edges);
        let mut nodes = graph.node_identifiers().collect::<Vec<_>>();
        nodes.sort();
        let ungraph = UndirectedAdaptor(&graph);

        let reachable: HashSet<_> = Dfs::new(&ungraph, nodes[2]).iter(&ungraph).collect();
        let all: HashSet<_> = nodes.into_iter().collect();
        assert_eq!(reachable, all);
    }
}

#[test]
fn property_undirected_adaptor_edge_targets_match_undirected_neighbors() {
    for n in 3_u32..10 {
        let mut graph = petgraph::graph::DiGraph::<(), ()>::new();
        let nodes = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
        for i in 0..(n - 1) {
            graph.add_edge(nodes[i as usize], nodes[(i + 1) as usize], ());
        }
        let ungraph = UndirectedAdaptor(&graph);

        for &node in &nodes {
            let expected = graph
                .edges_directed(node, petgraph::Direction::Incoming)
                .map(|edge| edge.source())
                .chain(
                    graph
                        .edges_directed(node, petgraph::Direction::Outgoing)
                        .map(|edge| edge.target()),
                )
                .collect::<HashSet<_>>();
            let actual = ungraph
                .edges(node)
                .map(|edge| edge.target())
                .collect::<HashSet<_>>();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_ford_fulkerson_stable_graph_rewiring_keeps_max_flow_stable() {
    for scale in [1_u32, 2, 3] {
        let mut g: StableGraph<(), u32, Directed> = StableGraph::new();

        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());

        let ac = g.add_edge(a, c, scale);
        let _ = g.add_edge(a, b, scale);
        let bc = g.add_edge(b, c, scale);
        let _ = g.add_edge(b, d, scale);

        g.remove_edge(bc);
        assert_eq!(scale, ford_fulkerson(&g, a, d).0);

        let _ = g.add_edge(b, c, scale);
        g.remove_edge(ac);
        assert_eq!(scale, ford_fulkerson(&g, a, d).0);

        let _ = g.add_edge(a, c, scale);
        let _ = g.add_edge(c, d, scale);
        assert_eq!(2 * scale, ford_fulkerson(&g, a, d).0);
    }
}

#[test]
fn property_astar_admissible_inconsistent_heuristic_still_finds_optimal_path() {
    for scale in [1_i32, 2, 3, 4] {
        let mut g = Graph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        let d = g.add_node("D");
        g.add_edge(a, b, 3 * scale);
        g.add_edge(b, c, 3 * scale);
        g.add_edge(c, d, 3 * scale);
        g.add_edge(a, c, 8 * scale);
        g.add_edge(a, d, 10 * scale);

        let admissible_inconsistent = |n: NodeIndex| match g[n] {
            "A" => 9 * scale,
            "B" => 6 * scale,
            "C" => 0,
            _ => 0,
        };
        let optimal =
            petgraph::algo::astar(&g, a, |n| n == d, |e| *e.weight(), admissible_inconsistent);
        assert_eq!(optimal, Some((9 * scale, vec![a, b, c, d])));
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_stablegraph_add_edge_rejects_vacant_target_nodes() {
    for _ in 0..8 {
        let mut g: StableGraph<(), (), Directed> = StableGraph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let _ = g.add_node(());
        assert!(g.remove_node(b).is_some());

        let add = catch_unwind(AssertUnwindSafe(|| g.add_edge(a, b, ())));
        assert!(add.is_err());
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_stablegraph_find_edge_skips_vacant_nodes() {
    for seed in 1_u64..40 {
        let mut g: StableGraph<(), (), Directed> = StableGraph::new();
        for _ in 0..48 {
            g.add_node(());
        }
        let mut s = seed;
        for _ in 0..800 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = NodeIndex::new(((s >> 16) % 64) as usize);
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            let b = NodeIndex::new(((s >> 16) % 64) as usize);
            match s & 3 {
                0 => {
                    if g.contains_node(a) && g.contains_node(b) {
                        g.add_edge(a, b, ());
                    }
                }
                1 => {
                    if let Some(e) = g.find_edge(a, b) {
                        g.remove_edge(e);
                    }
                }
                2 => {
                    if g.contains_node(a) {
                        g.remove_node(a);
                    }
                }
                _ => {
                    g.add_node(());
                }
            }
            if !g.contains_node(a) || !g.contains_node(b) {
                assert!(g.find_edge(a, b).is_none());
            }
        }
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_stablegraph_neighbors_of_removed_node_are_empty() {
    for _ in 0..8 {
        let mut g: StableGraph<(), (), Directed> = StableGraph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let e = g.add_node(());
        let f = g.add_node(());
        g.add_edge(a, c, ());
        let edge_one = g.add_edge(c, d, ());
        g.add_edge(d, e, ());
        assert_eq!(edge_one.index(), 1);

        assert!(g.remove_node(b).is_some());
        assert!(g.remove_node(f).is_some());
        assert!(!g.contains_node(f));
        assert_eq!(g.neighbors(f).count(), 0);
        assert_eq!(g.neighbors_undirected(f).count(), 0);
        assert_eq!(
            g.neighbors_directed(f, petgraph::Direction::Incoming)
                .count(),
            0
        );
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_stablegraph_edges_of_removed_node_are_empty() {
    for _ in 0..8 {
        let mut g: StableGraph<(), (), Directed> = StableGraph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let e = g.add_node(());
        let f = g.add_node(());
        g.add_edge(a, c, ());
        let edge_one = g.add_edge(c, d, ());
        g.add_edge(d, e, ());
        assert_eq!(edge_one.index(), 1);

        assert!(g.remove_node(b).is_some());
        assert!(g.remove_node(f).is_some());
        assert!(!g.contains_node(f));
        assert_eq!(g.edges(f).count(), 0);
        assert_eq!(
            g.edges_directed(f, petgraph::Direction::Outgoing).count(),
            0
        );
        assert_eq!(
            g.edges_directed(f, petgraph::Direction::Incoming).count(),
            0
        );
    }
}

#[test]
#[cfg(feature = "stable_graph")]
fn property_tarjan_scc_handles_stablegraph_holes() {
    let mut gr: StableGraph<(), ()> = StableGraph::from_edges([
        (6, 0),
        (0, 3),
        (3, 6),
        (8, 6),
        (8, 2),
        (2, 5),
        (5, 8),
        (7, 5),
        (1, 7),
        (7, 4),
        (4, 1),
    ]);
    let x = gr.add_node(());
    gr.add_edge(NodeIndex::new(7), x, ());
    gr.add_edge(x, NodeIndex::new(1), ());
    gr.remove_node(NodeIndex::new(4));

    let mut res = tarjan_scc(&gr);
    for scc in &mut res {
        scc.sort();
    }
    res.sort_by(|v, w| v[0].cmp(&w[0]));

    assert_eq!(
        res,
        vec![
            vec![NodeIndex::new(0), NodeIndex::new(3), NodeIndex::new(6)],
            vec![NodeIndex::new(1), NodeIndex::new(7), x],
            vec![NodeIndex::new(2), NodeIndex::new(5), NodeIndex::new(8)],
        ]
    );
}

#[test]
fn property_isomorphic_matching_respects_directed_edge_semantics() {
    let g0 = Graph::<(), i32, Directed>::from_edges([(0, 0, 1), (0, 1, 2), (0, 2, 3), (1, 2, 4)]);

    let mut g1 = g0.clone();
    let first_edge = g1.edge_indices().next().expect("graph has edges");
    g1[first_edge] = 99;

    assert!(!is_isomorphic_matching(
        &g0,
        &g1,
        |x, y| x == y,
        |x, y| x == y
    ));
}

#[test]
fn property_graph_find_edge_reaches_all_outgoing_edges() {
    for size in 3..10 {
        let mut g = Graph::<(), (), Directed>::new();
        let src = g.add_node(());
        let dsts = (0..size).map(|_| g.add_node(())).collect::<Vec<_>>();

        for &dst in &dsts {
            g.add_edge(src, dst, ());
        }
        for &dst in dsts.iter().rev() {
            g.add_edge(dst, src, ());
        }

        for &dst in &dsts {
            assert!(g.find_edge(src, dst).is_some());
        }
    }
}
