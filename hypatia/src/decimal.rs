use std::ops::{Add, Mul};


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
