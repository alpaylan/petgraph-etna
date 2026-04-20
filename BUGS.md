# petgraph — Injected Bugs

Total mutations: 4

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `iterative_dfs_order` | `iterative_dfs_order_0fae246_1` | `patches/iterative_dfs_order_0fae246_1.patch` | `patch` | `0fae246b07f3dba059a40288c4a7a96588bef19b` |
| 2 | `floyd_warshall_undirected` | `floyd_warshall_undirected_4c7f18e_1` | `patches/floyd_warshall_undirected_4c7f18e_1.patch` | `patch` | `4c7f18e73a730527b4b4fb571190d71d639b6376` |
| 3 | `stable_graph_node_bound` | `stable_graph_node_bound_b87cf17_1` | `patches/stable_graph_node_bound_b87cf17_1.patch` | `patch` | `b87cf17dd5690f8de408015fa014b51c25fbf1d6` |
| 4 | `graphmap_incoming_self_loop` | `graphmap_incoming_self_loop_e39f0f9_1` | `patches/graphmap_incoming_self_loop_e39f0f9_1.patch` | `patch` | `e39f0f9523d4037ad212e89f51e1b7e8434964af` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `iterative_dfs_order_0fae246_1` | `property_dfs_reuse_no_duplicate_visits` | `witness_dfs_reuse_no_duplicate_visits_case_line`, `witness_dfs_reuse_no_duplicate_visits_case_branch` |
| `floyd_warshall_undirected_4c7f18e_1` | `property_floyd_warshall_undirected_symmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes`, `witness_floyd_warshall_undirected_symmetric_case_triangle` |
| `stable_graph_node_bound_b87cf17_1` | `property_stable_graph_node_bound_tight` | `witness_stable_graph_node_bound_tight_case_empty`, `witness_stable_graph_node_bound_tight_case_three` |
| `graphmap_incoming_self_loop_e39f0f9_1` | `property_graphmap_incoming_self_loop` | `witness_graphmap_incoming_self_loop_case_bare` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_dfs_reuse_no_duplicate_visits` | OK | OK | OK | OK |
| `property_floyd_warshall_undirected_symmetric` | OK | OK | OK | OK |
| `property_stable_graph_node_bound_tight` | OK | OK | OK | OK |
| `property_graphmap_incoming_self_loop` | OK | OK | OK | OK |

## Bug Details

### 1. iterative_dfs_order (0fae246_1)
- **Variant**: `iterative_dfs_order_0fae246_1`
- **Location**: `crates/petgraph/src/visit/traversal.rs`, `Dfs::move_to` and `Dfs::next`
- **Property**: `property_dfs_reuse_no_duplicate_visits`
- **Witnesses**: `witness_dfs_reuse_no_duplicate_visits_case_line`, `witness_dfs_reuse_no_duplicate_visits_case_branch`
- **Fix commit**: `0fae246b07f3dba059a40288c4a7a96588bef19b` — "Fix bug in order of iterative dfs"
- **Invariant violated**: When a `Dfs` is reused via `move_to(start)` after a prior full traversal, calling `next()` must never re-emit a node that the shared discovered map already marks as visited.
- **How the mutation triggers**: The buggy implementation pre-visits `start` in `move_to` and drops the pop-time `discovered.visit(node)` gate in `next`, so restarting at an already-discovered node pops the node and returns it a second time. The fix moves the visit check to pop-time and no longer pre-visits in `move_to`.

### 2. floyd_warshall_undirected (4c7f18e_1)
- **Variant**: `floyd_warshall_undirected_4c7f18e_1`
- **Location**: `crates/petgraph/src/algo/floyd_warshall.rs`, edge-seed loop
- **Property**: `property_floyd_warshall_undirected_symmetric`
- **Witnesses**: `witness_floyd_warshall_undirected_symmetric_case_two_nodes`, `witness_floyd_warshall_undirected_symmetric_case_triangle`
- **Fix commit**: `4c7f18e73a730527b4b4fb571190d71d639b6376` — "Fix Floyd-Warshall algorithm behavior toward undirected graphs (#487)"
- **Invariant violated**: On an undirected graph, `floyd_warshall` returns a symmetric distance map: `dist(u, v) == dist(v, u)` for every pair of nodes.
- **How the mutation triggers**: The buggy seed loop records only `dist[source][target]` from each `edge_reference`. On an `UnGraph`, the reverse direction stays at the initial infinity, so every non-self edge becomes an asymmetric seed and the Floyd-Warshall closure never repairs the lower triangle. The fix additionally seeds `dist[target][source]` when the graph is undirected.

### 3. stable_graph_node_bound (b87cf17_1)
- **Variant**: `stable_graph_node_bound_b87cf17_1`
- **Location**: `crates/petgraph/src/graph_impl/stable_graph/mod.rs`, `NodeIndexable::node_bound`
- **Property**: `property_stable_graph_node_bound_tight`
- **Witnesses**: `witness_stable_graph_node_bound_tight_case_empty`, `witness_stable_graph_node_bound_tight_case_three`
- **Fix commit**: `b87cf17dd5690f8de408015fa014b51c25fbf1d6` — "BUG: Fix off by one in StableGraph::node_bound"
- **Invariant violated**: For a `StableGraph` with no removed nodes, `node_bound() == node_count()`. In particular, `node_bound()` of an empty graph is `0`.
- **How the mutation triggers**: The buggy implementation scans the node slab with `rposition(...)` and computes `unwrap_or(0) + 1`, which collapses to `1` on an empty graph and is off by one on dense graphs. The fix uses `node_indices().next_back().map_or(0, |i| i.index() + 1)`, which returns `0` on an empty graph and exactly `node_count()` when the graph has no holes.

### 4. graphmap_incoming_self_loop (e39f0f9_1)
- **Variant**: `graphmap_incoming_self_loop_e39f0f9_1`
- **Location**: `crates/petgraph/src/graphmap.rs`, `NeighborsDirected::next`
- **Property**: `property_graphmap_incoming_self_loop`
- **Witnesses**: `witness_graphmap_incoming_self_loop_case_bare`
- **Fix commit**: `e39f0f9523d4037ad212e89f51e1b7e8434964af` — "FIX: Include self loops in incoming edges"
- **Invariant violated**: For a directed `GraphMap` with a self-loop on node `n`, iterating `neighbors_directed(n, Incoming)` yields `n` at least once.
- **How the mutation triggers**: The buggy iterator filters the internal adjacency stream by direction alone. Self-loops are recorded only with `Outgoing`, so the `Incoming` filter always rejects them. The fix carries the start node into the iterator and accepts entries where the direction matches **or** the neighbor equals the start node (the self-loop case).
