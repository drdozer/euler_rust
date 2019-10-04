fn main() {
    let triangle_text = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    let triangle_numbers: Vec<Vec<u64>> = triangle_text.trim()
        .lines()
        .map(|l| l.split_whitespace().map(|d| d.parse::<u64>().unwrap()).collect())
        .collect();
    
    let mut sums: Vec<Vec<u64>> = Vec::new();
    sums.push({
        let mut row: Vec<u64> = Vec::new();
        row.push(triangle_numbers[0][0]);
        row
    });

    // dynamic programming - set [row][col] with max[row-1][col-1, col]
    for r in 2 .. triangle_numbers.len() {
        // let last_sum = &sums[r - 1];
        // let mut this_sums: Vec<u64> = Vec::new();
        // let this_tri = &triangle_numbers[r];

        // for i in 0 .. last_sum.len() {
        //     this_sums.push(
        //         this_tri[i] +
        //         last_sum[i].max(last_sum[i+1]));
        // }
        // this_sums.push(this_tri[])
        // sums.push(this_sums);
    }
}
