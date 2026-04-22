# petgraph — ETNA Tasks

Total tasks: 16

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `floyd_warshall_undirected_4c7f18e_1` | proptest | `FloydWarshallUndirectedSymmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` |
| 002 | `floyd_warshall_undirected_4c7f18e_1` | quickcheck | `FloydWarshallUndirectedSymmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` |
| 003 | `floyd_warshall_undirected_4c7f18e_1` | crabcheck | `FloydWarshallUndirectedSymmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` |
| 004 | `floyd_warshall_undirected_4c7f18e_1` | hegel | `FloydWarshallUndirectedSymmetric` | `witness_floyd_warshall_undirected_symmetric_case_two_nodes` |
| 005 | `graphmap_incoming_self_loop_e39f0f9_1` | proptest | `GraphmapIncomingSelfLoop` | `witness_graphmap_incoming_self_loop_case_bare` |
| 006 | `graphmap_incoming_self_loop_e39f0f9_1` | quickcheck | `GraphmapIncomingSelfLoop` | `witness_graphmap_incoming_self_loop_case_bare` |
| 007 | `graphmap_incoming_self_loop_e39f0f9_1` | crabcheck | `GraphmapIncomingSelfLoop` | `witness_graphmap_incoming_self_loop_case_bare` |
| 008 | `graphmap_incoming_self_loop_e39f0f9_1` | hegel | `GraphmapIncomingSelfLoop` | `witness_graphmap_incoming_self_loop_case_bare` |
| 009 | `iterative_dfs_order_0fae246_1` | proptest | `DfsReuseNoDuplicateVisits` | `witness_dfs_reuse_no_duplicate_visits_case_line` |
| 010 | `iterative_dfs_order_0fae246_1` | quickcheck | `DfsReuseNoDuplicateVisits` | `witness_dfs_reuse_no_duplicate_visits_case_line` |
| 011 | `iterative_dfs_order_0fae246_1` | crabcheck | `DfsReuseNoDuplicateVisits` | `witness_dfs_reuse_no_duplicate_visits_case_line` |
| 012 | `iterative_dfs_order_0fae246_1` | hegel | `DfsReuseNoDuplicateVisits` | `witness_dfs_reuse_no_duplicate_visits_case_line` |
| 013 | `stable_graph_node_bound_b87cf17_1` | proptest | `StableGraphNodeBoundTight` | `witness_stable_graph_node_bound_tight_case_empty` |
| 014 | `stable_graph_node_bound_b87cf17_1` | quickcheck | `StableGraphNodeBoundTight` | `witness_stable_graph_node_bound_tight_case_empty` |
| 015 | `stable_graph_node_bound_b87cf17_1` | crabcheck | `StableGraphNodeBoundTight` | `witness_stable_graph_node_bound_tight_case_empty` |
| 016 | `stable_graph_node_bound_b87cf17_1` | hegel | `StableGraphNodeBoundTight` | `witness_stable_graph_node_bound_tight_case_empty` |

## Witness Catalog

- `witness_floyd_warshall_undirected_symmetric_case_two_nodes` — base passes, variant fails
- `witness_floyd_warshall_undirected_symmetric_case_triangle` — base passes, variant fails
- `witness_graphmap_incoming_self_loop_case_bare` — base passes, variant fails
- `witness_dfs_reuse_no_duplicate_visits_case_line` — base passes, variant fails
- `witness_dfs_reuse_no_duplicate_visits_case_branch` — base passes, variant fails
- `witness_stable_graph_node_bound_tight_case_empty` — base passes, variant fails
- `witness_stable_graph_node_bound_tight_case_three` — base passes, variant fails
