use std::io;
use std::num::ParseIntError;

mod check_q;
mod one;
mod two;

fn main() -> Result<(), ParseIntError> {
    let mut input = String::new();
    loop {
        println!("type quit to exit");
        println!("type one or two");
        io::stdin().read_line(&mut input).unwrap();
        if check_q::check_q(&mut input) {
            return Ok(());
        }

        if input == "one" || input == "1" {
            one::one();
        } else if input == "two" || input == "2" {
            two::two()?;
        }
        input.clear();
    }
}
