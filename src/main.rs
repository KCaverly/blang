use blang::repl::REPL;

fn main() {
    let repl = REPL::new(">> ".to_string());
    repl.run();
}
