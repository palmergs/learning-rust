//extern crate rand;

use Event::NewRelease;
//use rand::{thread_rng, Rng};

enum Event {
    NewRelease,
}

fn probability(_: &Event) -> i32 {
//    let mut random = rand::thread_rng();
//    random.gen_range(0, 101)
    80
}

fn descriptive_probability(event: Event) -> &'static str {
    match probability(&event) {
        100 => "certain",
        0 => "impossible",
        0 ... 25 => "very unlikely",
        25 ... 50 => "unlikely",
        50 ... 75 => "likely",
        75 ... 100 => "very likely",
        _ => unreachable!()
    }
}

fn main() {
    println!("{}", descriptive_probability(NewRelease));
}
