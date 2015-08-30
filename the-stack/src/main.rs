fn bar() {
    let a = 5;
}

fn foo(i: &i32) {
    bar();
    let y = 5;
    let z = 100;
    bar();
}

fn main() {
    let x = 42;
    let y = Box::new(5);
    foo(&x);
}
