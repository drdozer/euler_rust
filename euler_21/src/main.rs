use hypatia::numbers::Primes;

fn main() {
    let mut primes = Primes::default();

    let mut sum_amicables = 0u64;

    for i in 2..10_000 {
        let factors_of_i = primes.factorise(i);
        let sum_of_divisors = factors_of_i.sum_factors() - i;

        // skip self-amicables
        if sum_of_divisors == i { continue; }

        let factors_of_s = primes.factorise(sum_of_divisors);
        let sod_s = factors_of_s.sum_factors() - sum_of_divisors;

        // not sure how to compute only half of each, so let's just add i
        if sod_s == i  { sum_amicables += i }
    }

    println!("Sum of amicables is {}", sum_amicables);
}
