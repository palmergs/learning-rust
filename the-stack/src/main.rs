fn bar() {
    let a = 5;
    println!("In bar a={}", a); 
}

fn foo(i: &i32, j: &i32) {
    bar();
    let mut y : i32 = *i;
    let z = j;
    println!("In foo y={} and z={}", y, z);
    y = y * 2;
    println!("In foo y={}", y);
    bar();
}

fn main() {
    let x = 42;
    let y = Box::new(5);
    foo(&x, &y);
}
