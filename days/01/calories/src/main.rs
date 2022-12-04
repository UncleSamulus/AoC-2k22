//! Read a file containing sets of number
//! Compute the max of the sum of each set
//! Print the result

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut max = 0;
    let mut current = 0;
    for line in contents.lines() {
        // Check if line is empty
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }
    println!("Max: {}", max);
}