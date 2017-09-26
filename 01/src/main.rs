extern crate base64;

mod exercise_04;
mod exercise_05;
mod exercise_06;

mod edit_distance;
mod ascii;
mod single_byte_xor;
mod hex;
mod read_file;

use std::env::args;

// todo add the rest
use exercise_04::run_04;
use exercise_05::run_05;
use exercise_06::run_06;

fn main() {
    if let Some(which_exercise) = args().nth(1) {
        match which_exercise.parse() {
            Ok(4) => run_04(),
            Ok(5) => run_05(),
            Ok(6) => run_06(),
            _ => println!("no such exercise"),
        };
    } else {
        println!("please specify an exercise to run");
    }
}
