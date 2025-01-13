use std::io;

fn solve_me_first(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // variable declaration
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut num_str_2).ok().expect("read error");

    // parse integers
    let num_1: i32 = num_str_1.trim().parse().ok().expect("parse error");
    let num_2: i32 = num_str_2.trim().parse().ok().expect("parse error");

    // compute and print the sum
    println!("{}", solve_me_first(num_1, num_2));
}
