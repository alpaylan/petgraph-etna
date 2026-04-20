// ETNA workload runner for petgraph.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: DfsReuseNoDuplicateVisits | FloydWarshallUndirectedSymmetric |
//             StableGraphNodeBoundTight | GraphmapIncomingSelfLoop | All
//
// Every invocation prints exactly one JSON line to stdout and exits 0
// (except argv parsing, which exits 2).

use crabcheck::quickcheck as crabcheck_qc;
use crabcheck::quickcheck::Arbitrary as CcArbitrary;
use hegel::{generators as hgen, HealthCheck, Hegel, Settings as HegelSettings, TestCase};
use petgraph::etna::{
    property_dfs_reuse_no_duplicate_visits, property_floyd_warshall_undirected_symmetric,
    property_graphmap_incoming_self_loop, property_stable_graph_node_bound_tight, PropertyResult,
};
use proptest::prelude::*;
use proptest::test_runner::{Config as ProptestConfig, TestCaseError, TestError, TestRunner};
use quickcheck::{Arbitrary as QcArbitrary, Gen, QuickCheck, ResultStatus, TestResult};
use rand::Rng;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "DfsReuseNoDuplicateVisits",
    "FloydWarshallUndirectedSymmetric",
    "StableGraphNodeBoundTight",
    "GraphmapIncomingSelfLoop",
];

fn cases_budget() -> u64 {
    std::env::var("ETNA_CASES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(u64::MAX)
}

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if let Err(e) = r {
            return (Err(e), total);
        }
    }
    (Ok(()), total)
}

// ---------- Canonical witness builders (for the `etna` replay tool) ----------

fn check_dfs_reuse() -> Result<(), String> {
    to_err(property_dfs_reuse_no_duplicate_visits(
        3,
        vec![(0, 1), (1, 2)],
        0,
        0,
    ))
}

fn check_floyd_warshall() -> Result<(), String> {
    to_err(property_floyd_warshall_undirected_symmetric(
        2,
        vec![(0, 1, 5)],
    ))
}

fn check_stable_graph_node_bound() -> Result<(), String> {
    to_err(property_stable_graph_node_bound_tight(0))
}

fn check_graphmap_self_loop() -> Result<(), String> {
    to_err(property_graphmap_incoming_self_loop(Vec::new()))
}

fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "DfsReuseNoDuplicateVisits" => check_dfs_reuse(),
        "FloydWarshallUndirectedSymmetric" => check_floyd_warshall(),
        "StableGraphNodeBoundTight" => check_stable_graph_node_bound(),
        "GraphmapIncomingSelfLoop" => check_graphmap_self_loop(),
        _ => {
            return (
                Err(format!("Unknown property for etna: {property}")),
                Metrics::default(),
            );
        }
    };
    (
        result,
        Metrics {
            inputs: 1,
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------- shared input newtypes ----------
//
// Each newtype wraps the concrete arguments of a property so that each
// framework's Arbitrary impl can pick the same generator, and so that
// counterexamples serialize through `Debug`.

#[derive(Clone)]
struct DfsInput {
    n: u8,
    edges: Vec<(u8, u8)>,
    start_a: u8,
    start_b: u8,
}

#[derive(Clone)]
struct FloydInput {
    n: u8,
    edges: Vec<(u8, u8, u16)>,
}

#[derive(Clone, Copy)]
struct StableGraphN(u8);

#[derive(Clone)]
struct GraphmapEdges(Vec<(u8, u8)>);

impl fmt::Debug for DfsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "n={} edges={:?} start_a={} start_b={}",
            self.n, self.edges, self.start_a, self.start_b
        )
    }
}
impl fmt::Debug for FloydInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n={} edges={:?}", self.n, self.edges)
    }
}
impl fmt::Debug for StableGraphN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for GraphmapEdges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Display for DfsInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl fmt::Display for FloydInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl fmt::Display for StableGraphN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Display for GraphmapEdges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

// ---------- rand-based generators (crabcheck + hegel helpers share this logic) ----------

fn random_dfs_input<R: Rng>(rng: &mut R) -> DfsInput {
    // n in 1..=8 keeps the property's Discard clause from firing too often.
    let n = rng.random_range(1u8..=8);
    let e_count = rng.random_range(0usize..=(n as usize * 2));
    let edges: Vec<(u8, u8)> = (0..e_count)
        .map(|_| (rng.random_range(0..n), rng.random_range(0..n)))
        .collect();
    let start_a = rng.random_range(0..n);
    let start_b = rng.random_range(0..n);
    DfsInput {
        n,
        edges,
        start_a,
        start_b,
    }
}

fn random_floyd_input<R: Rng>(rng: &mut R) -> FloydInput {
    let n = rng.random_range(1u8..=6);
    let e_count = rng.random_range(0usize..=(n as usize * 2));
    let edges: Vec<(u8, u8, u16)> = (0..e_count)
        .map(|_| {
            (
                rng.random_range(0..n),
                rng.random_range(0..n),
                rng.random_range(0u16..=20),
            )
        })
        .collect();
    FloydInput { n, edges }
}

fn random_stable_n<R: Rng>(rng: &mut R) -> u8 {
    rng.random_range(0u8..=10)
}

fn random_graphmap_edges<R: Rng>(rng: &mut R) -> Vec<(u8, u8)> {
    let len = rng.random_range(0usize..=6);
    (0..len)
        .map(|_| (rng.random_range(0u8..=8), rng.random_range(0u8..=8)))
        .collect()
}

// ---------- quickcheck Arbitrary ----------

impl QcArbitrary for DfsInput {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = g.random_range(1u8..=8);
        let e_count = g.random_range(0usize..=(n as usize * 2));
        let edges: Vec<(u8, u8)> = (0..e_count)
            .map(|_| (g.random_range(0..n), g.random_range(0..n)))
            .collect();
        let start_a = g.random_range(0..n);
        let start_b = g.random_range(0..n);
        DfsInput {
            n,
            edges,
            start_a,
            start_b,
        }
    }
}

impl QcArbitrary for FloydInput {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = g.random_range(1u8..=6);
        let e_count = g.random_range(0usize..=(n as usize * 2));
        let edges: Vec<(u8, u8, u16)> = (0..e_count)
            .map(|_| {
                (
                    g.random_range(0..n),
                    g.random_range(0..n),
                    g.random_range(0u16..=20),
                )
            })
            .collect();
        FloydInput { n, edges }
    }
}

impl QcArbitrary for StableGraphN {
    fn arbitrary(g: &mut Gen) -> Self {
        StableGraphN(g.random_range(0u8..=10))
    }
}

impl QcArbitrary for GraphmapEdges {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.random_range(0usize..=6);
        let v: Vec<(u8, u8)> = (0..len)
            .map(|_| (g.random_range(0u8..=8), g.random_range(0u8..=8)))
            .collect();
        GraphmapEdges(v)
    }
}

// ---------- crabcheck Arbitrary ----------

impl<R: Rng> CcArbitrary<R> for DfsInput {
    fn generate(rng: &mut R, _n: usize) -> Self {
        random_dfs_input(rng)
    }
}
impl<R: Rng> CcArbitrary<R> for FloydInput {
    fn generate(rng: &mut R, _n: usize) -> Self {
        random_floyd_input(rng)
    }
}
impl<R: Rng> CcArbitrary<R> for StableGraphN {
    fn generate(rng: &mut R, _n: usize) -> Self {
        StableGraphN(random_stable_n(rng))
    }
}
impl<R: Rng> CcArbitrary<R> for GraphmapEdges {
    fn generate(rng: &mut R, _n: usize) -> Self {
        GraphmapEdges(random_graphmap_edges(rng))
    }
}

// ---------- proptest strategies ----------

fn dfs_strategy() -> BoxedStrategy<DfsInput> {
    (1u8..=8u8)
        .prop_flat_map(|n| {
            (
                Just(n),
                prop::collection::vec((0u8..n, 0u8..n), 0..=(n as usize * 2)),
                0u8..n,
                0u8..n,
            )
        })
        .prop_map(|(n, edges, start_a, start_b)| DfsInput {
            n,
            edges,
            start_a,
            start_b,
        })
        .boxed()
}

fn floyd_strategy() -> BoxedStrategy<FloydInput> {
    (1u8..=6u8)
        .prop_flat_map(|n| {
            (
                Just(n),
                prop::collection::vec(
                    (0u8..n, 0u8..n, 0u16..=20u16),
                    0..=(n as usize * 2),
                ),
            )
        })
        .prop_map(|(n, edges)| FloydInput { n, edges })
        .boxed()
}

fn stable_n_strategy() -> BoxedStrategy<u8> {
    (0u8..=10u8).boxed()
}

fn graphmap_edges_strategy() -> BoxedStrategy<Vec<(u8, u8)>> {
    prop::collection::vec((0u8..=8u8, 0u8..=8u8), 0..=6).boxed()
}

fn run_proptest_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let cfg = ProptestConfig {
        cases: cases_budget() as u32,
        max_shrink_iters: 32,
        failure_persistence: None,
        ..ProptestConfig::default()
    };
    let mut runner = TestRunner::new(cfg);
    let c = counter.clone();
    let result: Result<(), String> = match property {
        "DfsReuseNoDuplicateVisits" => runner
            .run(&dfs_strategy(), move |inp| {
                c.fetch_add(1, Ordering::Relaxed);
                let cex = inp.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_dfs_reuse_no_duplicate_visits(
                        inp.n,
                        inp.edges,
                        inp.start_a,
                        inp.start_b,
                    )
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({:?})", cex)))
                    }
                }
            })
            .map_err(|e| match e {
                TestError::Fail(reason, _) => reason.to_string(),
                other => other.to_string(),
            }),
        "FloydWarshallUndirectedSymmetric" => runner
            .run(&floyd_strategy(), move |inp| {
                c.fetch_add(1, Ordering::Relaxed);
                let cex = inp.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_floyd_warshall_undirected_symmetric(inp.n, inp.edges)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({:?})", cex)))
                    }
                }
            })
            .map_err(|e| match e {
                TestError::Fail(reason, _) => reason.to_string(),
                other => other.to_string(),
            }),
        "StableGraphNodeBoundTight" => runner
            .run(&stable_n_strategy(), move |n| {
                c.fetch_add(1, Ordering::Relaxed);
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_stable_graph_node_bound_tight(n)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({})", n)))
                    }
                }
            })
            .map_err(|e| match e {
                TestError::Fail(reason, _) => reason.to_string(),
                other => other.to_string(),
            }),
        "GraphmapIncomingSelfLoop" => runner
            .run(&graphmap_edges_strategy(), move |edges| {
                c.fetch_add(1, Ordering::Relaxed);
                let cex = edges.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_graphmap_incoming_self_loop(edges)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({:?})", cex)))
                    }
                }
            })
            .map_err(|e| match e {
                TestError::Fail(reason, _) => reason.to_string(),
                other => other.to_string(),
            }),
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ---------- quickcheck (forked crate with `etna` feature) ----------

static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn qc_dfs(inp: DfsInput) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_dfs_reuse_no_duplicate_visits(inp.n, inp.edges, inp.start_a, inp.start_b) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_floyd(inp: FloydInput) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_floyd_warshall_undirected_symmetric(inp.n, inp.edges) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_stable_bound(StableGraphN(n): StableGraphN) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_stable_graph_node_bound_tight(n) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_graphmap(GraphmapEdges(e): GraphmapEdges) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_graphmap_incoming_self_loop(e) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let budget = cases_budget();
    let mut qc = QuickCheck::new()
        .tests(budget)
        .max_tests(budget.saturating_mul(2))
        .max_time(Duration::from_secs(86_400));
    let result = match property {
        "DfsReuseNoDuplicateVisits" => qc.quicktest(qc_dfs as fn(DfsInput) -> TestResult),
        "FloydWarshallUndirectedSymmetric" => {
            qc.quicktest(qc_floyd as fn(FloydInput) -> TestResult)
        }
        "StableGraphNodeBoundTight" => {
            qc.quicktest(qc_stable_bound as fn(StableGraphN) -> TestResult)
        }
        "GraphmapIncomingSelfLoop" => {
            qc.quicktest(qc_graphmap as fn(GraphmapEdges) -> TestResult)
        }
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        ResultStatus::Aborted { err } => Err(format!("quickcheck aborted: {err:?}")),
        ResultStatus::TimedOut => Err("quickcheck timed out".to_string()),
        ResultStatus::GaveUp => Err(format!(
            "quickcheck gave up after {} tests",
            result.n_tests_passed
        )),
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- crabcheck ----------

static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cc_dfs(inp: DfsInput) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_dfs_reuse_no_duplicate_visits(inp.n, inp.edges, inp.start_a, inp.start_b) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_floyd(inp: FloydInput) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_floyd_warshall_undirected_symmetric(inp.n, inp.edges) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_stable_bound(StableGraphN(n): StableGraphN) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_stable_graph_node_bound_tight(n) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_graphmap(GraphmapEdges(e): GraphmapEdges) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_graphmap_incoming_self_loop(e) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let cc_config = crabcheck_qc::Config {
        tests: cases_budget(),
    };
    let result = match property {
        "DfsReuseNoDuplicateVisits" => crabcheck_qc::quickcheck_with_config(cc_config, cc_dfs),
        "FloydWarshallUndirectedSymmetric" => {
            crabcheck_qc::quickcheck_with_config(cc_config, cc_floyd)
        }
        "StableGraphNodeBoundTight" => {
            crabcheck_qc::quickcheck_with_config(cc_config, cc_stable_bound)
        }
        "GraphmapIncomingSelfLoop" => {
            crabcheck_qc::quickcheck_with_config(cc_config, cc_graphmap)
        }
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        crabcheck_qc::ResultStatus::Finished => Ok(()),
        crabcheck_qc::ResultStatus::Failed { arguments } => {
            Err(format!("({})", arguments.join(" ")))
        }
        crabcheck_qc::ResultStatus::TimedOut => Err("crabcheck timed out".to_string()),
        crabcheck_qc::ResultStatus::GaveUp => Err(format!(
            "crabcheck gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        crabcheck_qc::ResultStatus::Aborted { error } => {
            Err(format!("crabcheck aborted: {error}"))
        }
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- hegel ----------

static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> HegelSettings {
    HegelSettings::new()
        .test_cases(cases_budget())
        .suppress_health_check(HealthCheck::all())
}

fn hg_draw_dfs(tc: &TestCase) -> DfsInput {
    let n = tc.draw(hgen::integers::<u8>().min_value(1).max_value(8));
    let e_count = tc.draw(
        hgen::integers::<usize>()
            .min_value(0)
            .max_value(n as usize * 2),
    );
    let mut edges = Vec::with_capacity(e_count);
    for _ in 0..e_count {
        let u = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
        let v = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
        edges.push((u, v));
    }
    let start_a = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
    let start_b = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
    DfsInput {
        n,
        edges,
        start_a,
        start_b,
    }
}

fn hg_draw_floyd(tc: &TestCase) -> FloydInput {
    let n = tc.draw(hgen::integers::<u8>().min_value(1).max_value(6));
    let e_count = tc.draw(
        hgen::integers::<usize>()
            .min_value(0)
            .max_value(n as usize * 2),
    );
    let mut edges = Vec::with_capacity(e_count);
    for _ in 0..e_count {
        let u = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
        let v = tc.draw(hgen::integers::<u8>().min_value(0).max_value(n - 1));
        let w = tc.draw(hgen::integers::<u16>().min_value(0).max_value(20));
        edges.push((u, v, w));
    }
    FloydInput { n, edges }
}

fn hg_draw_stable_n(tc: &TestCase) -> u8 {
    tc.draw(hgen::integers::<u8>().min_value(0).max_value(10))
}

fn hg_draw_graphmap_edges(tc: &TestCase) -> Vec<(u8, u8)> {
    let len = tc.draw(hgen::integers::<usize>().min_value(0).max_value(6));
    let mut edges = Vec::with_capacity(len);
    for _ in 0..len {
        let u = tc.draw(hgen::integers::<u8>().min_value(0).max_value(8));
        let v = tc.draw(hgen::integers::<u8>().min_value(0).max_value(8));
        edges.push((u, v));
    }
    edges
}

fn run_hegel_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match property {
        "DfsReuseNoDuplicateVisits" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let inp = hg_draw_dfs(&tc);
                let cex = inp.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_dfs_reuse_no_duplicate_visits(
                        inp.n,
                        inp.edges,
                        inp.start_a,
                        inp.start_b,
                    )
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?})", cex),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "FloydWarshallUndirectedSymmetric" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let inp = hg_draw_floyd(&tc);
                let cex = inp.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_floyd_warshall_undirected_symmetric(inp.n, inp.edges)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?})", cex),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "StableGraphNodeBoundTight" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = hg_draw_stable_n(&tc);
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_stable_graph_node_bound_tight(n)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({})", n),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "GraphmapIncomingSelfLoop" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let edges = hg_draw_graphmap_edges(&tc);
                let cex = edges.clone();
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    property_graphmap_incoming_self_loop(edges)
                }));
                match outcome {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("({:?})", cex),
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{}", property),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(msg
                .strip_prefix("Property test failed: ")
                .unwrap_or(&msg)
                .to_string())
        }
    };
    (status, metrics)
}

// ---------- dispatch ----------

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (
            Err(format!("Unknown tool: {tool}")),
            Metrics::default(),
        ),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: DfsReuseNoDuplicateVisits | FloydWarshallUndirectedSymmetric | StableGraphNodeBoundTight | GraphmapIncomingSelfLoop | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(tool, property, "aborted", Metrics::default(), None, Some(&msg));
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(e) => emit_json(tool, property, "failed", metrics, Some(&e), None),
    }
}
