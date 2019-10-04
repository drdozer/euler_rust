use itertools::Itertools;
use hypatia::prime_factors;

fn main() {
    let mut all_factors: Vec<_> = (1..=20)
        .flat_map(|i| prime_factors(i))
        .collect::<Vec<_>>();
    all_factors.sort_by_key(|f| f.prime);

    let grouped: Vec<_> = all_factors.iter()
        .group_by(|f| f.prime)
        .into_iter()
        .map(|(_, fs)| fs.max_by_key(|f| f.power).unwrap())
        .collect();

    let product_of_factors: u64 = grouped.iter()
        .map(|f| f.calculate())
        .product();

    println!("Grouped factors: {:#?}", grouped);
    println!("Product of factors: {}", product_of_factors);
}
