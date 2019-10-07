
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

use std::rc::Rc;
use std::sync::RwLock;
use std::iter::{IntoIterator};

pub struct Primes {
    ps: Rc<RwLock<Vec<u64>>>,
}

impl Default for Primes {
    fn default() -> Self {
        let mut ps = Vec::new();
        ps.push(2);
        Primes {
            ps: Rc::new(RwLock::new(ps))
        }
    }
}

/*
impl <'a> IntoIterator for &'a mut Primes {
    type Item = u64;

    type IntoIter = Box<dyn Iterator<Item = u64> + 'a>;
    
    fn into_iter(self) -> Self::IntoIter {

        let exist_iter = {
            let ps = self.ps.read().unwrap();
            ps.iter().map(|&p| p)
        };

        let extend_iter = {
            let mut ps = self.ps.write().unwrap();
            let last = ps.last().unwrap();
            let mut i = last + 1;
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
        };

        let it: Box<dyn Iterator<Item = u64>> = Box::new(
            exist_iter.chain(extend_iter));
        it
    }
}
*/

impl <'a> IntoIterator for &'a mut Primes {
    type Item = u64;
    type IntoIter = PrimesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let existing_ps = self.ps.clone();
        PrimesIter{
            ps: existing_ps,
            state: PrimesIterState::FromExisting(&existing_ps.read().unwrap().iter()),
        }
    }
}

pub struct PrimesIter<'a> {
    ps: Rc<RwLock<Vec<u64>>>,
    state: PrimesIterState<'a>
}

enum PrimesIterState<'a> {
    FromExisting(&'a std::slice::Iter<'a, u64>),
    FromExtention()
}

impl <'a> Iterator for PrimesIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.state {
            PrimesIterState::FromExisting(i) => match i.next() {
                Some(&n) => Some(n),
                None => {
                    self.state = PrimesIterState::FromExtention();
                    self.next() // chain
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
