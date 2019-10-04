use hypatia::triangular;

fn main() {
    // multiples of 3 and 5 under 1000

    let filter_sum: u64 = (1..1000).filter(|i| ((i % 3) * (i % 5)) == 0).sum();

    println!("by filter_sum: {}", filter_sum);

    let by_triangles = {
        let n_3_5 = 1000 / (3 * 5);
        let n_5 = 1000 / 5;
        let n_3 = 1000 / 3;

        triangular(n_3) * 3 +
        triangular(n_5) * 5 -
        triangular(n_3_5) * 3 * 5
    };

    println!("by triangular numbers: {}", by_triangles);
}
