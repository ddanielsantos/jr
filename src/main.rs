mod token;

use jr::{is_string, Token};
use std::error::Error;
use std::fs;

fn scan(source: &str) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();

    for (i, c) in source.char_indices() {
        match c {
            ' ' => res.push(Token::WhiteSpace),
            '{' => res.push(Token::LeftCurly),
            '}' => res.push(Token::RightCurly),
            '[' => res.push(Token::LeftBracket),
            ']' => res.push(Token::RightBracket),
            '"' => res.push(Token::Quote),
            ',' => res.push(Token::Comma),
            ':' => res.push(Token::Colon),
            ';' => res.push(Token::Semicolon),
            '.' => res.push(Token::Dot),
            '-' => res.push(Token::Minus),
            '0'..='9' => res.push(Token::Number),
            c if is_string(c) => res.push(Token::String),
            _ => panic!("Invalid char: '{}' at position {}", c, i),
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("source.json")?;

    println!("{:?}", scan(&f));

    Ok(())
}
