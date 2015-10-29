
fn match_char(slice: &str) {
    for c in slice.chars() {
        match c {
            'a' ... 'z' => println!("letter"),
            '0' ... '9' => println!("digit"),
            'A' ... 'Z' => println!("capital"),
            ' ' | '\n' | '\t' => println!("whitespace"),
            _ => println!("other")
        }
//      println!("{}", c);
    }
}

fn hachi(dog: &str) {
    let tmp = &dog[0..5];
    println!("Dog is {}", tmp);
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}


fn itchars(slice: &str) {
    for c in slice.chars() {
        println!("char = {}", c);
    }
}

fn main() {
    println!("Hello, world!");

    let mut s = "Hello".to_string();
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    let s2 = "Hello".to_string();
    takes_slice(&s2);

    let s3 = "hachiko";
    hachi(s3);

    let s4 = "Test";
    itchars(s4);

    let s5 = "Test".to_string();
    itchars(&s5);

    match_char("4 score and 7 years ago!");
}
