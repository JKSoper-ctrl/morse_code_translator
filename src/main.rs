use std::io;
use std::error::Error;
use std::process;
use morse_code_translator::string_to_morse;

fn main() {
    println!("Please input text to be translated into morse code:");
    
    let input: String = get_input().unwrap_or_else(|err| {
        eprintln!("Error reading input: {err}");
        process::exit(1);
    });

    let uppercase_input = input.to_uppercase();
    
    let output: String = string_to_morse(&uppercase_input).unwrap_or_else(|err| {
        eprintln!("Error translating text: {err}");
        process::exit(1);
    });

    println!("{output}");
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    Ok(input_string.trim_end().to_string())
}