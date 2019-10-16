use hypatia::numbers::{triangular, Primes};

// The number of factors a number has is related to its prime factors.
// For each primer factor (prime, power), you can chose from 0 to power (inclusive)
// repeats of it. So the total number of factors in a number is the product of
// one more than the powers of the prime factors.
//
// Note:
//
// triangular(n) = n * (n + 1) / 2
// So the factors of trangular(n) are 1, 2, triangular(n) and:
//   factors(n), factors(n+1)
// Similarly, the prime factors of triangular(n) is the (disjoint) union of the factors
//   prime_factors(n), prime_factors(n+1)
// since there can be no prime factors in common between adjacent integers.
//
// We don't optimize for this. It would make sense if we memoised the prime factors
// during the looping, so that we kept the factorisation of (n+1) from the previos
// iteration to act as the factorisation of (n) in the next.
//
// Factorisation could be further sped up by memoising factorisations so that as a big number
// is decomposed into factors, past solutions for smaller numbers could be composed.
//
fn main() {
    let min_count = 500;
    let mut i = 1u64;
    let mut ps = Primes::default();
    loop {
        let tri_i = triangular(i);
        let fs = ps.factorise(tri_i);
        let factor_count = fs.count_divisors();
  
        if factor_count > 500 {
            println!("First triangular number with over {} factors is: {}", min_count, tri_i);
            break;
        }
        i += 1;
    }
}
