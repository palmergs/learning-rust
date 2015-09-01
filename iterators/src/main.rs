

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
    println!("Hello, world!");
}
