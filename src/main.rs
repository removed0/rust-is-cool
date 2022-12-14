use std::io;

mod check_q;
mod one;
mod two;

fn main() {
    println!("type quit to exit");
    let mut input = String::new();
    loop {
        println!("type one or two");
        io::stdin().read_line(&mut input).unwrap();
        if check_q::check_q(&mut input) {
            return;
        }

        if input == "one" || input == "1" {
            one::one();
        } else if input == "two" || input == "2" {
            let _ = two::two();
        }
        input.clear();
    }
}
