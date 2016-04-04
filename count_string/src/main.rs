use std::env;
use std::fs;
use std::io;

fn main() {
    let input = env::args().nth(1).unwrap();
    let mut reader: Box<io::Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(input).unwrap())
    };
    io::copy(&mut reader, &mut io::stdout()).unwrap();
}

