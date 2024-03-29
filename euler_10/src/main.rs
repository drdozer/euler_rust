use hypatia::numbers::Primes;

fn main() {
    let limit = 2_000_000u64;

    let sum = Primes::default().iter().take_while(|p| *p < limit).sum::<u64>();
    println!("Sum of primes below {} is {}", limit, sum);
}
