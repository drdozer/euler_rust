use hypatia::numbers::fib;

fn main() {
    let even_sum: u64 = 
        fib().take_while(|&f| f < 4_000_000u64).filter(|&f| f % 2 == 0).sum();
    
    println!("even_sum: {}", even_sum);
}
