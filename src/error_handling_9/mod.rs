pub fn run() {
    let last_char = last_char_of_first_line("Hello, world\nHow are you today?").unwrap();
    println!("{last_char}");
}

/// ? is equivalent to
/// match greeting_file_result {
///     Ok(x) => Ok(x),
///     Err(error) => return Err(error)
/// };

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}