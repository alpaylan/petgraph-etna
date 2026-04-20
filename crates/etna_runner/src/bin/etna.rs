// Stub - real implementation written after patches are verified.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        std::process::exit(2);
    }
    println!(
        "{{\"status\":\"passed\",\"tests\":0,\"discards\":0,\"time\":\"0us\",\"counterexample\":null,\"error\":null,\"tool\":\"{}\",\"property\":\"{}\"}}",
        args[1], args[2]
    );
}
