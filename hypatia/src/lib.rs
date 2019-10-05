use std::ops::{Add, Mul};

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

pub fn prime_factors(mut n: u64) -> impl Iterator<Item = Factor> {
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

pub fn is_palendrome(n: u64) -> bool {
    let forwd = format!("{}", n);
    let bakwd = forwd.chars().rev().collect::<String>();
    forwd.chars().zip(bakwd.chars()).all(|(f, b)| f == b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Decimal {
    // store the digits in reverse order, so that digits[0] is the units, digits[1] the 10s and so on.
    digits: Vec<u8>,
}


impl Decimal {
    pub fn from_string(s: &str) -> Decimal {
        Decimal {
            digits: s.chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect()
        }
    }

    pub fn from_u32(n: u32) -> Decimal {
        Decimal::from_string(&n.to_string())
    }

    pub fn zero() -> Decimal {
        Decimal {
            digits: Vec::new(),
        }
    }

    pub fn digits(&self) -> &[u8] {
        &self.digits[..]
    } 
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digits.iter().rev().map(|d| d.to_string()).collect::<String>())
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for &Decimal {
    type Output = Decimal;

    fn mul(self, rhs: &Decimal) -> Decimal {
        // product length is the product of the individual lengths
        let l = self.digits.len() + rhs.digits.len() - 1;

        // we should make a rectangular table, but it simplifies things later if it is square and padded with zeros
        let mut table = vec![vec![0u8; rhs.digits.len()]; self.digits.len()];

        // for i in 0 .. self.digits.len() {
        //     for j in 0 .. rhs.digits.len() {
        //         table[i][j] = self.digits[i]*rhs.digits[j];
        //     }
        // }
        for (i, row) in table.iter_mut().enumerate() {
            let d_i = self.digits[i];
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = d_i*rhs.digits[j];
            }
        }


        // println!("table: {:?}", table);

        let mut diags: Vec<u16> = Vec::with_capacity(l+1);
        for i in 0 .. l {
            // println!("i: {}", i);
            let mut d_i = 0u16;
            for j in 0 ..= i {
                let k = i - j;
                if k < self.digits.len() && j < rhs.digits.len() {
                    // println!("\t{} x {}", k, j);
                    d_i += u16::from(table[i-j][j]);
                }
            }
            diags.push(d_i);
        }

        let mut carry = 0u16;
        let mut digits: Vec<u8> = Vec::new();
        for d in diags {
            let s = d + carry;
            digits.push((s % 10) as u8);
            carry = s / 10;
        }
        if carry != 0 { digits.push(carry as u8); }

        Decimal { digits }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for &Decimal {
    type Output = Decimal;

    fn add(self, rhs: &Decimal) -> Decimal {
        // max len of lhs, rhs
        let l = self.digits.len().max(rhs.digits.len());

        // reserve +1 digits for sum overflow
        let mut sum: Vec<u8> = Vec::with_capacity(l+1);
        
        let mut carry = 0;
        for i in 0..l {
            let self_i = if i < self.digits.len() { self.digits[i] } else { 0 };
            let rhs_i = if i < rhs.digits.len()  { rhs.digits[i] } else { 0 };

            let s = self_i + rhs_i + carry;
            let d = s % 10;
            carry = s / 10;

            sum.push(d);
        }

        if carry != 0 {
            sum.push(carry);
        }

        Decimal { digits: sum }
    }
}

fn digit_names() -> [&'static str; 9] {
    [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
        ]
}

fn teens_names() -> [&'static str; 10] {
    [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen"
    ]
}

fn tens_names() -> [&'static str; 8] {
    [
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety"
    ]
}

pub fn number_as_words(n: usize) -> String {
    let units = n % 10;
    let tens = (n / 10) % 10;
    let teens = n % 100;
    let hundreds = (n / 100) % 10;
    let thousands = n / 1000;

    let mut words = String::new();

    if thousands > 0 {
        words.push_str(&number_as_words(thousands));
        words.push_str(" thousand");
    }

    let mut needs_and = if hundreds > 0 {
        if !words.is_empty() { words.push(' '); }
        words.push_str(digit_names()[hundreds - 1]);
        words.push_str(" hundred");
        true
    } else {
        false
    };

    if teens < 20 && teens > 9 {
        if needs_and { words.push_str(" and"); }
        if !words.is_empty() { words.push(' '); }
        words.push_str(teens_names()[teens - 10]);
    } else {
        if tens != 0 {
            if needs_and { words.push_str(" and"); needs_and = false; }
            if !words.is_empty() { words.push(' '); }
            words.push_str(tens_names()[tens - 2]);
        }
        if units != 0 {
            if needs_and { words.push_str(" and"); }
            if !words.is_empty() { words.push(' '); }
            words.push_str(digit_names()[units - 1]);
        }
    }

    words
}