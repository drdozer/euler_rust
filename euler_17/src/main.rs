use hypatia::number_as_words;

// This solves the problem by brute force.
// A more elegant way is to factor it out e.g. 1..9, 10..19, 20..99, 100, and so on.
fn main() {
    let mut ch = 0;

    for i in 1..=1000 {
        let s = number_as_words(i);

        println!("{}\t{}", i, s);
        ch += s.chars().filter(|c| *c != ' ').count();
    }

    println!("Total characters: {}", ch);
}
