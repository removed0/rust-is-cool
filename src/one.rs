use super::check_q::check_q;
use std::io;

pub fn one() {
    let mut input = String::new();
    loop {
        println!("type something");
        io::stdin().read_line(&mut input).unwrap();
        if check_q(&mut input) {
            return;
        }

        println!("++ - {} - ++", input);
        input.clear();
    }
}
