use std::cmp::Ordering;

pub fn triangular(n: u64) -> u64 {
    ((n * n) + n) / 2
}

pub fn fib() -> impl Iterator<Item = u64> {
    let mut f0 = 0;
    let mut f1 = 1;

    std::iter::from_fn(move || {
        let f = f0 + f1;
        f0 = f1;
        f1 = f;
        Some(f)
    })
}

// pub fn factorial<N>() -> impl Iterator<Item = u128> {
//     let mut f = 1;
//     let mut i = 0;

//     std::iter::from_fn(move || {
//         println!("factorial at {}, {}", i, f);
//         let f0 = f;
//         i +=1;
//         f *= i;
//         Some(f0)
//     })
// }

#[derive(Debug)]
pub struct Factorisation {
    pub n: u64,
    pub factors: Factors
}

impl Factorisation {

    // The proper divisors exclude the number itself.
    pub fn sum_proper_divisors(&self) -> u64 {
        self.factors.sum_divisors() - self.n
    }

    // Check if the number is perfect or not.
    pub fn perfection(&self) -> Perfection {
        match self.sum_proper_divisors().cmp(&self.n) {
            Ordering::Less => Perfection::Deficient,
            Ordering::Equal => Perfection::Perfect,
            Ordering::Greater => Perfection::Abundant,
        }
    }

}

#[derive(Debug)]
pub struct Factors(Vec<Factor>);

impl Factors {
    pub fn iter(&self) -> std::slice::Iter<'_, Factor> { self.0.iter() }

    // Convert this factors into the equivalent factorisation.
    //
    // This calculates `n` by products.
    // It consumes this object, since we should really either be working in a field with `n` being carried around, or be working directly on the factors, but not flipping between them.
    pub fn factorisation(self) -> Factorisation {
        Factorisation {
            n: self.iter().map(Factor::calculate).product(),
            factors: self
        }
    }

    // The count of divisors can be calulated efficiently from the prime powers.
    // For each prime in the prime factors with a power of p, it can contribute p+1 modulo families of divisors.
    // Therefore, the total number of divisors a number has is the product of p+1 for each prime factor power.
    pub fn count_divisors(&self) -> u32 {
        self.iter().map(|f| f.power + 1).product::<u32>()
    }

    // The sum of all divisors can be calculated efficiently from the primes and their powers.
    //
    // The divisors can be found by exhaustively looping over each power of each prime as a big series of nested loops.
    // In the inner-most loop, take the product of all the contributions of the primes.
    // Then sum over each of these loops.
    //
    // This is expensive!
    //
    // Alternatively, take advantage of distribution laws.
    // Given the sum of the powers of a single prime factor (e.g. `7,2` becomes `7^0+7^1+7^2`), take the product of these.
    // This works because `a0*b0 + a0*b1 + a1*b0 + a1*b1 + ... = (a0 + a1 + ...) * (b0 + b1 + ...)`.
    // so we reduce the number of sums from polynomial to linear over the factors.
    //
    // We an further optimize this, since the sum of powers is a geometric sum, hence we can solve it analytically without looping.
    pub fn sum_divisors(&self) -> u64 {
        self.iter().map(Factor::power_sum).product()
    }

    // Iterate over two prime factorisations, returning the factors and their powers in each of the inputs, in order.
    //
    // For factors that appear in one or the other fatorisation but not both, the power in the absent one is 0.
    pub fn factors_iterator<'a>(&'a self, other: &'a Self) -> impl 'a + Iterator<Item=(u64, (u32, u32))> {
        let mut lhs = self.iter();
        let mut rhs = other.iter();

        let mut l_next = lhs.next();
        let mut r_next = rhs.next();

        std::iter::from_fn(move ||
            match (l_next, r_next) {
                (Some(l), Some(r)) => {
                    if l.prime < r.prime {
                        l_next = lhs.next();
                        Some((l.prime, (l.power, 0)))
                    } else if l.prime > r.prime {
                        r_next = rhs.next();
                        Some((r.prime, (0, r.power)))
                    } else {
                        l_next = lhs.next();
                        r_next = rhs.next();
                        Some((l.prime, (l.power, r.power)))
                    }
                }
                (Some(l), None) => {
                    l_next = lhs.next();
                    Some((l.prime, (l.power, 0)))
                }
                (None, Some(r)) => {
                    r_next = rhs.next();
                    Some((r.prime, (0, r.power)))
                }
                (None, None) => None
            }
        )
    }

    // The largest common factor of two numbers.
    //
    // If two numbers share a prime factor, then the largest common factor contains the smallest power.
    // In the special case where a prime is a factor of one and not the other, it can not be part of the common factor.
    // This is equivalent to filtering out all factors where the minimum power is zero.
    pub fn largest_common_factor(&self, other: &Self) -> Self {
        Factors(self.factors_iterator(other)
            .map(|(pr, (l, r))| Factor { prime: pr, power: l.min(r) })
            .filter(|&f| f.power != 0)
            .collect()
        )
    }

    // The smallest common multiple of two numbers.
    //
    // For each prime factor of two numbers, the smallest common multiple is each prime factor raised to the largest power found in either one.
    pub fn smallest_common_multiple(&self, other: &Self) -> Self {
        Factors(self.factors_iterator(other)
            .map(|(pr, (l, r))| Factor { prime: pr, power: l.max(r) })
            .collect()
        )
    }
}

// Multiplication of prime factor representations.
//
// This is slightly odd, in that `Mul` is defined on references, but returns the raw type.
impl std::ops::Mul for &Factors {
    type Output = Factors;

    // The product of two factorised numbers.
    //
    // The product can be trivially found by summing the powers of each factor.
    fn mul(self, other: Self) -> Self::Output {
        Factors(self.factors_iterator(other)
            .map(|(pr, (l, r))| Factor { prime: pr, power: l+r })
            .collect())
    }

}

#[derive(PartialEq)]
pub enum Perfection {
    Deficient,
    Perfect,
    Abundant,
}

#[derive(Clone, Copy, Debug)]
pub struct Factor {
    pub prime: u64,
    pub power: u32,
}

impl Factor {
    pub fn calculate(&self) -> u64 {
        self.prime.pow(self.power)
    }

    // The sum of all powers of a prime, from 0.
    //
    // This could be calcualted by looping and summing. However, this is the geometric sum `n^0 + n^1 + .. + n^p = (n^(p+1)-1)/(p-1)` which doesn't require looping.
    pub fn power_sum(&self) -> u64 {
        let po = self.power;
        let pr = self.prime;

        let num = pr.pow(po+1) - 1;
        let den = pr - 1;

        num / den
    }
}




pub struct Primes(Vec<u64>);

impl Primes {
    pub fn iter<'a>(&'a mut self) -> PrimesIterator<'a> {
        PrimesIterator { ps: &mut self.0, state: PIState::I(0) }
    }

    pub fn factorise<'a>(&'a mut self, n: u64) -> Factorisation {
        let mut m = n;
        let mut ps = self.iter();
    
        Factorisation {
            n,
            factors: Factors(
                std::iter::from_fn(move || {
                    loop {
                        let pr = ps.next().unwrap();
                        if pr > m { return None }
                        let mut po = 0;
                        loop {
                            if m % pr != 0 {
                                break;
                            }
                            po += 1;
                            m /= pr;
                        }
                        if po > 0 {
                            return Some(Factor{ prime: pr, power: po })
                        }
                    }
                }).collect()
            )
        }
    }
}

impl Default for Primes {
    fn default() -> Self {
        let mut ps = Vec::new();
        ps.push(2);
        Primes(ps)
    }
}

pub struct PrimesIterator<'a> {
    ps: &'a mut Vec<u64>,
    state: PIState,
}

enum PIState {
    I(usize),
    P(u64)
}

impl <'a> Iterator for PrimesIterator<'a> {
    type Item=u64;

    fn next(&mut self) -> Option<u64> {
        match self.state {
            PIState::I(ref mut i) if *i < self.ps.len() => {
                let p = self.ps[*i];
                *i+=1;
                Some(p)
            }
            PIState::I(_) => {
                let pi = self.ps[self.ps.len()-1] + 1;
                self.state = PIState::P(pi);
                self.next()
            }
            PIState::P(ref mut pi) => {
                let mut i = *pi;
                // we trade one square root op here vs needing to compute i*i repeatedly within the loop
                let i_root = (i as f64).sqrt().ceil() as u64;
                loop {
                    if self.ps.iter()
                        .take_while(|&&p| p <= i_root)
                        .any(|&p| i % p == 0) // rely upon this short-circuting
                    {
                        i += 1;
                        continue;
                    } else {
                        *pi = i+1;
                        self.ps.push(i);
                        break Some(i);
                    }
                }
            }
        }
    }
}



pub fn is_palendrome(n: u64) -> bool {
    let forwd = format!("{}", n);
    let bakwd = forwd.chars().rev().collect::<String>();
    forwd.chars().zip(bakwd.chars()).all(|(f, b)| f == b)
}
