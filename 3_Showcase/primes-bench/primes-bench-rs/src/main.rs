use std::env;
use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let limit = (n as f64).sqrt() as u64;
    let mut i = 3u64;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    let n: u64 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000_000);

    let start = Instant::now();
    // single-threaded version
    let count: u64 = (2..=n).into_iter().map(|x| is_prime(x) as u64).sum();
    // parrallel version
    // use rayon::prelude::*;
    // let count: u64 = (2..=n).into_par_iter().map(|x| is_prime(x) as u64).sum();

    let elapsed = start.elapsed();

    println!("primes up to {}: {}", n, count);
    println!("elapsed: {:.3?}", elapsed);
}
