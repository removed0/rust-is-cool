use std::io;

fn main() {
    let mut input = String::new();
    println!("enter smth");
    io::stdin().read_line(&mut input).unwrap();

    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    println!("++ - {} - ++", input);
}
