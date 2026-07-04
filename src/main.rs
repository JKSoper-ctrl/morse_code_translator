use std::io;
use std::error::Error;
use std::process;

fn main() {
    println!("Please input text to be translated into morse code:");
    
    let input: String = get_input().unwrap_or_else(|err| {
        eprintln!("Error reading input: {err}");
        process::exit(1);
    });

    println!("{input}");
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    Ok(input_string.trim_end().to_string())
}