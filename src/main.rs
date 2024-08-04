use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// the first input value
    input1: i64,

    /// the second input value
    input2: i64,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", add(cli.input1, cli.input2))
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(33, add(16, 17));
}
