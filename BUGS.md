# petgraph — Injected Bugs

Total mutations: 4

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `floyd_warshall_undirected_4c7f18e_1` | `floyd_warshall_undirected` | `crates/petgraph/src/algo/floyd_warshall.rs` | `patch` | `4c7f18e73a730527b4b4fb571190d71d639b6376` |
| 2 | `graphmap_incoming_self_loop_e39f0f9_1` | `graphmap_incoming_self_loop` | `crates/petgraph/src/graphmap.rs` | `patch` | `e39f0f9523d4037ad212e89f51e1b7e8434964af` |
| 3 | `iterative_dfs_order_0fae246_1` | `iterative_dfs_order` | `crates/petgraph/src/visit/traversal.rs` | `patch` | `0fae246b07f3dba059a40288c4a7a96588bef19b` |
| 4 | `stable_graph_node_bound_b87cf17_1` | `stable_graph_node_bound` | `crates/petgraph/src/graph_impl/stable_graph/mod.rs` | `patch` | `b87cf17dd5690f8de408015fa014b51c25fbf1d6` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `floyd_warshall_undirected_4c7f18e_1` | `FloydWarshallUndirectedSymmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes`, `witness_floyd_warshall_undirected_symmetric_case_triangle` |
| `graphmap_incoming_self_loop_e39f0f9_1` | `GraphmapIncomingSelfLoop` | `witness_graphmap_incoming_self_loop_case_bare` |
| `iterative_dfs_order_0fae246_1` | `DfsReuseNoDuplicateVisits` | `witness_dfs_reuse_no_duplicate_visits_case_line`, `witness_dfs_reuse_no_duplicate_visits_case_branch` |
| `stable_graph_node_bound_b87cf17_1` | `StableGraphNodeBoundTight` | `witness_stable_graph_node_bound_tight_case_empty`, `witness_stable_graph_node_bound_tight_case_three` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `FloydWarshallUndirectedSymmetric` | ✓ | ✓ | ✓ | ✓ |
| `GraphmapIncomingSelfLoop` | ✓ | ✓ | ✓ | ✓ |
| `DfsReuseNoDuplicateVisits` | ✓ | ✓ | ✓ | ✓ |
| `StableGraphNodeBoundTight` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. floyd_warshall_undirected

- **Variant**: `floyd_warshall_undirected_4c7f18e_1`
- **Location**: `crates/petgraph/src/algo/floyd_warshall.rs`
- **Property**: `FloydWarshallUndirectedSymmetric`
- **Witness(es)**:
  - `witness_floyd_warshall_undirected_symmetric_case_two_nodes`
  - `witness_floyd_warshall_undirected_symmetric_case_triangle`
- **Source**: Fix Floyd-Warshall algorithm behavior toward undirected graphs (#487)
  > `floyd_warshall` seeded only `dist[source][target]` from each edge reference, leaving the reverse entries at infinity on undirected graphs; the Floyd-Warshall closure never repaired the asymmetric seeds, producing a non-symmetric distance map.
- **Fix commit**: `4c7f18e73a730527b4b4fb571190d71d639b6376` — Fix Floyd-Warshall algorithm behavior toward undirected graphs (#487)
- **Invariant violated**: On an undirected graph, `floyd_warshall` returns a symmetric distance map: `dist(u, v) == dist(v, u)` for every pair of nodes.
- **How the mutation triggers**: The buggy seed loop records only `dist[source][target]` from each `edge_reference`. On an `UnGraph`, the reverse direction stays at the initial infinity, so every non-self edge becomes an asymmetric seed and the Floyd-Warshall closure never repairs the lower triangle. The fix additionally seeds `dist[target][source]` when the graph is undirected.

### 2. graphmap_incoming_self_loop

- **Variant**: `graphmap_incoming_self_loop_e39f0f9_1`
- **Location**: `crates/petgraph/src/graphmap.rs`
- **Property**: `GraphmapIncomingSelfLoop`
- **Witness(es)**:
  - `witness_graphmap_incoming_self_loop_case_bare`
- **Source**: FIX: Include self loops in incoming edges
  > `GraphMap::neighbors_directed(n, Incoming)` filtered the adjacency stream by direction only; self-loops are stored with `Outgoing`, so the Incoming filter rejected them, and `neighbors_directed(n, Incoming)` on a self-looping node produced an empty iterator.
- **Fix commit**: `e39f0f9523d4037ad212e89f51e1b7e8434964af` — FIX: Include self loops in incoming edges
- **Invariant violated**: For a directed `GraphMap` with a self-loop on node `n`, iterating `neighbors_directed(n, Incoming)` yields `n` at least once.
- **How the mutation triggers**: The buggy iterator filters the internal adjacency stream by direction alone. Self-loops are recorded only with `Outgoing`, so the `Incoming` filter always rejects them. The fix carries the start node into the iterator and accepts entries where the direction matches **or** the neighbor equals the start node (the self-loop case).

### 3. iterative_dfs_order

- **Variant**: `iterative_dfs_order_0fae246_1`
- **Location**: `crates/petgraph/src/visit/traversal.rs`
- **Property**: `DfsReuseNoDuplicateVisits`
- **Witness(es)**:
  - `witness_dfs_reuse_no_duplicate_visits_case_line`
  - `witness_dfs_reuse_no_duplicate_visits_case_branch`
- **Source**: Fix bug in order of iterative dfs
  > `Dfs::move_to` pre-visited the new start node while `next` dropped the pop-time `discovered.visit(node)` gate; reusing a Dfs via `move_to` after a full traversal re-emitted the already-discovered start node. The fix keeps the visit check at pop-time and no longer pre-visits.
- **Fix commit**: `0fae246b07f3dba059a40288c4a7a96588bef19b` — Fix bug in order of iterative dfs
- **Invariant violated**: When a `Dfs` is reused via `move_to(start)` after a prior full traversal, calling `next()` must never re-emit a node that the shared discovered map already marks as visited.
- **How the mutation triggers**: The buggy implementation pre-visits `start` in `move_to` and drops the pop-time `discovered.visit(node)` gate in `next`, so restarting at an already-discovered node pops the node and returns it a second time. The fix moves the visit check to pop-time and no longer pre-visits in `move_to`.

### 4. stable_graph_node_bound

- **Variant**: `stable_graph_node_bound_b87cf17_1`
- **Location**: `crates/petgraph/src/graph_impl/stable_graph/mod.rs`
- **Property**: `StableGraphNodeBoundTight`
- **Witness(es)**:
  - `witness_stable_graph_node_bound_tight_case_empty`
  - `witness_stable_graph_node_bound_tight_case_three`
- **Source**: BUG: Fix off by one in StableGraph::node_bound
  > `StableGraph::node_bound` used `rposition(...).unwrap_or(0) + 1`, which collapses to `1` on an empty graph and is off by one on dense graphs; the fix uses `next_back().map_or(0, |i| i.index() + 1)`.
- **Fix commit**: `b87cf17dd5690f8de408015fa014b51c25fbf1d6` — BUG: Fix off by one in StableGraph::node_bound
- **Invariant violated**: For a `StableGraph` with no removed nodes, `node_bound() == node_count()`. In particular, `node_bound()` of an empty graph is `0`.
- **How the mutation triggers**: The buggy implementation scans the node slab with `rposition(...)` and computes `unwrap_or(0) + 1`, which collapses to `1` on an empty graph and is off by one on dense graphs. The fix uses `node_indices().next_back().map_or(0, |i| i.index() + 1)`, which returns `0` on an empty graph and exactly `node_count()` when the graph has no holes.
