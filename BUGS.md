# petgraph Injected Bugs

This workload contains **27** detected bugs derived from historical fixes in [petgraph/petgraph](https://github.com/petgraph/petgraph).

Each bug is injected with [marauders](http://github.com/alpaylan/marauders) comment syntax.
Validation in this document is from explicit `marauders set/reset` + targeted `cargo test` runs.

## How To Inspect

```sh
# list all mutations
marauders list --path workloads/Rust/petgraph

# activate one bug variant
marauders set --path workloads/Rust/petgraph --variant <variant_name>
cargo test -q -p petgraph <test_filter> --manifest-path workloads/Rust/petgraph/Cargo.toml
marauders reset --path workloads/Rust/petgraph
```

## Bug Index

| #   | Name                                                   | Variant                                                          | File                                                          | Type                                               | Failing Tests | Fix Commit                                                                                        |
| --- | ------------------------------------------------------ | ---------------------------------------------------------------- | ------------------------------------------------------------- | -------------------------------------------------- | ------------: | ------------------------------------------------------------------------------------------------- |
| 1   | `csr_add_node_row_resets_offset`                       | `csr_add_node_row_resets_offset_dab2dbf_1`                       | `crates/petgraph/src/csr.rs:288`                              | `csr-row-offset-reset`                             |             1 | [`dab2dbf`](https://github.com/petgraph/petgraph/commit/dab2dbf5aaf574617633c6a7800457b61d2124fb) |
| 2   | `csr_adjacency_matrix_uses_edge_count`                 | `csr_adjacency_matrix_uses_edge_count_7fa3aac_1`                 | `crates/petgraph/src/csr.rs:878`                              | `wrong-adjacency-matrix-dimension`                 |             2 | [`7fa3aac`](https://github.com/petgraph/petgraph/commit/7fa3aac97168de7fca54644a5b45c464d5245535) |
| 3   | `dfs_move_to_marks_visited`                            | `dfs_move_to_marks_visited_0fae246_1`                            | `crates/petgraph/src/visit/traversal.rs:102`                  | `premature-dfs-visited-mark`                       |             3 | [`0fae246`](https://github.com/petgraph/petgraph/commit/0fae246b07f3dba059a40288c4a7a96588bef19b) |
| 4   | `dominators_include_root_self`                         | `dominators_include_root_self_602308d_1`                         | `crates/petgraph/src/algo/dominators.rs:139`                  | `missing-root-self-exclusion`                      |             1 | [`602308d`](https://github.com/petgraph/petgraph/commit/602308d8745f0af34e929d1ec56e370a4a7ff378) |
| 5   | `edgefiltered_directed_neighbor_endpoint`              | `edgefiltered_directed_neighbor_endpoint_1147022_1`              | `crates/petgraph/src/visit/filter.rs:590`                     | `wrong-directed-neighbor-endpoint`                 |             1 | [`1147022`](https://github.com/petgraph/petgraph/commit/1147022802b34624c76849214202a8b610abea14) |
| 6   | `ff_flow_capacity_bound`                               | `ff_flow_capacity_bound_ebee197_1`                               | `crates/petgraph/src/algo/maximum_flow/ford_fulkerson.rs:180` | `wrong-flow-buffer-bound`                          |             1 | [`ebee197`](https://github.com/petgraph/petgraph/commit/ebee19788263054c79450a4c798831053b4a9c48) |
| 7   | `floyd_warshall_undirected_guard_inverted`             | `floyd_warshall_undirected_guard_inverted_4c7f18e_1`             | `crates/petgraph/src/algo/floyd_warshall.rs:321`              | `inverted-undirected-guard`                        |             1 | [`4c7f18e`](https://github.com/petgraph/petgraph/commit/4c7f18e73a730527b4b4fb571190d71d639b6376) |
| 8   | `graph_edges_undirected_self_loops_duplicated`         | `graph_edges_undirected_self_loops_duplicated_af1aa98_1`         | `crates/petgraph/src/graph_impl/mod.rs:2119`                  | `undirected-edge-self-loop-double-visit`           |             1 | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| 9   | `graph_neighbors_directed_skip_start_not_cleared`      | `graph_neighbors_directed_skip_start_not_cleared_af1aa98_1`      | `crates/petgraph/src/graph_impl/mod.rs:940`                   | `directed-neighbor-iterator-cleared`               |             1 | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| 10  | `graph_neighbors_undirected_self_loops_duplicated`     | `graph_neighbors_undirected_self_loops_duplicated_af1aa98_1`     | `crates/petgraph/src/graph_impl/mod.rs:1951`                  | `undirected-neighbor-self-loop-filter-broken`      |             1 | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| 11  | `graph_reverse_node_links_not_swapped`                 | `graph_reverse_node_links_not_swapped_28eb089_1`                 | `crates/petgraph/src/graph_impl/mod.rs:1356`                  | `reverse-node-links-cleared`                       |             1 | [`28eb089`](https://github.com/petgraph/petgraph/commit/28eb08936827bf42bbd650fc09f5545fb55b9a09) |
| 12  | `graph_walkneighbors_undirected_self_loops_duplicated` | `graph_walkneighbors_undirected_self_loops_duplicated_af1aa98_1` | `crates/petgraph/src/graph_impl/mod.rs:2460`                  | `undirected-walkneighbors-self-loop-filter-broken` |             1 | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| 13  | `graphmap_incoming_self_loop_missing`                  | `graphmap_incoming_self_loop_missing_e39f0f9_1`                  | `crates/petgraph/src/graphmap.rs:773`                         | `missing-self-loop-in-incoming-neighbors`          |             1 | [`e39f0f9`](https://github.com/petgraph/petgraph/commit/e39f0f9523d4037ad212e89f51e1b7e8434964af) |
| 14  | `graphmap_remove_node_edge_key_direction`              | `graphmap_remove_node_edge_key_direction_cdbad50_1`              | `crates/petgraph/src/graphmap.rs:313`                         | `wrong-edge-removal-direction`                     |             1 | [`cdbad50`](https://github.com/petgraph/petgraph/commit/cdbad50a5703a1b162a5579bb644c21063f3ceaa) |
| 15  | `isomorphism_empty_iter_override_dropped`              | `isomorphism_empty_iter_override_dropped_d33a613_1`              | `crates/petgraph/src/algo/isomorphism.rs:748`                 | `empty-isomorphism-override-dropped`               |             2 | [`d33a613`](https://github.com/petgraph/petgraph/commit/d33a613f80f3eb5c1b295e0b3cf270bbd3292708) |
| 16  | `isomorphism_subgraph_inssize_strict`                  | `isomorphism_subgraph_inssize_strict_bc0e036_1`                  | `crates/petgraph/src/algo/isomorphism.rs:627`                 | `strict-ins-size-match`                            |             1 | [`bc0e036`](https://github.com/petgraph/petgraph/commit/bc0e036682a1f6e68b8bf91901f4cfc387f05b0d) |
| 17  | `isomorphism_subgraph_outsize_strict`                  | `isomorphism_subgraph_outsize_strict_bc0e036_1`                  | `crates/petgraph/src/algo/isomorphism.rs:620`                 | `strict-out-size-match`                            |             1 | [`bc0e036`](https://github.com/petgraph/petgraph/commit/bc0e036682a1f6e68b8bf91901f4cfc387f05b0d) |
| 18  | `kosaraju_skips_first_identifier`                      | `kosaraju_skips_first_identifier_1596cb2_1`                      | `crates/petgraph/src/algo/scc/kosaraju_scc.rs:106`            | `kosaraju-first-pass-empty-iteration`              |             1 | [`1596cb2`](https://github.com/petgraph/petgraph/commit/1596cb22efcfaf0c4a1e5a05fb693618b3acc478) |
| 19  | `nodefiltered_incoming_uses_target`                    | `nodefiltered_incoming_uses_target_1c26733_1`                    | `crates/petgraph/src/visit/filter.rs:330`                     | `incoming-direction-filter-bypass`                 |             1 | [`1c26733`](https://github.com/petgraph/petgraph/commit/1c267332c76bc612c61750f7f082b917e7141781) |
| 20  | `quickcheck_random01_rng_source`                       | `quickcheck_random01_rng_source_1125c33_1`                       | `crates/petgraph/src/quickcheck.rs:21`                        | `wrong-rng-source`                                 |             1 | [`1125c33`](https://github.com/petgraph/petgraph/commit/1125c33fea0ec3f85d25fc658f1cbe7de7631cd2) |
| 21  | `simple_paths_ignore_visited_target_hint`              | `simple_paths_ignore_visited_target_hint_8f7a0d9_1`              | `crates/petgraph/src/algo/simple_paths.rs:125`                | `missing-visited-target-filter`                    |             1 | [`8f7a0d9`](https://github.com/petgraph/petgraph/commit/8f7a0d93279f575627db90430d1a0aab242c3e9c) |
| 22  | `spfa_queue_pop_back_order`                            | `spfa_queue_pop_back_order_29f4c92_1`                            | `crates/petgraph/src/algo/spfa.rs:139`                        | `lifo-queue-order`                                 |             1 | [`29f4c92`](https://github.com/petgraph/petgraph/commit/29f4c92f5464e5609ddb0fec31c811f46215cbc0) |
| 23  | `stablegraph_node_bound_plus_two`                      | `stablegraph_node_bound_plus_two_b87cf17_1`                      | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:2360`     | `node-bound-off-by-one`                            |             2 | [`b87cf17`](https://github.com/petgraph/petgraph/commit/b87cf17dd5690f8de408015fa014b51c25fbf1d6) |
| 24  | `stablegraph_reverse_edge_free_list_swap`              | `stablegraph_reverse_edge_free_list_swap_b682695_1`              | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:246`      | `reversed-edge-free-list-corruption`               |             1 | [`b682695`](https://github.com/petgraph/petgraph/commit/b682695f29833184b322333e151a588be8f98842) |
| 25  | `stablegraph_reverse_node_free_list_swap`              | `stablegraph_reverse_node_free_list_swap_b682695_1`              | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:258`      | `reversed-node-free-list-corruption`               |             2 | [`b682695`](https://github.com/petgraph/petgraph/commit/b682695f29833184b322333e151a588be8f98842) |
| 26  | `undirected_adaptor_incoming_not_reversed`             | `undirected_adaptor_incoming_not_reversed_01b17d9_1`             | `crates/petgraph/src/visit/undirected_adaptor.rs:41`          | `incoming-edge-orientation-lost`                   |             2 | [`01b17d9`](https://github.com/petgraph/petgraph/commit/01b17d9b6b510e4604aca4f9a59f76b287ed8425) |
| 27  | `unionfind_union_reports_no_merge`                     | `unionfind_union_reports_no_merge_4e0584d_1`                     | `crates/petgraph/src/unionfind.rs:183`                        | `unionfind-union-return-value-broken`              |             1 | [`4e0584d`](https://github.com/petgraph/petgraph/commit/4e0584d5f7b7b1351c780fc89e74ed52fe19d27d) |

## Detector Mapping

| #   | Variant                                                          | Canonical Failing Detector Test                                                        | Status     |
| --- | ---------------------------------------------------------------- | -------------------------------------------------------------------------------------- | ---------- |
| 1   | `csr_add_node_row_resets_offset_dab2dbf_1`                       | `property_csr_add_node_preserves_existing_edges`                                       | `detected` |
| 2   | `csr_adjacency_matrix_uses_edge_count_7fa3aac_1`                 | `property_adjacency_matrix_matches_graph_structure`                                    | `detected` |
| 3   | `dfs_move_to_marks_visited_0fae246_1`                            | `property_csr_dfs_move_to_reaches_reconnected_component`                               | `detected` |
| 4   | `dominators_include_root_self_602308d_1`                         | `property_dominators_exclude_self_from_immediately_dominated_by`                       | `detected` |
| 5   | `edgefiltered_directed_neighbor_endpoint_1147022_1`              | `property_edgefiltered_reversed_reachability_is_stable`                                | `detected` |
| 6   | `ff_flow_capacity_bound_ebee197_1`                               | `property_ford_fulkerson_stable_graph_rewiring_keeps_max_flow_stable`                  | `detected` |
| 7   | `floyd_warshall_undirected_guard_inverted_4c7f18e_1`             | `property_floyd_warshall_distances_are_symmetric_for_undirected_graphs`                | `detected` |
| 8   | `graph_edges_undirected_self_loops_duplicated_af1aa98_1`         | `neighbors_selfloops`                                                                  | `detected` |
| 9   | `graph_neighbors_directed_skip_start_not_cleared_af1aa98_1`      | `neighbors_selfloops`                                                                  | `detected` |
| 10  | `graph_neighbors_undirected_self_loops_duplicated_af1aa98_1`     | `neighbors_selfloops`                                                                  | `detected` |
| 11  | `graph_reverse_node_links_not_swapped_28eb089_1`                 | `test_edge_iterators_directed`                                                         | `detected` |
| 12  | `graph_walkneighbors_undirected_self_loops_duplicated_af1aa98_1` | `neighbors_selfloops`                                                                  | `detected` |
| 13  | `graphmap_incoming_self_loop_missing_e39f0f9_1`                  | `property_graphmap_incoming_neighbors_include_self_loops`                              | `detected` |
| 14  | `graphmap_remove_node_edge_key_direction_cdbad50_1`              | `property_graphmap_remove_node_clears_all_incident_edges`                              | `detected` |
| 15  | `isomorphism_empty_iter_override_dropped_d33a613_1`              | `property_subgraph_isomorphism_empty_pattern_yields_single_empty_mapping`              | `detected` |
| 16  | `isomorphism_subgraph_inssize_strict_bc0e036_1`                  | `property_subgraph_isomorphism_preserves_small_labeled_path_embedding`                 | `detected` |
| 17  | `isomorphism_subgraph_outsize_strict_bc0e036_1`                  | `property_subgraph_isomorphism_triangle_embeddings_exist`                              | `detected` |
| 18  | `kosaraju_skips_first_identifier_1596cb2_1`                      | `matrix_graph::tests::test_kosaraju_scc_with_removed_node`                             | `detected` |
| 19  | `nodefiltered_incoming_uses_target_1c26733_1`                    | `property_node_filtered_incoming_edges_respect_node_filter`                            | `detected` |
| 20  | `quickcheck_random01_rng_source_1125c33_1`                       | `quickcheck::tests::property_random_01_has_non_zero_support_for_small_generator_sizes` | `detected` |
| 21  | `simple_paths_ignore_visited_target_hint_8f7a0d9_1`              | `property_simple_paths_node_to_itself_stays_empty_in_complete_digraph`                 | `detected` |
| 22  | `spfa_queue_pop_back_order_29f4c92_1`                            | `property_spfa_uses_queue_order_for_shortest_paths`                                    | `detected` |
| 23  | `stablegraph_node_bound_plus_two_b87cf17_1`                      | `property_adjacency_matrix_matches_graph_structure`                                    | `detected` |
| 24  | `stablegraph_reverse_edge_free_list_swap_b682695_1`              | `graph_impl::stable_graph::property_reverse_after_removed_edges_preserves_free_lists`  | `detected` |
| 25  | `stablegraph_reverse_node_free_list_swap_b682695_1`              | `graph_impl::stable_graph::property_reverse_after_removed_nodes_preserves_free_lists`  | `detected` |
| 26  | `undirected_adaptor_incoming_not_reversed_01b17d9_1`             | `property_undirected_adaptor_edge_targets_match_undirected_neighbors`                  | `detected` |
| 27  | `unionfind_union_reports_no_merge_4e0584d_1`                     | `uf_rand`                                                                              | `detected` |

## Bug 1: `csr_add_node_row_resets_offset`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/csr.rs:288`                                                                  |
| **Variant**                  | `csr_add_node_row_resets_offset_dab2dbf_1`                                                        |
| **Tags**                     | `csr, add-node, row-offset`                                                                       |
| **Fix commit**               | [`dab2dbf`](https://github.com/petgraph/petgraph/commit/dab2dbf5aaf574617633c6a7800457b61d2124fb) |
| **Fix context**              | Fix bug where adding node to csr graph stole existing edges (Wed Aug 7 21:09:00 2019 +0200)       |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `csr-row-offset-reset`                                                                            |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_csr_add_node_preserves_existing_edges`                                                  |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | adding a node resets CSR row offset and disconnects existing edge windows                         |

**Detecting tests**

- `csr::tests::test_add_node_with_existing_edges`

## Bug 2: `csr_adjacency_matrix_uses_edge_count`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/csr.rs:878`                                                                  |
| **Variant**                  | `csr_adjacency_matrix_uses_edge_count_7fa3aac_1`                                                  |
| **Tags**                     | `csr, adjacency-matrix, dimensions`                                                               |
| **Fix commit**               | [`7fa3aac`](https://github.com/petgraph/petgraph/commit/7fa3aac97168de7fca54644a5b45c464d5245535) |
| **Fix context**              | fix: adjacency matrix for csr and adjacency list (#648) (Sun Dec 29 13:06:51 2024 -0300)          |
| **Related links**            | [PR/Issue #648](https://github.com/petgraph/petgraph/pull/648)                                    |
| **Type**                     | `wrong-adjacency-matrix-dimension`                                                                |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_adjacency_matrix_matches_graph_structure`                                               |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | adjacency matrix capacity uses edge count and panics on node-index writes                         |

**Detecting tests**

- `test_adjacency_matrix_for_csr_directed`
- `test_adjacency_matrix_for_csr_undirected`

## Bug 3: `dfs_move_to_marks_visited`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/visit/traversal.rs:102`                                                      |
| **Variant**                  | `dfs_move_to_marks_visited_0fae246_1`                                                             |
| **Tags**                     | `dfs, move-to, visitation`                                                                        |
| **Fix commit**               | [`0fae246`](https://github.com/petgraph/petgraph/commit/0fae246b07f3dba059a40288c4a7a96588bef19b) |
| **Fix context**              | Fix bug in order of iterative dfs (Thu Dec 19 10:55:53 2019 -0500)                                |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `premature-dfs-visited-mark`                                                                      |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_csr_dfs_move_to_reaches_reconnected_component`                                          |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | move_to pre-marks start node and DFS skips traversal order guarantees                             |

**Detecting tests**

- `csr::tests::csr_dfs`
- `matrix_graph::tests::test_kosaraju_scc_with_removed_node`
- `visit::undirected_adaptor::tests::test_is_reachable`

## Bug 4: `dominators_include_root_self`

|                              |                                                                                                                      |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/dominators.rs:139`                                                                         |
| **Variant**                  | `dominators_include_root_self_602308d_1`                                                                             |
| **Tags**                     | `dominators, root, self-dominance`                                                                                   |
| **Fix commit**               | [`602308d`](https://github.com/petgraph/petgraph/commit/602308d8745f0af34e929d1ec56e370a4a7ff378)                    |
| **Fix context**              | fix: `immediately_dominated_by` function called on root of graph returns root itself (Sun Dec 1 11:26:29 2024 -0500) |
| **Related links**            | (none)                                                                                                               |
| **Type**                     | `missing-root-self-exclusion`                                                                                        |
| **Test mode**                | `debug`                                                                                                              |
| **Failing property test**    | `property_dominators_exclude_self_from_immediately_dominated_by`                                                     |
| **Property detector status** | `detected`                                                                                                           |
| **Failure profile**          | root node reported as immediately dominated by itself                                                                |

**Detecting tests**

- `algo::dominators::tests::test_iter_dominators`

## Bug 5: `edgefiltered_directed_neighbor_endpoint`

|                              |                                                                                                     |
| ---------------------------- | --------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/visit/filter.rs:590`                                                           |
| **Variant**                  | `edgefiltered_directed_neighbor_endpoint_1147022_1`                                                 |
| **Tags**                     | `edgefiltered, neighbors-directed, endpoint`                                                        |
| **Fix commit**               | [`1147022`](https://github.com/petgraph/petgraph/commit/1147022802b34624c76849214202a8b610abea14)   |
| **Fix context**              | Fix IntoNeighborsDirected for EdgeFiltered under undirected graphs (Thu Nov 15 20:31:29 2018 +1100) |
| **Related links**            | (none)                                                                                              |
| **Type**                     | `wrong-directed-neighbor-endpoint`                                                                  |
| **Test mode**                | `debug`                                                                                             |
| **Failing property test**    | `property_edgefiltered_reversed_reachability_is_stable`                                             |
| **Property detector status** | `detected`                                                                                          |
| **Failure profile**          | edge-filtered directed neighbors return wrong endpoint direction                                    |

**Detecting tests**

- `filtered_edge_reverse`

## Bug 6: `ff_flow_capacity_bound`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/maximum_flow/ford_fulkerson.rs:180`                                     |
| **Variant**                  | `ff_flow_capacity_bound_ebee197_1`                                                                |
| **Tags**                     | `flow, capacity, stable-graph`                                                                    |
| **Fix commit**               | [`ebee197`](https://github.com/petgraph/petgraph/commit/ebee19788263054c79450a4c798831053b4a9c48) |
| **Fix context**              | fix: Ford Fulkerson sometimes Panics on StableGraphs (#793) (Tue May 20 15:38:55 2025 +0200)      |
| **Related links**            | [PR/Issue #793](https://github.com/petgraph/petgraph/pull/793)                                    |
| **Type**                     | `wrong-flow-buffer-bound`                                                                         |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_ford_fulkerson_stable_graph_rewiring_keeps_max_flow_stable`                             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | flow buffer too small for stable-graph edge bound; runtime index panic                            |

**Detecting tests**

- `test_ford_fulkerson_stable_graphs`

## Bug 7: `floyd_warshall_undirected_guard_inverted`

|                              |                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------ |
| **File**                     | `crates/petgraph/src/algo/floyd_warshall.rs:321`                                                       |
| **Variant**                  | `floyd_warshall_undirected_guard_inverted_4c7f18e_1`                                                   |
| **Tags**                     | `floyd-warshall, undirected, edge-mirroring`                                                           |
| **Fix commit**               | [`4c7f18e`](https://github.com/petgraph/petgraph/commit/4c7f18e73a730527b4b4fb571190d71d639b6376)      |
| **Fix context**              | Fix Floyd-Warshall algorithm behavior toward undirected graphs (#487) (Fri Jun 10 16:07:34 2022 +0900) |
| **Related links**            | [PR/Issue #487](https://github.com/petgraph/petgraph/pull/487)                                         |
| **Type**                     | `inverted-undirected-guard`                                                                            |
| **Test mode**                | `debug`                                                                                                |
| **Failing property test**    | `property_floyd_warshall_distances_are_symmetric_for_undirected_graphs`                                |
| **Property detector status** | `detected`                                                                                             |
| **Failure profile**          | undirected shortest-path expectations fail due to missing mirrored edge updates                        |

**Detecting tests**

- `algo::steiner_tree::test::test_subgraph_from_metric_closure`

## Bug 8: `graph_edges_undirected_self_loops_duplicated`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/mod.rs:2119`                                                      |
| **Variant**                  | `graph_edges_undirected_self_loops_duplicated_af1aa98_1`                                          |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| **Fix context**              | Fix neighbor and edge iterators visiting self loops twice (Mon Nov 30 20:08:46 2015 +0100)        |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `undirected-edge-self-loop-double-visit`                                                          |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `neighbors_selfloops`                                                                             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | undirected edge iterator skips incoming/self-loop edges incorrectly                               |

**Detecting tests**

- `neighbors_selfloops`

## Bug 9: `graph_neighbors_directed_skip_start_not_cleared`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/mod.rs:940`                                                       |
| **Variant**                  | `graph_neighbors_directed_skip_start_not_cleared_af1aa98_1`                                       |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| **Fix context**              | Fix neighbor and edge iterators visiting self loops twice (Mon Nov 30 20:08:46 2015 +0100)        |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `directed-neighbor-iterator-cleared`                                                              |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `neighbors_selfloops`                                                                             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | directed neighbor iteration clears traversal head and misses reachable neighbors                  |

**Detecting tests**

- `neighbors_selfloops`

## Bug 10: `graph_neighbors_undirected_self_loops_duplicated`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/mod.rs:1951`                                                      |
| **Variant**                  | `graph_neighbors_undirected_self_loops_duplicated_af1aa98_1`                                      |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| **Fix context**              | Fix neighbor and edge iterators visiting self loops twice (Mon Nov 30 20:08:46 2015 +0100)        |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `undirected-neighbor-self-loop-filter-broken`                                                     |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `neighbors_selfloops`                                                                             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | undirected neighbor iterator drops incoming/self-loop neighbors                                   |

**Detecting tests**

- `neighbors_selfloops`

## Bug 11: `graph_reverse_node_links_not_swapped`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/mod.rs:1356`                                                      |
| **Variant**                  | `graph_reverse_node_links_not_swapped_28eb089_1`                                                  |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`28eb089`](https://github.com/petgraph/petgraph/commit/28eb08936827bf42bbd650fc09f5545fb55b9a09) |
| **Fix context**              | Fix buggy Graph::reverse (Wed Nov 25 15:38:59 2015 +0100)                                         |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `reverse-node-links-cleared`                                                                      |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `test_edge_iterators_directed`                                                                    |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | graph reverse drops all node adjacency links and breaks incoming traversal invariants             |

**Detecting tests**

- `test_edge_iterators_directed`

## Bug 12: `graph_walkneighbors_undirected_self_loops_duplicated`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/mod.rs:2460`                                                      |
| **Variant**                  | `graph_walkneighbors_undirected_self_loops_duplicated_af1aa98_1`                                  |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`af1aa98`](https://github.com/petgraph/petgraph/commit/af1aa98ac0709d730e97db3c74059dbc1340eb4e) |
| **Fix context**              | Fix neighbor and edge iterators visiting self loops twice (Mon Nov 30 20:08:46 2015 +0100)        |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `undirected-walkneighbors-self-loop-filter-broken`                                                |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `neighbors_selfloops`                                                                             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | walkneighbors omits incoming neighbors when traversing undirected adjacency                       |

**Detecting tests**

- `neighbors_selfloops`

## Bug 13: `graphmap_incoming_self_loop_missing`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graphmap.rs:773`                                                             |
| **Variant**                  | `graphmap_incoming_self_loop_missing_e39f0f9_1`                                                   |
| **Tags**                     | `graphmap, neighbors-directed, self-loop`                                                         |
| **Fix commit**               | [`e39f0f9`](https://github.com/petgraph/petgraph/commit/e39f0f9523d4037ad212e89f51e1b7e8434964af) |
| **Fix context**              | FIX: Include self loops in incoming edges (Sun Oct 6 10:46:08 2019 +0200)                         |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `missing-self-loop-in-incoming-neighbors`                                                         |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_graphmap_incoming_neighbors_include_self_loops`                                         |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | incoming directed neighbor iterator omits self-loop neighbors                                     |

**Detecting tests**

- `neighbors_incoming_includes_self_loops`

## Bug 14: `graphmap_remove_node_edge_key_direction`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graphmap.rs:313`                                                             |
| **Variant**                  | `graphmap_remove_node_edge_key_direction_cdbad50_1`                                               |
| **Tags**                     | `graphmap, remove-node, edge-key`                                                                 |
| **Fix commit**               | [`cdbad50`](https://github.com/petgraph/petgraph/commit/cdbad50a5703a1b162a5579bb644c21063f3ceaa) |
| **Fix context**              | Fix GraphMap::remove_node not removing some edges (#432) (Sun May 16 20:16:12 2021 +0200)         |
| **Related links**            | [PR/Issue #432](https://github.com/petgraph/petgraph/pull/432)                                    |
| **Type**                     | `wrong-edge-removal-direction`                                                                    |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_graphmap_remove_node_clears_all_incident_edges`                                         |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | remove_node leaves stale edge links for one direction                                             |

**Detecting tests**

- `remove_node`

## Bug 15: `isomorphism_empty_iter_override_dropped`

|                              |                                                                                                          |
| ---------------------------- | -------------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/isomorphism.rs:748`                                                            |
| **Variant**                  | `isomorphism_empty_iter_override_dropped_d33a613_1`                                                      |
| **Tags**                     | `isomorphism, empty-graph, iterator`                                                                     |
| **Fix commit**               | [`d33a613`](https://github.com/petgraph/petgraph/commit/d33a613f80f3eb5c1b295e0b3cf270bbd3292708)        |
| **Fix context**              | fix: Infinite `subgraph_isomorphisms_iter` for empty isomorphisms (#780) (Sat Jul 5 13:20:20 2025 +0200) |
| **Related links**            | [PR/Issue #780](https://github.com/petgraph/petgraph/pull/780)                                           |
| **Type**                     | `empty-isomorphism-override-dropped`                                                                     |
| **Test mode**                | `debug`                                                                                                  |
| **Failing property test**    | `property_subgraph_isomorphism_empty_pattern_yields_single_empty_mapping`                                |
| **Property detector status** | `detected`                                                                                               |
| **Failure profile**          | empty-graph isomorphism mapping is dropped instead of yielded once                                       |

**Detecting tests**

- `iso1`
- `iter_subgraph_empty`

## Bug 16: `isomorphism_subgraph_inssize_strict`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/isomorphism.rs:627`                                                     |
| **Variant**                  | `isomorphism_subgraph_inssize_strict_bc0e036_1`                                                   |
| **Tags**                     | `isomorphism, subgraph, ins-size`                                                                 |
| **Fix commit**               | [`bc0e036`](https://github.com/petgraph/petgraph/commit/bc0e036682a1f6e68b8bf91901f4cfc387f05b0d) |
| **Fix context**              | Fix subgraph isomorphism (#472) (Mon Jul 4 15:26:08 2022 +0300)                                   |
| **Related links**            | [PR/Issue #472](https://github.com/petgraph/petgraph/pull/472)                                    |
| **Type**                     | `strict-ins-size-match`                                                                           |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_subgraph_isomorphism_preserves_small_labeled_path_embedding`                            |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | subgraph iterator misses valid mappings when frontier in-size differs                             |

**Detecting tests**

- `iter_subgraph`

## Bug 17: `isomorphism_subgraph_outsize_strict`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/isomorphism.rs:620`                                                     |
| **Variant**                  | `isomorphism_subgraph_outsize_strict_bc0e036_1`                                                   |
| **Tags**                     | `isomorphism, subgraph, out-size`                                                                 |
| **Fix commit**               | [`bc0e036`](https://github.com/petgraph/petgraph/commit/bc0e036682a1f6e68b8bf91901f4cfc387f05b0d) |
| **Fix context**              | Fix subgraph isomorphism (#472) (Mon Jul 4 15:26:08 2022 +0300)                                   |
| **Related links**            | [PR/Issue #472](https://github.com/petgraph/petgraph/pull/472)                                    |
| **Type**                     | `strict-out-size-match`                                                                           |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_subgraph_isomorphism_triangle_embeddings_exist`                                         |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | subgraph matcher rejects valid mappings when frontier out-size differs                            |

**Detecting tests**

- `iso_subgraph`

## Bug 18: `kosaraju_skips_first_identifier`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/scc/kosaraju_scc.rs:106`                                                |
| **Variant**                  | `kosaraju_skips_first_identifier_1596cb2_1`                                                       |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`1596cb2`](https://github.com/petgraph/petgraph/commit/1596cb22efcfaf0c4a1e5a05fb693618b3acc478) |
| **Fix context**              | scc: Fix bug -- we were skipping node 0 all this time (Fri Jan 16 22:11:58 2015 +0100)            |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `kosaraju-first-pass-empty-iteration`                                                             |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `matrix_graph::tests::test_kosaraju_scc_with_removed_node`                                        |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | kosaraju first phase ignores all node identifiers and yields invalid SCC decomposition            |

**Detecting tests**

- `matrix_graph::tests::test_kosaraju_scc_with_removed_node`

## Bug 19: `nodefiltered_incoming_uses_target`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/visit/filter.rs:330`                                                         |
| **Variant**                  | `nodefiltered_incoming_uses_target_1c26733_1`                                                     |
| **Tags**                     | `filter, incoming, nodefilter`                                                                    |
| **Fix commit**               | [`1c26733`](https://github.com/petgraph/petgraph/commit/1c267332c76bc612c61750f7f082b917e7141781) |
| **Fix context**              | Fix IntoEdgesDirected implementation for NodeFiltered (#476) (Fri Jun 10 03:35:51 2022 -0700)     |
| **Related links**            | [PR/Issue #476](https://github.com/petgraph/petgraph/pull/476)                                    |
| **Type**                     | `incoming-direction-filter-bypass`                                                                |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_node_filtered_incoming_edges_respect_node_filter`                                       |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | incoming edge filter checks wrong endpoint and leaks excluded nodes                               |

**Detecting tests**

- `test_node_filtered_iterators_directed`

## Bug 20: `quickcheck_random01_rng_source`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/quickcheck.rs:21`                                                            |
| **Variant**                  | `quickcheck_random01_rng_source_1125c33_1`                                                        |
| **Tags**                     | `quickcheck, rng, distribution`                                                                   |
| **Fix commit**               | [`1125c33`](https://github.com/petgraph/petgraph/commit/1125c33fea0ec3f85d25fc658f1cbe7de7631cd2) |
| **Fix context**              | fix: Quickcheck random01 function only outputs 0 (#798) (Thu May 29 19:03:48 2025 +0200)          |
| **Related links**            | [PR/Issue #798](https://github.com/petgraph/petgraph/pull/798)                                    |
| **Type**                     | `wrong-rng-source`                                                                                |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `quickcheck::tests::property_random_01_has_non_zero_support_for_small_generator_sizes`            |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | random_01 collapses to zero-sized-generator behavior and fails deterministic regression test      |

**Detecting tests**

- `quickcheck::tests::random_01_not_constant_zero_for_zero_sized_gen`

## Bug 21: `simple_paths_ignore_visited_target_hint`

|                              |                                                                                                                |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/simple_paths.rs:125`                                                                 |
| **Variant**                  | `simple_paths_ignore_visited_target_hint_8f7a0d9_1`                                                            |
| **Tags**                     | `paths, self-loop, target-check`                                                                               |
| **Fix commit**               | [`8f7a0d9`](https://github.com/petgraph/petgraph/commit/8f7a0d93279f575627db90430d1a0aab242c3e9c)              |
| **Fix context**              | feat: Fix self-loop bug in all_simple_paths and enable multiple targets (#865) (Sun Sep 7 19:34:19 2025 +0100) |
| **Related links**            | [PR/Issue #865](https://github.com/petgraph/petgraph/pull/865)                                                 |
| **Type**                     | `missing-visited-target-filter`                                                                                |
| **Test mode**                | `debug`                                                                                                        |
| **Failing property test**    | `property_simple_paths_node_to_itself_stays_empty_in_complete_digraph`                                         |
| **Property detector status** | `detected`                                                                                                     |
| **Failure profile**          | self-target path search incorrectly returns paths through already visited nodes                                |

**Detecting tests**

- `algo::simple_paths::test::test_simple_paths_from_node_to_itself_in_complete_graph`

## Bug 22: `spfa_queue_pop_back_order`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/algo/spfa.rs:139`                                                            |
| **Variant**                  | `spfa_queue_pop_back_order_29f4c92_1`                                                             |
| **Tags**                     | `spfa, queue, traversal-order`                                                                    |
| **Fix commit**               | [`29f4c92`](https://github.com/petgraph/petgraph/commit/29f4c92f5464e5609ddb0fec31c811f46215cbc0) |
| **Fix context**              | fix: use a queue for SPFA  (#893) (Mon Sep 29 03:53:07 2025 +0900)                                |
| **Related links**            | [PR/Issue #893](https://github.com/petgraph/petgraph/pull/893)                                    |
| **Type**                     | `lifo-queue-order`                                                                                |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_spfa_uses_queue_order_for_shortest_paths`                                               |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | SPFA queue becomes stack-like and misses expected shortest-path behavior                          |

**Detecting tests**

- `spfa_no_neg_cycle`

## Bug 23: `stablegraph_node_bound_plus_two`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:2360`                                         |
| **Variant**                  | `stablegraph_node_bound_plus_two_b87cf17_1`                                                       |
| **Tags**                     | `stablegraph, node-bound, off-by-one`                                                             |
| **Fix commit**               | [`b87cf17`](https://github.com/petgraph/petgraph/commit/b87cf17dd5690f8de408015fa014b51c25fbf1d6) |
| **Fix context**              | BUG: Fix off by one in StableGraph::node_bound (Sun Sep 10 22:40:18 2017 +0200)                   |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `node-bound-off-by-one`                                                                           |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `property_adjacency_matrix_matches_graph_structure`                                               |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | node_bound overestimates by one extra index and breaks bound assumptions                          |

**Detecting tests**

- `test_adjacency_matrix_for_stable_graph_directed`
- `test_adjacency_matrix_for_stable_graph_undirected`

## Bug 24: `stablegraph_reverse_edge_free_list_swap`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:246`                                          |
| **Variant**                  | `stablegraph_reverse_edge_free_list_swap_b682695_1`                                               |
| **Tags**                     | `stablegraph, reverse, edge-free-list`                                                            |
| **Fix commit**               | [`b682695`](https://github.com/petgraph/petgraph/commit/b682695f29833184b322333e151a588be8f98842) |
| **Fix context**              | fix: `StableGraph::reverse` breaks free lists (#890) (Mon Sep 29 04:05:05 2025 +0900)             |
| **Related links**            | [PR/Issue #890](https://github.com/petgraph/petgraph/pull/890)                                    |
| **Type**                     | `reversed-edge-free-list-corruption`                                                              |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `graph_impl::stable_graph::property_reverse_after_removed_edges_preserves_free_lists`             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | reverse swaps free edge slots and corrupts free-edge list                                         |

**Detecting tests**

- `graph_impl::stable_graph::test_reverse_after_remove_edges`

## Bug 25: `stablegraph_reverse_node_free_list_swap`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/graph_impl/stable_graph/mod.rs:258`                                          |
| **Variant**                  | `stablegraph_reverse_node_free_list_swap_b682695_1`                                               |
| **Tags**                     | `stablegraph, reverse, node-free-list`                                                            |
| **Fix commit**               | [`b682695`](https://github.com/petgraph/petgraph/commit/b682695f29833184b322333e151a588be8f98842) |
| **Fix context**              | fix: `StableGraph::reverse` breaks free lists (#890) (Mon Sep 29 04:05:05 2025 +0900)             |
| **Related links**            | [PR/Issue #890](https://github.com/petgraph/petgraph/pull/890)                                    |
| **Type**                     | `reversed-node-free-list-corruption`                                                              |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `graph_impl::stable_graph::property_reverse_after_removed_nodes_preserves_free_lists`             |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | reverse swaps free node slots and corrupts free-node list                                         |

**Detecting tests**

- `graph_impl::stable_graph::test_reverse`
- `graph_impl::stable_graph::test_reverse_after_remove_nodes`

## Bug 26: `undirected_adaptor_incoming_not_reversed`

|                              |                                                                                                                                |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **File**                     | `crates/petgraph/src/visit/undirected_adaptor.rs:41`                                                                           |
| **Variant**                  | `undirected_adaptor_incoming_not_reversed_01b17d9_1`                                                                           |
| **Tags**                     | `undirected-adaptor, edges, orientation`                                                                                       |
| **Fix commit**               | [`01b17d9`](https://github.com/petgraph/petgraph/commit/01b17d9b6b510e4604aca4f9a59f76b287ed8425)                              |
| **Fix context**              | fix: Algos don't work on `UndirectedAdaptor` (#870) (#871) (Wed Sep 10 03:40:37 2025 -0400)                                    |
| **Related links**            | [PR/Issue #870](https://github.com/petgraph/petgraph/pull/870), [PR/Issue #871](https://github.com/petgraph/petgraph/pull/871) |
| **Type**                     | `incoming-edge-orientation-lost`                                                                                               |
| **Test mode**                | `debug`                                                                                                                        |
| **Failing property test**    | `property_undirected_adaptor_edge_targets_match_undirected_neighbors`                                                          |
| **Property detector status** | `detected`                                                                                                                     |
| **Failure profile**          | undirected adaptor cannot traverse incoming directed edges correctly                                                           |

**Detecting tests**

- `visit::undirected_adaptor::tests::test_undirected_adaptor_can_traverse`
- `visit::undirected_adaptor::tests::test_undirected_edge_refs_point_both_ways`

## Bug 27: `unionfind_union_reports_no_merge`

|                              |                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------- |
| **File**                     | `crates/petgraph/src/unionfind.rs:183`                                                            |
| **Variant**                  | `unionfind_union_reports_no_merge_4e0584d_1`                                                      |
| **Tags**                     | `(none)`                                                                                          |
| **Fix commit**               | [`4e0584d`](https://github.com/petgraph/petgraph/commit/4e0584d5f7b7b1351c780fc89e74ed52fe19d27d) |
| **Fix context**              | Fix bug in into_labeling (Fri Jan 16 22:11:58 2015 +0100)                                         |
| **Related links**            | (none)                                                                                            |
| **Type**                     | `unionfind-union-return-value-broken`                                                             |
| **Test mode**                | `debug`                                                                                           |
| **Failing property test**    | `uf_rand`                                                                                         |
| **Property detector status** | `detected`                                                                                        |
| **Failure profile**          | union operation mutates structure but always reports false (no merge)                             |

**Detecting tests**

- `uf_rand`

## Sync Notes

- Last synchronized: `2026-03-23 17:56 UTC`.
- Checkpoints synced to `27` final detected mutations: `mutations.json`, `tests.json`, `docs.json`, `report.json`, `manual_mutations.json`.
- This run adds 7 variants from the extension batch (`new_mutation_batch_2026-03-23.json`).
