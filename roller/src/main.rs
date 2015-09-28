extern crate rand;

use std::env;
// use std::process;
use rand::{thread_rng, Rng};

fn d(sides : i32) -> i32 {
    let mut random = rand::thread_rng();
    (random.gen_range(0, sides) + 1)
}

fn sum_d(dice : i32, sides : i32) -> i32 {
    let mut sum : i32 = 0;
    for x in 1..dice {
        sum = sum + d(sides);
    }
    sum
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    println!("Program: {}", program);
    for x in &args {
        println!("{}", x);
    }

    let range = match args[1].parse::<i32>() {
        Ok(x) => { x }
        Err(f) => { panic!(f.to_string()) }
    };
    println!("Range = {}", range);

    for x in 0..10 {
        println!("{}.\t3d{} = {}", x+1, range, sum_d(3, range));
    }

    // std::process::exit(val);
}
