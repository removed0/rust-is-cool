use crate::r_check;
use std::io;

pub fn one() {
    let mut input = String::new();
    loop {
        println!("enter smth");
        io::stdin().read_line(&mut input).unwrap();

        if r_check::r_check(&mut input) {
            return;
        }
        println!("++ - {} - ++", input);
        input.clear();
    }
}
