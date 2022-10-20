use super::check_q::check_q;
use std::io;
use std::num::ParseIntError;

pub fn two() -> Result<(), ParseIntError> {
    let mut input = String::new();
    loop {
        println!("enter the first number");

        io::stdin().read_line(&mut input).unwrap();
        if check_q(&mut input) {
            return Ok(());
        }

        let first: i32 = input.parse()?;
        input.clear();

        println!("enter the second number");

        io::stdin().read_line(&mut input).unwrap();
        if check_q(&mut input) {
            return Ok(());
        }

        let second: i32 = input.parse()?;

        println!("{} + {} = {}", first, second, first + second);
        input.clear();
    }
}
