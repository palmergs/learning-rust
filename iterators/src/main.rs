
fn finder() {
    println!("Check for existing value");
    let greater_than_eight = (1..10).find(|x| *x > 8);
    match greater_than_eight {
        Some(_) => println!("Found some"),
        None => println!("None found."),
    }
}

fn collector() {
    println!("Build range into vector");
    let one_to_ten = (1..10).collect::<Vec<_>>();
    for num in &one_to_ten {
        println!("{}", num);  
    }
}

fn itvector() {
    println!("Iterate over vector");
    let nums = vec![1, 2, 3];
    for num in &nums {
        println!("{}", num);  // num is pointer to i32
        println!("{}", *num); // dereference num
    }
}

fn main() {
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }
    itvector();
    collector();
    finder();
    println!("Hello, world!");
}
