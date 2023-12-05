mod one;

use one::One;

fn main() {
    let result = One();
    println!("{}", format!("Hello, world := {}!", result.expect("expect to get a good answer")));
}
