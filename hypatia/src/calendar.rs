#[derive(Clone, Copy)]
pub struct Year(u16);

#[derive(Clone, Copy)]
pub struct Month(u8);   // 0=January .. 11=December
pub struct Day(u8);     // 0=Sunday .. 6=Saturday

pub const MONTHS: u8 = 12;
pub const DAYS: u8 = 7;

impl Year {
    pub fn new(y: u16) -> Year { Year(y) }
}

impl Month {
    pub fn new(m: u8) -> Month { Month(m) }
}

pub fn is_leap_year(y: Year) -> bool {
    let div4 = y.0 % 4 == 0;
    let century = y.0 / 100;
    let cent_div4 = century % 4 == 0;

    div4 && !cent_div4
}

pub fn days_in_month(y: Year, m: Month) -> u16 {
    match m.0 {
        // Thirty days have September(8), April(3), June(5) and November(10)
        3 | 5 | 8 | 10 => 30,
        // Saving February alone,
        1 => if !is_leap_year(y) {
            // which has twenty-eight, rain or shine
            28
        } else {
            // And on leap years, twenty-nine
            29
        }
        // All the rest have thirty-one
        _ => 31,
    }
}

pub fn days_in_year(y: Year) -> u16 {
    if is_leap_year(y) { 366 } else { 365 }
}
