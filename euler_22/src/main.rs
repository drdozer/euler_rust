
fn main() -> Result<(), std::io::Error> {
    let names_text = std::fs::read_to_string("data/p022_names.txt")?;
    let mut names: Vec<String> = {
        let mut spl: Vec<String> = names_text.split(',').map(str::to_string).collect();
        spl.iter_mut().for_each(|s| String::retain(s, |c| c != '"'));
        spl.sort();
        spl
    };

    let total: u64 = names.iter().enumerate().map(|(i, n): (usize, &String)| {
        let n_score: u64 = n.chars().map(char_value).sum();
        n_score * (i as u64 + 1)
    }).sum();
    println!("{:?}", total);

    Ok(())
}

fn char_value(c: char) -> u64 {
    ((c as u32) - ('A' as u32) + 1) as u64
}