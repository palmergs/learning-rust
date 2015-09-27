extern crate rand;

use std::env;
use rand::{thread_rng, Rng};

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    println!("Program: {}", program);
    for x in &args {
        println!("{}", x);
    }

    let range = match args[1].parse::<u32>() {
        Ok(x) => { x }
        Err(f) => { panic!(f.to_string()) }
    };
    println!("Range = {}", range);

    let mut generator = rand::thread_rng();
    let val : u32 = generator.gen_range(0, range);
    println!("Random number = {}", val);
}
