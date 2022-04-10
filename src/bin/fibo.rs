fn fibo(i: i64) -> i64 {
    match i {
        0 => 0,
        1 => 1,
        _ => fibo(i - 1) + fibo(i - 2)
    }
}

fn main() {
    // Get the
    let input: String = std::env::args().nth(1).unwrap();
    let i: i64 = input.parse().unwrap();
    println!("fibo({}) = {}", i, fibo(i));
}