fn fibo(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

fn main() {
    let input: String = std::env::args().nth(1).unwrap();
    let i: i64 = input.parse().unwrap();
    println!("fibo({}) = {}", i, fibo(i));
}
