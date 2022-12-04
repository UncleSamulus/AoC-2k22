//! # Read a file containing a set of 'Rock Paper Scissors' play
//! 
//! ## Example
//! ```bash
//! cargo run -- ../input.txt # Execute the program with the input file
//! ```

use std::fs;
use std::env;

mod rps;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("{}", rps::compute_game_score(&contents));
}