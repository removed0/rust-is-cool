use std::io;

fn main() {
    let mut input = String::new();
    println!("enter smth");
    io::stdin().read_line(&mut input).unwrap();
    println!("++ {} ++", input);
}
