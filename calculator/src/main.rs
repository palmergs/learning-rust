fn main() {
    // a simple integer calculator:
    // '+' or '-' add or substract by 1
    // '*' or '/' means multiply or divide by 2

    let program = "+ + * - /";
    let mut accumulator = 0;
    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignore everything else */ }
        }
    }

    println!("The program \"{}\" calculates the value {}", program, accumulator);
}