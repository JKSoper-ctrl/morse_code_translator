use std::fmt;

#[derive(Debug)]
pub struct UnsupportedCharError {pub unsupported_character: char}

impl fmt::Display for UnsupportedCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported character: '{}'", self.unsupported_character)
    }
}

impl std::error::Error for UnsupportedCharError {}

pub fn string_to_morse(input: &str) -> Result<String, UnsupportedCharError> {
    let mut morse = String::new();
    
    for character in input.chars() {
        let morse_char = char_to_morse(character)?;
        morse.push_str(morse_char);
        morse.push(' ');
    };

    Ok(morse.trim_end().to_string())
}

pub fn char_to_morse(character: char) -> Result<&'static str, UnsupportedCharError> {
    match character {
        'A' => Ok(".-"),
        'B' => Ok("-..."),
        'C' => Ok("-.-."),
        'D' => Ok("-.."),
        'E' => Ok("."),
        'F' => Ok("..-."),
        'G' => Ok("--."),
        'H' => Ok("...."),
        'I' => Ok(".."),
        'J' => Ok(".---"),
        'K' => Ok("-.-"),
        'L' => Ok(".-.."),
        'M' => Ok("--"),
        'N' => Ok("-."),
        'O' => Ok("---"),
        'P' => Ok(".--."),
        'Q' => Ok("--.-"),
        'R' => Ok(".-."),
        'S' => Ok("..."),
        'T' => Ok("-"),
        'U' => Ok("..-"),
        'V' => Ok("...-"),
        'W' => Ok(".--"),
        'X' => Ok("-..-"),
        'Y' => Ok("-.--"),
        'Z' => Ok("--.."),
        '0' => Ok("-----"),
        '1' => Ok(".----"),
        '2' => Ok("..---"),
        '3' => Ok("...--"),
        '4' => Ok("....-"),
        '5' => Ok("....."),
        '6' => Ok("-...."),
        '7' => Ok("--..."),
        '8' => Ok("---.."),
        '9' => Ok("----."),
        ' ' => Ok("/"),
        other_character => Err(UnsupportedCharError{unsupported_character: other_character}),
    }
}