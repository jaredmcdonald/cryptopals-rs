extern crate base64;
extern crate openssl;

mod exercise_04;
mod exercise_05;
mod exercise_06;
mod exercise_07;

mod edit_distance;
mod ascii;
mod english;
mod repeating_key_xor;
mod hex;
mod read_file;
mod utils;

use std::env::args;

// todo add the rest
use exercise_04::run_04;
use exercise_05::run_05;
use exercise_06::run_06;
use exercise_07::run_07;

fn main() {
    if let Some(which_exercise) = args().nth(1) {
        match which_exercise.parse() {
            Ok(4) => run_04(),
            Ok(5) => run_05(),
            Ok(6) => run_06(),
            Ok(7) => run_07(),
            _ => println!("no such exercise"),
        };
    } else {
        println!("please specify an exercise to run");
    }
}
