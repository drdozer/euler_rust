use hypatia::Decimal;

fn main() {
    let d_2 = Decimal::from_u32(2);
    let expo = 1000;

    let mut pow = Decimal::from_u32(1);
    for _ in 0..expo {
        pow = &pow * &d_2;
    }

    println!("2^{} = {}", expo, pow);
    let s: u64 = pow.digits().iter().map(|&d| u64::from(d)).sum();
    println!("Digit sum: {}", s);
}
