use std::io;

mod one;
mod r_check;
mod two;

fn main() {
    let mut input = String::new();
    loop {
        println!("enter one or two");
        io::stdin().read_line(&mut input).unwrap();

        if r_check::r_check(&mut input) {
            return;
        }
        if input == "one" || input == "1" {
            one::one();
        } else if input == "two" || input == "2" {
            two::two();
        }
        input.clear();
    }
}
