
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

pub fn primes() -> impl Iterator<Item = u64> {
    let mut ps = Vec::new();
    let mut i = 2u64;

    std::iter::from_fn(move || {
        // really important for performance to only search up to (inclusive)
        // the square root of the number to be tested
        let i_root = (i as f64).sqrt().ceil() as u64;
        loop {
            if ps.iter()
                .take_while(|p| **p <= i_root)
                .any(|p| i % *p == 0)
            {
                i += 1;
                continue;
            } else {
                let p = i;
                i += 1;
                ps.push(p);
                return Some(p);
            }
        }
    })
}

#[derive(Debug)]
pub struct Factor {
    pub prime: u64,
    pub power: u32,
}

pub fn factors(n: u64) -> impl Iterator<Item = u64> {
    (1..n).filter(move |i| n % *i == 0)
}

impl Factor {
    pub fn calculate(&self) -> u64 {
        self.prime.pow(self.power)
    }
}

pub fn prime_factors(n: u64) -> impl Iterator<Item = Factor> {
    let mut n = n;
    let mut ps = primes();
    std::iter::from_fn(move || {
        loop {
            let pr = ps.next().unwrap();
            if pr > n { return None }
            let mut po = 0;
            loop {
                if n % pr != 0 {
                    break;
                }
                po += 1;
                n /= pr;
            }
            if po > 0 {
                return Some(Factor{ prime: pr, power: po })
            }
        }
    })
}



pub struct Primes(Vec<u64>);

impl Primes {
    fn iter<'a>(&'a mut self) -> PrimesIterator<'a> {
        PrimesIterator::IterateOverExisting(&mut self.0, 0)
    }
}

impl Default for Primes {
    fn default() -> Self {
        let mut ps = Vec::new();
        ps.push(2);
        Primes(ps)
    }
}

pub enum PrimesIterator<'a> {
    IterateOverExisting(&'a mut Vec<u64>, usize),
    IterateOverExtending(&'a mut Vec<u64>, u64),
}

impl <'a> Iterator for PrimesIterator<'a> {
    type Item=u64;

    fn next(&mut self) -> Option<u64> {
        match *self {
            PrimesIterator::IterateOverExisting(ref mut ps, ref mut i) => if *i < ps.len() {
                let p = ps[*i];
                *i+=1;
                Some(p)
            } else {
                let pi = ps[*i-1] + 1;
                *self = PrimesIterator::IterateOverExtending(ps, pi);
                self.next()
            }
            PrimesIterator::IterateOverExtending(ref mut ps, ref mut pi) => {
                let mut i = *pi;
                let i_root = (i as f64).sqrt().ceil() as u64;
                loop {
                    if ps.iter()
                        .take_while(|&&p| p <= i_root)
                        .any(|&p| i % p == 0)
                    {
                        i += 1;
                        continue;
                    } else {
                        *pi = i+1;
                        ps.push(i);
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
