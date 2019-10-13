use hypatia::calendar::*;

fn main() {
    let mut mon0 = 0;
    let mut mon = 0;
    let mut d = 0u16;
    for y in 1900 ..= 2000  {
        let first = y == 1900;
        let y = Year::new(y);
        for m in 0..MONTHS {
            let m = Month::new(m);
            if d % 7 == 0 {
                mon += 1;
                if first { mon0 += 1; }
            }
            d += days_in_month(y, m);
        }
    }

    println!("First days of the month that are Sundays in 1900: {}", mon0);
    println!("First days of the month that are Sundays through 2000: {}", mon);
    println!("giving {} from 1901", mon - mon0);
}
