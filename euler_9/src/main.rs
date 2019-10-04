
// pythagorous tripple
//
// a,b,c where:
//  a^2 + b^2 = c^2
//
// and a,b,c > 0
//
// n-sum where:
//  a + b + c = n
//
// it follows that:
//  a<c, b<c, c < n
//
// we can further take advantage that the triple (i, j, c) is not distinct from (j, i, c)
// to impose a well-behaved ordering such that we only consider:
//
//  a <= b
//
// This gives us the full bounds:
//  a <= b < c < n
//
// Then, given a + b + c = n and choices for n, c, b, we can calulate a as:
//  a = n - c - b
//
// The value of c must be strictly greater than:
//  n/3
// and no more than
//  n-2 (for the case where a = b = 1)
// The value of b must be at least:
//  (n-c)/2
fn main() {
    let n = 1000u64;

    for c in (n/3)..=(n-2) {
        let c2 = c*c;
        let n_c = n - c;

        for b in (n_c/2 + n_c%2)..(n_c) {
            let b2 = b*b;
            let a = n - c - b;
            assert!(a > 0, "Factor a should be positive but was {} for {} and {}", a, b, c);
            assert!(a <= b, "Factor a {} should be no more than factor b {} at c {} with resudual {}", a, b, c, n_c);
            let a2 = a*a;
            let sum = a + b + c;
            assert!(sum == n, "Sum of factors {}, {}, {} should be {} rather than {}", a, b, c, n, sum);
            // println!("Considering {} {} {} with {} {} {}", a, b, c, a2, b2, c2);
            if a2 + b2 == c2 {
                println!("Found pythagorous triple: {} {} {}.", a, b, c);
                let prod = a * b * c;
                println!("Product of factors: {}", prod);
            }
        }
    }
}
