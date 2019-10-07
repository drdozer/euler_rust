use hypatia::numbers::is_palendrome;

fn main() {
    let largest = (100..999)
        .flat_map(|i| (100..i).map(move |j| (i, j, i*j)))
        .filter(|ijp| is_palendrome(ijp.2)).max_by_key(|ijp| ijp.2)
        .unwrap();
    println!("largest palendrome is {} x {} = {}", largest.0, largest.1, largest.2);
}
