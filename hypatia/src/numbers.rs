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
pub struct Factors {
    n: u64,
    vec: Vec<Factor>
}

impl Factors {
    pub fn vec_ref(&self) -> &Vec<Factor> {
        &self.vec
    }

    pub fn vec(self) -> Vec<Factor> {
        self.vec
    }

    // The count of divisors can be calulated efficiently from the prime powers.
    // For each prime in the prime factors with a power of p, it can contribute p+1 modulo families of divisors.
    // Therefore, the total number of divisors a number has is the product of p+1 for each prime factor power.
    pub fn count_divisors(&self) -> u32 {
        self.vec.iter().map(|f| f.power + 1).product::<u32>()
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
        self.vec.iter().map(Factor::power_sum).product()
    }

    // The proper divisors exclude thenumber itself.
    pub fn sum_proper_divisors(&self) -> u64 {
        self.sum_divisors() - self.n
    }

    // Check if the number is perfect or not.
    pub fn is_perfect_number(&self) -> Perfection {
        match self.sum_proper_divisors().cmp(&self.n) {
            Ordering::Less => Perfection::Deficient,
            Ordering::Equal => Perfection::Perfect,
            Ordering::Greater => Perfection::Abundant,
        }
    }
}

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

    pub fn factorise<'a>(&'a mut self, n: u64) -> Factors {
        let mut m = n;
        let mut ps = self.iter();
    
        Factors {
            n,
            vec: std::iter::from_fn(move || {
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
                let i_root = (i as f64).sqrt().ceil() as u64;
                loop {
                    if self.ps.iter()
                        .take_while(|&&p| p <= i_root)
                        .any(|&p| i % p == 0)
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
