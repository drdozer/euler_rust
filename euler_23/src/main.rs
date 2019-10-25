use std::collections::HashSet;

use hypatia::numbers::{Primes, Perfection};

fn main() {
    let mut primes = Primes::default();

    let abundants: Vec<u64> = (1..=28123)
        .map(|i| primes.factorise(i))
        .filter(|f| f.perfection() == Perfection::Abundant)
        .map(|f| f.n)
        .collect();
    
    let abundant_sums: HashSet<u64> = abundants.iter().enumerate()
        .flat_map(|(i, a)| abundants.iter().take(i+1).map(move |b| a+b))
        .collect();
    
    let sum_of_non_sums: u64 = (1..=28123)
        .filter(|i| !abundant_sums.contains(i))
        .sum();

    println!("Sum of non-abundant sum integers: {}", sum_of_non_sums);
}


// fixme: this runs embarasingly slowly -- there must be some numerology to speed this up.
// Firstly, the hashset should be replaced with a bitset.
// Then - there should be a more clever way to manage the sum.
//
// Additinally, factorizations should be memoised
//  - once you have factorised a number down to one with a known factorisation, short-circuit
