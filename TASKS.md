# petgraph — ETNA Tasks

Total tasks: 16

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task. The `<PropertyKey>` token in the command column uses the PascalCase key recognised by `crates/etna_runner/src/bin/etna.rs`; passing `All` runs every property for the named framework in a single invocation.

## Property keys

| Property | PropertyKey |
|----------|-------------|
| `property_dfs_reuse_no_duplicate_visits` | `DfsReuseNoDuplicateVisits` |
| `property_floyd_warshall_undirected_symmetric` | `FloydWarshallUndirectedSymmetric` |
| `property_stable_graph_node_bound_tight` | `StableGraphNodeBoundTight` |
| `property_graphmap_incoming_self_loop` | `GraphmapIncomingSelfLoop` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `iterative_dfs_order_0fae246_1` | proptest | `property_dfs_reuse_no_duplicate_visits` | `witness_dfs_reuse_no_duplicate_visits_case_line` | `cargo run --release --bin etna -- proptest DfsReuseNoDuplicateVisits` |
| 002 | `iterative_dfs_order_0fae246_1` | quickcheck | `property_dfs_reuse_no_duplicate_visits` | `witness_dfs_reuse_no_duplicate_visits_case_line` | `cargo run --release --bin etna -- quickcheck DfsReuseNoDuplicateVisits` |
| 003 | `iterative_dfs_order_0fae246_1` | crabcheck | `property_dfs_reuse_no_duplicate_visits` | `witness_dfs_reuse_no_duplicate_visits_case_line` | `cargo run --release --bin etna -- crabcheck DfsReuseNoDuplicateVisits` |
| 004 | `iterative_dfs_order_0fae246_1` | hegel | `property_dfs_reuse_no_duplicate_visits` | `witness_dfs_reuse_no_duplicate_visits_case_line` | `cargo run --release --bin etna -- hegel DfsReuseNoDuplicateVisits` |
| 005 | `floyd_warshall_undirected_4c7f18e_1` | proptest | `property_floyd_warshall_undirected_symmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` | `cargo run --release --bin etna -- proptest FloydWarshallUndirectedSymmetric` |
| 006 | `floyd_warshall_undirected_4c7f18e_1` | quickcheck | `property_floyd_warshall_undirected_symmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` | `cargo run --release --bin etna -- quickcheck FloydWarshallUndirectedSymmetric` |
| 007 | `floyd_warshall_undirected_4c7f18e_1` | crabcheck | `property_floyd_warshall_undirected_symmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` | `cargo run --release --bin etna -- crabcheck FloydWarshallUndirectedSymmetric` |
| 008 | `floyd_warshall_undirected_4c7f18e_1` | hegel | `property_floyd_warshall_undirected_symmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` | `cargo run --release --bin etna -- hegel FloydWarshallUndirectedSymmetric` |
| 009 | `stable_graph_node_bound_b87cf17_1` | proptest | `property_stable_graph_node_bound_tight` | `witness_stable_graph_node_bound_tight_case_empty` | `cargo run --release --bin etna -- proptest StableGraphNodeBoundTight` |
| 010 | `stable_graph_node_bound_b87cf17_1` | quickcheck | `property_stable_graph_node_bound_tight` | `witness_stable_graph_node_bound_tight_case_empty` | `cargo run --release --bin etna -- quickcheck StableGraphNodeBoundTight` |
| 011 | `stable_graph_node_bound_b87cf17_1` | crabcheck | `property_stable_graph_node_bound_tight` | `witness_stable_graph_node_bound_tight_case_empty` | `cargo run --release --bin etna -- crabcheck StableGraphNodeBoundTight` |
| 012 | `stable_graph_node_bound_b87cf17_1` | hegel | `property_stable_graph_node_bound_tight` | `witness_stable_graph_node_bound_tight_case_empty` | `cargo run --release --bin etna -- hegel StableGraphNodeBoundTight` |
| 013 | `graphmap_incoming_self_loop_e39f0f9_1` | proptest | `property_graphmap_incoming_self_loop` | `witness_graphmap_incoming_self_loop_case_bare` | `cargo run --release --bin etna -- proptest GraphmapIncomingSelfLoop` |
| 014 | `graphmap_incoming_self_loop_e39f0f9_1` | quickcheck | `property_graphmap_incoming_self_loop` | `witness_graphmap_incoming_self_loop_case_bare` | `cargo run --release --bin etna -- quickcheck GraphmapIncomingSelfLoop` |
| 015 | `graphmap_incoming_self_loop_e39f0f9_1` | crabcheck | `property_graphmap_incoming_self_loop` | `witness_graphmap_incoming_self_loop_case_bare` | `cargo run --release --bin etna -- crabcheck GraphmapIncomingSelfLoop` |
| 016 | `graphmap_incoming_self_loop_e39f0f9_1` | hegel | `property_graphmap_incoming_self_loop` | `witness_graphmap_incoming_self_loop_case_bare` | `cargo run --release --bin etna -- hegel GraphmapIncomingSelfLoop` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_dfs_reuse_no_duplicate_visits_case_line` — edges `[(0,1),(1,2)]`, `start_a=0`, `start_b=0` → after full DFS from 0, `move_to(0)` + `next()` must yield `None` (buggy impl re-emits node 0).
- `witness_dfs_reuse_no_duplicate_visits_case_branch` — edges `[(0,2),(1,2)]`, `start_a=1`, `start_b=0` → exercises the reuse path on an already-discovered neighbor; base behavior is one new emission.
- `witness_floyd_warshall_undirected_symmetric_case_two_nodes` — undirected edge `(0,1,5)` → `dist(0,1) == dist(1,0) == 5` (buggy impl leaves `dist(1,0) = ∞`).
- `witness_floyd_warshall_undirected_symmetric_case_triangle` — triangle `(0,1,1),(1,2,1),(2,0,1)` → symmetry must hold on every pair.
- `witness_stable_graph_node_bound_tight_case_empty` — `n = 0` → `node_bound == 0` (buggy impl returns `1`).
- `witness_stable_graph_node_bound_tight_case_three` — `n = 3` dense → `node_bound == 3` (buggy impl returns `4`).
- `witness_graphmap_incoming_self_loop_case_bare` — no extra edges, just a self-loop on node `0` → `neighbors_directed(0, Incoming)` contains `0` (buggy impl misses it).
