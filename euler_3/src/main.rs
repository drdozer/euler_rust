use hypatia::numbers::{Primes, Factorisation};

fn main() {
    let num = 600_851_475_143u64;

    let fs: Factorisation = Primes::default().factorise(num);
    println!("factors of {} are {:#?}", num, fs.factors);
    println!("largest factor: {}", fs.factors.iter().last().unwrap().prime);
    // println!("largest prime factor of {} is {}", num, largest_prime_factor);
}

