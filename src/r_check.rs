pub fn r_check(input: &mut String) -> bool {
    input.pop();
    if input.ends_with('\r') {
        input.pop();
    }
    input == "q"
}
