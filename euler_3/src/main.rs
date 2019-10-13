use hypatia::numbers::{Primes, Factors};

fn main() {
    let num = 600_851_475_143u64;

    let fs: Factors = Primes::default().factorise(num);
    println!("factors of {} are {:#?}", num, fs.vec_ref());
    println!("largest factor: {}", fs.vec_ref().last().unwrap().prime);
    // println!("largest prime factor of {} is {}", num, largest_prime_factor);
}

