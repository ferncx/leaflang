mod lexer;
mod tokens;
mod parser;
use std::{fs::File, io::{stdin, stdout, Read, Write}};

use tokens::token::{Exception, Exceptions, Tokens};

fn main() {
    /* let mut file =String::new();
    print!("Enter the relative path of the .ll file to run: ");
    let _ = stdout().flush();
    stdin().read_line(&mut file).expect("Did not enter a correct string");
    file = file.trim().to_string();
    */
    let file = "C:/Users/Jayleaf/Desktop/Code/leaflangrs/examples/hello_world.ll";
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut lexer: lexer::lexer::Lexer = lexer::lexer::Lexer::init(contents); // lexer lexer lexer!
    while lexer.advance(1) {
        let token = lexer.get_next_token();
        match token {
            Tokens::Invalid => throw_error(Exception { error: Exceptions::InvalidToken, message: String::new(), character_pos: lexer.index }),
            _ => { continue; },
        }
    }
    let mut file_str = String::new();
    let mut parser = parser::parser::Parser::init(lexer.tokens);
    while parser.advance(1) {
        let parsed_token = parser.parse_current_token();
        file_str.push_str(&(parsed_token + "\n"));
    }
    println!("{}", file_str)
}

fn throw_error(error: Exception) {
    eprintln!("Exception: {:#?}", error.error);
    std::process::exit(1);
}

fn throw_custom_error(message: String) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}
