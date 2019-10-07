
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

use std::cell::RefCell;
// use std::ops::DerefMut;
// use std::ops::Deref;

pub struct Primes {
    ps: RefCell<Vec<u64>>,
}

impl Primes {
    fn iter<'a>(&'a self) -> impl Iterator<Item=u64> + 'a {
        let existing = self.ps.borrow().iter().map(|&p| p);
        let mut extending = {
            let mut ps = self.ps.borrow_mut();
            let i = ps.iter().last().unwrap() + 1;
            PrimeExtention { ps: &ps, i }
        };

        existing.chain(extending)
    }
}

impl Default for Primes {
    fn default() -> Self {
        let mut ps = Vec::new();
        ps.push(2);
        Primes {
            ps: RefCell::new(ps)
        }
    }
}


pub struct PrimeExtention<'a> {
    ps: &'a Vec<u64>,
    i: u64,
}

impl Iterator for PrimeExtention<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let i_root = (self.i as f64).sqrt().ceil() as u64;
        loop {
            if self.ps.iter()
                .take_while(|p| **p <= i_root)
                .any(|p| self.i % *p == 0)
            {
                self.i += 1;
                continue;
            } else {
                let p = self.i;
                self.i += 1;
                self.ps.push(p);
                return Some(p);
            }
        }
    }
}





pub fn is_palendrome(n: u64) -> bool {
    let forwd = format!("{}", n);
    let bakwd = forwd.chars().rev().collect::<String>();
    forwd.chars().zip(bakwd.chars()).all(|(f, b)| f == b)
}
