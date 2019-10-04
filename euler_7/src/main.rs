use hypatia::primes;


fn main() {
    let n = 10001;

    // 2 is prime 1, not prime 0, so we need element n-1
    let ps_n = primes().nth(n-1).unwrap();

    println!("Prime {} is {}", n, ps_n);
}
