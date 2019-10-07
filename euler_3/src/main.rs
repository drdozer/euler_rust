use hypatia::numbers::{prime_factors, Factor};

fn main() {
    let num = 600_851_475_143u64;

    let fs: Vec<Factor> = prime_factors(num).collect();
    println!("factors of {} are {:#?}", num, fs);
    println!("largest factor: {}", fs.last().unwrap().prime);
    // println!("largest prime factor of {} is {}", num, largest_prime_factor);
}

