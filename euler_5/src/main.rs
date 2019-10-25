use itertools::Itertools;
use hypatia::numbers::{Factor, Primes};

fn main() {
    let mut primes = Primes::default();

    // manually implemented
    // let mut all_factors: Vec<Factor> = (1..=20)
    //     .map(|n| primes.factorise(n))
    //     .flat_map::<Vec<_>, _>(|f| f.factors.iter().cloned().collect())
    //     .collect();
    // all_factors.sort_by_key(|f| f.prime);

    // let grouped: Vec<_> = all_factors.iter()
    //     .group_by(|f| f.prime)
    //     .into_iter()
    //     .map(|(_, fs)| fs.max_by_key(|f| f.power).unwrap())
    //     .collect();

    // let product_of_factors: u64 = grouped.iter()
    //     .map(|f| f.calculate())
    //     .product();

    // println!("Grouped factors: {:#?}", grouped);
    // println!("Product of factors: {}", product_of_factors);

    // Using smallest common multiple
    let fact_2 = primes.factorise(2);
    let scm = (3..=20)
        .map(|n| primes.factorise(n).factors)
        .fold(fact_2.factors, |l, r| l.smallest_common_multiple(&r));
    
    println!("Smallest common multiple factors: {:#?}", scm);
    println!("As value: {}", scm.factorisation().n)

}
