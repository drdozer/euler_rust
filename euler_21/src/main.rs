use hypatia::numbers::Primes;

fn main() {
    let mut primes = Primes::default();

    let mut sum_amicables = 0u64;

    for i in 2..10_000 {
        let factors_of_i = primes.factorise(i);
        let proper_divisors_sum = factors_of_i.sum_proper_divisors();

        // skip self-amicables
        if proper_divisors_sum == i { continue; }

        let factors_of_s = primes.factorise(proper_divisors_sum);
        let sod_s = factors_of_s.sum_proper_divisors();

        // not sure how to compute only half of each, so let's just add i
        if sod_s == i  { sum_amicables += i }
    }

    println!("Sum of amicables is {}", sum_amicables);
}
