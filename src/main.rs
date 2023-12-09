use std::fs;

mod one;
mod two;

// use one;
// use two;

fn main() {
    // let result = one::One(fs::read_to_string("src/assets/one.txt"));
    
    let result = two::Two(fs::read_to_string("src/assets/two.txt").expect("couldn't open file"));
    
    println!("{}", format!("Hello, world := {}!", result.expect("expect to get a good answer")));
}
