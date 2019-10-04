fn main() {
    let mut matrix = [[1u64; 21] ; 21];

    for i in 1 ..=20 {
        for j in 1..=20 {
            matrix[i][j] = matrix[i-1][j] + matrix[i][j-1];
        }
    }

    let last = matrix[20][20];

    println!("Matrix: {:?}", matrix);

    println!("Last entry in matrix is {}", last);
}
