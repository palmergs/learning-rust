use std::env;
use std::fs;
use std::io;

fn main() {
    let input = env::args().nth(1).unwrap();

    //let mut reader: Box<io::Read> = if input == "-" {
    //    Box::new(io::stdin())
    //} else {
    //    Box::new(fs::File::open(input).unwrap())
    //};

    let mut reader: Box<io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _ => Box::new(fs::File::open(input).unwrap())
    };

    //io::copy(&mut reader, &mut io::stdout()).unwrap();

    let mut buffer = String::new();
    let count = match reader.read_to_string(&mut buffer) {
        Ok(n) => n,
        Err(_) => 0
    };

    println!("({}) {}", count, buffer); 
}

