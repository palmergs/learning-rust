fn bar() {
    let a = 5;
}

fn foo() {
    bar();
    let y = 5;
    let z = 100;
    bar();
}

fn main() {
    let x = 42;
    foo();
}
