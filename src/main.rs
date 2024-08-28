mod lexer;
mod tokens;
use std::{fs::File, io::{stdin, stdout, Read, Write}};

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
        lexer.skip_whitespace();
        print!("{}", lexer.get_next_token().to_char());
    }

}
