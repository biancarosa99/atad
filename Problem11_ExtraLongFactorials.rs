extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::One;
use std::io::{self, BufRead};

/* Calculacte n!=1*2*3*...*(n-1)*n
note: factorials of n>20 can't be stored in a 64-bit long long variable, we need big integers
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = BigInt::one();
    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}