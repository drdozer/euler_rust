
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