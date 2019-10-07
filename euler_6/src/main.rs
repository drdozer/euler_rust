use hypatia::numbers::triangular;

fn main() {
    let num = 100;

    let sq_sm = square_of_sums(num);
    let sm_sq = sum_of_squares(num);
    let dif = sq_sm - sm_sq;

    println!("Square of sums: {}", sq_sm);
    println!("Sum of squares: {}", sm_sq);
    println!("Difference: {}", dif);
}

// square of sum is
// (1 + 2 + ... + n)^2
// which is triangular(n)^2
fn square_of_sums(n: u64) -> u64 {
    let sum = triangular(n);
    sum * sum
}

fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|i| i*i).sum()
}