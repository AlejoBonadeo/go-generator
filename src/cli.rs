use std::io;

pub fn get_user_input(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
