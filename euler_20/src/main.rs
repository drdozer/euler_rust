use num::cast::FromPrimitive;
use num::BigUint;

fn main() {
    let fact_100 = factorial(100);
    println!("Factorial 100: {}", fact_100);
    let fact_100_string = fact_100.to_str_radix(10);
    println!("Factorial 100 as a string: {}", fact_100_string);
    let dig_sum = fact_100_string.chars().map(|d| d.to_string().parse::<u32>().unwrap()).sum::<u32>();
    println!("  as digits: {}", dig_sum);
}

fn factorial(n: usize) -> BigUint {
    let mut f = BigUint::from_u64(1).unwrap();

    for i in 1..=n {
        f *= i
    }

    f
}