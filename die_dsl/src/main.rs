fn tokenize(c : char) {
    match c {
        v @ '0' ... '9' => println!("digit = {} {}", c, v),
        'd' => println!("die roll"),
        _ => println!("unknown")
    }
}

fn main() {
    let program = "d20";
    for token in program.chars() {
        tokenize(token);
    }
}
