// Deterministic witness tests for each ETNA variant.
//
// Each witness is a concrete `#[test]` that calls `property_<name>` with
// frozen inputs. On base HEAD: all pass. On each `etna/<variant>` branch
// (or with the relevant patch applied), the witness for that variant
// fails - that's the assertion that makes the bug detectable.

use petgraph::etna::{
    property_dfs_reuse_no_duplicate_visits, property_floyd_warshall_undirected_symmetric,
    property_graphmap_incoming_self_loop, property_stable_graph_node_bound_tight, PropertyResult,
};

fn expect_pass(r: PropertyResult) {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => {}
        PropertyResult::Fail(m) => panic!("property failed: {m}"),
    }
}

// ---------- iterative_dfs_order_0fae246_1 ----------

#[test]
fn witness_dfs_reuse_no_duplicate_visits_case_line() {
    // Line graph 0->1->2. After DFS from 0, all three are discovered.
    // move_to(0) then next() must yield None on the fixed impl. On the
    // buggy impl (move_to pre-visits start but next() doesn't re-check
    // on pop), next() pops 0 and emits it a second time.
    let edges = vec![(0u8, 1u8), (1, 2)];
    expect_pass(property_dfs_reuse_no_duplicate_visits(3, edges, 0, 0));
}

#[test]
fn witness_dfs_reuse_no_duplicate_visits_case_branch() {
    // First DFS from 1 reaches {1, 2}. Then move_to(0) - 0 is fresh,
    // its neighbor 2 is already discovered. Fixed impl emits just 0.
    // Buggy impl: move_to(0) pre-visits 0, pushes 0; next() pops 0,
    // iterates neighbors push 2? 2 is already discovered so push is
    // skipped. next() returns Some(0). That's also one emission, so
    // same as fixed - not a detector for this case. The line case is
    // the real detector; this witness just exercises the reuse path
    // to make sure base behavior is otherwise stable.
    let edges = vec![(0u8, 2u8), (1, 2)];
    expect_pass(property_dfs_reuse_no_duplicate_visits(3, edges, 1, 0));
}

// ---------- floyd_warshall_undirected_4c7f18e_1 ----------

#[test]
fn witness_floyd_warshall_undirected_symmetric_case_two_nodes() {
    // Undirected graph 0 -(5)- 1. Expected dist matrix: all pairs
    // symmetric; dist(0,1) == dist(1,0) == 5. Pre-fix implementation
    // would leave dist(1,0) at infinity because the initial edge seed
    // only filled dist(0,1).
    let edges = vec![(0u8, 1u8, 5u16)];
    expect_pass(property_floyd_warshall_undirected_symmetric(2, edges));
}

#[test]
fn witness_floyd_warshall_undirected_symmetric_case_triangle() {
    // 0 - 1 - 2 - 0, all unit weight. Symmetry must hold on all pairs.
    let edges = vec![(0u8, 1u8, 1u16), (1, 2, 1), (2, 0, 1)];
    expect_pass(property_floyd_warshall_undirected_symmetric(3, edges));
}

// ---------- stable_graph_node_bound_b87cf17_1 ----------

#[test]
fn witness_stable_graph_node_bound_tight_case_empty() {
    // Empty graph: node_count == 0, node_bound must also be 0.
    // Pre-fix returned 1 via rposition(...).unwrap_or(0) + 1.
    expect_pass(property_stable_graph_node_bound_tight(0));
}

#[test]
fn witness_stable_graph_node_bound_tight_case_three() {
    // Three dense nodes: node_count == 3, node_bound == 3.
    // Pre-fix returned 4.
    expect_pass(property_stable_graph_node_bound_tight(3));
}

// ---------- graphmap_incoming_self_loop_e39f0f9_1 ----------

#[test]
fn witness_graphmap_incoming_self_loop_case_bare() {
    // Only a self-loop on node 0. neighbors_directed(0, Incoming) must
    // yield 0 at least once. Pre-fix filtered out self-loops because
    // they were only recorded with Outgoing direction.
    let edges: Vec<(u8, u8)> = vec![];
    expect_pass(property_graphmap_incoming_self_loop(edges));
}
