use super::check_q::check_q;
use std::io;

pub fn two() {
    let mut input = String::new();
    loop {
        println!("enter the first number");

        io::stdin().read_line(&mut input).unwrap();
        if check_q(&mut input) {
            return;
        }

        let first: i32 = input.parse().unwrap();
        input.clear();

        println!("enter the second number");

        io::stdin().read_line(&mut input).unwrap();
        if check_q(&mut input) {
            return;
        }

        let second: i32 = input.parse().unwrap();

        println!("{} + {} = {}", first, second, first + second);
        input.clear();
    }
}
