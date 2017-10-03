extern crate base64;
extern crate openssl;
extern crate rand;

// mod exercise_01;
mod exercise_02;
// mod exercise_03;
mod exercise_04;
mod exercise_05;
mod exercise_06;
mod exercise_07;
mod exercise_08;
mod exercise_09;
mod exercise_10;
mod exercise_11;
mod exercise_12;
mod exercise_13;

mod edit_distance;
mod ascii;
mod english;
mod repeating_key_xor;
mod hex;
mod read_file;
mod utils;
mod aes;
mod aes_oracles;
mod pkcs_7;

use std::env::args;

// use exercise_01::run_01;
use exercise_02::run_02;
// use exercise_03::run_03;
use exercise_04::run_04;
use exercise_05::run_05;
use exercise_06::run_06;
use exercise_07::run_07;
use exercise_08::run_08;
use exercise_09::run_09;
use exercise_10::run_10;
use exercise_11::run_11;
use exercise_12::run_12;
use exercise_13::run_13;

fn main() {
    if let Some(which_exercise) = args().nth(1) {
        match which_exercise.parse() {
            // Ok(1) => run_01(),
            Ok(2) => run_02(),
            // Ok(3) => run_03(),
            Ok(4) => run_04(),
            Ok(5) => run_05(),
            Ok(6) => run_06(),
            Ok(7) => run_07(),
            Ok(8) => run_08(),
            Ok(9) => run_09(),
            Ok(10) => run_10(),
            Ok(11) => run_11(),
            Ok(12) => run_12(),
            Ok(13) => run_13(),
            _ => println!("no such exercise"),
        };
    } else {
        println!("please specify an exercise to run");
    }
}
