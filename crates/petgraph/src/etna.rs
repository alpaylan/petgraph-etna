//! ETNA framework-neutral property functions for petgraph.
//!
//! Each `property_<name>` is a pure function taking concrete, owned inputs
//! and returning `PropertyResult`. Framework adapters (proptest / quickcheck /
//! crabcheck / hegel) in `crates/etna_runner/src/bin/etna.rs` and witness
//! tests in `crates/petgraph/tests/etna_witnesses.rs` all call these
//! functions directly - the invariants are never re-implemented inside an
//! adapter.

#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(missing_docs)]

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::algo::floyd_warshall;
use crate::graph::{DiGraph, UnGraph};
use crate::graphmap::DiGraphMap;
use crate::stable_graph::StableGraph;
use crate::visit::{Dfs, NodeIndexable};
use crate::{Directed, Direction};

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

fn ok_or(r: bool, msg: impl Into<String>) -> PropertyResult {
    if r {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(msg.into())
    }
}

// ---------------------------------------------------------------------
// property_dfs_reuse_no_duplicate_visits
// ---------------------------------------------------------------------
//
// `Dfs::move_to(x)` is documented as "keep the discovered map, but clear
// the visit stack and restart the dfs from a particular node x". After
// a full DFS from one start completes, calling `move_to(y)` on ANY
// already-discovered node must not cause `next()` to re-emit that node.
// The pre-fix implementation pre-visited the start in `move_to` but
// returned the popped node from `next()` without a pop-time visit check,
// so restarting at an already-discovered node re-emitted it. The fix
// moved the visit check to pop-time and stopped pre-visiting in move_to.
//
// Input: the edge list of a small directed graph plus two start nodes.
// After a full DFS from `start_a`, we call `move_to(start_b)` and
// check that any new emissions are nodes not already emitted.

pub fn property_dfs_reuse_no_duplicate_visits(
    n: u8,
    edges: Vec<(u8, u8)>,
    start_a: u8,
    start_b: u8,
) -> PropertyResult {
    let n = n as usize;
    if n == 0 || n > 8 {
        return PropertyResult::Discard;
    }
    let start_a = start_a as usize;
    let start_b = start_b as usize;
    if start_a >= n || start_b >= n {
        return PropertyResult::Discard;
    }
    let mut g: DiGraph<(), ()> = DiGraph::new();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    for (u, v) in &edges {
        let u = *u as usize;
        let v = *v as usize;
        if u >= n || v >= n {
            return PropertyResult::Discard;
        }
        g.add_edge(nodes[u], nodes[v], ());
    }

    let mut dfs = Dfs::new(&g, nodes[start_a]);
    let mut seen: BTreeSet<_> = BTreeSet::new();
    while let Some(nx) = dfs.next(&g) {
        if !seen.insert(nx) {
            return PropertyResult::Fail(format!(
                "first DFS emitted {:?} twice",
                nx
            ));
        }
        if seen.len() > n {
            return PropertyResult::Fail(format!(
                "first DFS emitted > {} items",
                n
            ));
        }
    }

    // Restart from start_b, reusing the discovered map. move_to may reach
    // either fresh nodes (those unreachable from start_a) or not - but
    // it must NOT re-emit any node already in `seen`.
    dfs.move_to(nodes[start_b]);
    let mut reemit: usize = 0;
    while let Some(nx) = dfs.next(&g) {
        if !seen.insert(nx) {
            return PropertyResult::Fail(format!(
                "move_to({:?}) re-emitted already-discovered {:?}",
                start_b, nx
            ));
        }
        reemit += 1;
        if reemit > n {
            return PropertyResult::Fail(format!(
                "second DFS emitted > {} items",
                n
            ));
        }
    }
    PropertyResult::Pass
}

// ---------------------------------------------------------------------
// property_floyd_warshall_undirected_symmetric
// ---------------------------------------------------------------------
//
// On an UNDIRECTED graph, Floyd-Warshall's distance matrix must be
// symmetric: dist(u, v) == dist(v, u) for every pair. The pre-fix
// implementation only seeded dist[src][tgt] from each edge_reference,
// so the reverse direction stayed at "infinity" and the algorithm
// returned wrong answers. The fix reflects the initial weight into
// both directions when the graph is undirected.
//
// Input: a list of edges with integer weights. We build an UnGraph,
// run Floyd-Warshall, and assert symmetry across every reachable pair.

pub fn property_floyd_warshall_undirected_symmetric(
    n: u8,
    edges: Vec<(u8, u8, u16)>,
) -> PropertyResult {
    let n = n as usize;
    if n == 0 || n > 6 {
        return PropertyResult::Discard;
    }
    let mut g: UnGraph<(), i64> = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    for (u, v, w) in &edges {
        let u = *u as usize;
        let v = *v as usize;
        if u >= n || v >= n {
            return PropertyResult::Discard;
        }
        if u == v {
            // skip self loops; they don't affect symmetry
            continue;
        }
        g.add_edge(nodes[u], nodes[v], i64::from(*w));
    }

    let result = floyd_warshall(&g, |er| *er.weight());
    let dist = match result {
        Ok(d) => d,
        Err(_) => return PropertyResult::Discard, // negative cycle: library contract
    };

    let inf = i64::MAX;
    for i in 0..n {
        for j in 0..n {
            let ni = nodes[i];
            let nj = nodes[j];
            let d_ij = dist.get(&(ni, nj)).copied().unwrap_or(inf);
            let d_ji = dist.get(&(nj, ni)).copied().unwrap_or(inf);
            if d_ij != d_ji {
                return PropertyResult::Fail(format!(
                    "asymmetry: dist({},{})={} but dist({},{})={}",
                    i, j, d_ij, j, i, d_ji
                ));
            }
        }
    }
    PropertyResult::Pass
}

// ---------------------------------------------------------------------
// property_stable_graph_node_bound_tight
// ---------------------------------------------------------------------
//
// For a StableGraph with no removed nodes (no holes), node_bound() must
// equal node_count(). The pre-fix implementation computed
// `rposition(...) + 1` with `unwrap_or(0)`, which collapsed to 1 on an
// empty graph (should be 0) and was off-by-one generally. The fix uses
// `node_indices().next_back().map_or(0, |i| i.index() + 1)`, which
// returns 0 for an empty graph and exactly node_count() when dense.

pub fn property_stable_graph_node_bound_tight(n: u8) -> PropertyResult {
    let n = n as usize;
    if n > 16 {
        return PropertyResult::Discard;
    }
    let mut g: StableGraph<(), (), Directed> = StableGraph::new();
    for _ in 0..n {
        g.add_node(());
    }
    let bound = <StableGraph<(), (), Directed> as NodeIndexable>::node_bound(&g);
    let count = g.node_count();
    ok_or(
        bound == count,
        format!("StableGraph::node_bound={} but node_count={}", bound, count),
    )
}

// ---------------------------------------------------------------------
// property_graphmap_incoming_self_loop
// ---------------------------------------------------------------------
//
// For a directed GraphMap with a self-loop, `neighbors_directed(n, Incoming)`
// must yield `n` at least once. The pre-fix implementation filtered
// the internal adjacency iterator on direction alone; because self-loops
// are stored with `Outgoing`, they never matched the `Incoming` filter.
// The fix carries the start node down into the iterator and unions
// matching-direction neighbors with the start node itself.
//
// Input: a small set of directed edges. We add a self-loop on node 0
// and assert it shows up as an incoming neighbor. Non-self-loop edges
// are optional context.

pub fn property_graphmap_incoming_self_loop(edges: Vec<(u8, u8)>) -> PropertyResult {
    let mut g: DiGraphMap<u8, ()> = DiGraphMap::new();
    // Always add a deterministic self-loop on node 0.
    g.add_edge(0u8, 0u8, ());
    for (u, v) in &edges {
        // Bound to keep the graph small and deterministic.
        if *u > 15 || *v > 15 {
            return PropertyResult::Discard;
        }
        g.add_edge(*u, *v, ());
    }
    let incoming: Vec<u8> = g.neighbors_directed(0u8, Direction::Incoming).collect();
    if !incoming.contains(&0u8) {
        return PropertyResult::Fail(
            "GraphMap::neighbors_directed(0, Incoming) missed self-loop".to_string(),
        );
    }
    PropertyResult::Pass
}
