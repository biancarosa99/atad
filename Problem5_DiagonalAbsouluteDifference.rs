use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Given a square matrix, calculate the absolute difference between the sums of its diagonals.
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    let n = arr.len();

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];  // Summing primary diagonal
        secondary_diagonal_sum += arr[i][n - 1 - i];  // Summing secondary diagonal
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()  // Returning absolute difference
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for _ in 0..n as usize {
        arr.push(stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect());
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
