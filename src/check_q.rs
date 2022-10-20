pub fn check_q(input: &mut String) -> bool {
    input.pop();
    if input.ends_with('\r') {
        input.pop();
    }
    input == "q" || input == "quit" || input == "exit"
}
