use crate::r_check;
use std::io;

pub fn two() {
    let mut input = String::new();
    loop {
        println!("enter first number");

        io::stdin().read_line(&mut input).unwrap();
        if r_check::r_check(&mut input) {
            return;
        }
        let first: i32 = input.parse().unwrap();
        input.clear();

        println!("enter second number");

        io::stdin().read_line(&mut input).unwrap();
        if r_check::r_check(&mut input) {
            return;
        }
        let second: i32 = input.parse().unwrap();

        println!("{} + {} = {}", first, second, first + second);
        input.clear();
    }
}
