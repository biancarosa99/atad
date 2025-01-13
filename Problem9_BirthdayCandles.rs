use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* birthdayCakeCandles takes as argument cadles array
   for candkes = [4,4,1,3], it should return 2, as 4 is the highes number (candle unit)
   and there are 2 of 4s
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max_height = candles.iter().max().unwrap();
    let count = candles.iter().filter(|&&height| height == *max_height).count();
    count as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}