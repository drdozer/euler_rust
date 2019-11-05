use hypatia::decimal::{Decimal};

fn main() {
    println!("First 12 fibs are: {:?}", Decimal::fib().enumerate().take(12).collect::<Vec<_>>());

    let (i, f) = Decimal::fib()
        .enumerate()
        .filter(|(_, f)| f.digits().len() >= 1000).next().unwrap();
    println!("First thousand digit fib is {} with {} digits", i+1, f.digits().len());
}
