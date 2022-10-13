use clap::Parser;

#[derive(Parser)]
struct Args {
    input: i64,
}

fn fibo(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

fn main() {
    let args = Args::parse();
    println!("fibo({}) = {}", args.input, fibo(args.input));
}
