use hypatia::numbers::{primes, Primes};


fn main() {
    let n = 10001;

    // 2 is prime 1, not prime 0, so we need element n-1
    let ps_n = primes().nth(n-1).unwrap();

    println!("Prime using primes {} is {}", n, ps_n);

    let ps_2_n = Primes::default().iter().nth(n-1).unwrap();

    println!("Prime using Primes {} is {}", n, ps_2_n);
}
