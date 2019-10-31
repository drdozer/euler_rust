use hypatia::numbers::factorial;

fn main() {
    const P: usize = 1_000_000;
    const N: usize = 10;

    // The project euler puzzle counts permutations from 1, not from 0.
    // This is a pain for us to compute with, so we subtract 1 instead and count perumutations from 0.
    let p_n = perm(P-1, N);

    println!("Permutation {} of the digits before {} is {:?}", P, N, p_n);

    // for p in 0..24 {
    //     let n = 4;
    //     println!("Perumtation {} of the digits before {} is {:?}", p, n, perm(p, n));
    // }
}


// Lexographic perumutation p of n items `0, 1, ..., n-1`
//
// There are `1*2*...*n=factorial(n)` ways to arange `n` items.
//
// Our lexographic ordering will increment the most significant digit last.
// For every increment of digit `i` (counting from 1),
// there must have been an entire loop through all `factorial(i-1)` arrangements of the less significant digits.
//
// This means that the p'th 'count' of digit `i` is `p / factorial(i-1)`.
//
// Our digits count from 0 up to `n-1`.
// There will only ever be one possible choice for the 0th digit, 2 for the 1st digit and so on.
// If the count is larger than that, it means the digit has wrapped.
// So the counter needs to be modulo `i+1`.
//
// `counter(i+1) = (p / factorial(i)) mod (i+1)`
//
// This tells us the counter at each position.
//
// By keeping a list of the unused digits in lexographic order, we can then pull out the one at that count.
// As the digit is used, it is removed from this remaining digits list so that it is not re-used.
fn perm(p: usize, n: usize) -> Vec<usize> {
    let mut counter: Vec<usize> = factorial()
        .enumerate()
        .map(|(i, f)| {
            // println!("Digit with prefix length {} has {} permutations.", i, f);
            // println!("The {}th count of these is {}", p, p / f);
            // println!("There are only {} alternatives for this digit, so we've counted to alternative {} of these.", {i as u64+1}, (p/f) % (i as u64+1)); 
            (p / f as usize) % (i+1)}
            )
        .take(n)
        .collect();

    counter.reverse();
    
    // println!("counters in perm order: {:?}", counter);
    let mut digits: Vec<usize> = (0..n).collect();
    for i in 0..n {
        let c = counter[i];
        let d = digits[c];
        // println!("Selecting item {} of {:?} as {}", c, digits, d);
        digits.retain(|&x| x != d);
        counter[i] = d;
    }

    // println!("digits: {:?}", counter);

    counter
}
