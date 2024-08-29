mod lexer;
mod tokens;
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
            Tokens::Invalid => throw_error(Exception { error: Exceptions::InvalidToken, character_pos: lexer.index }),
            Tokens::QuotationMark => {
                let string = lexer.discern_string().unwrap();
                print!("{}", string);
            },
            Tokens::FunctionDef(x, y, z) => {
                print!("fnc {}({}) [{:#?}]", x,
                { 
                    let mut string = String::new();
                    for i in y {
                        let st = format!("@{x}@ {y}", x = i.0.to_string(), y = i.1);
                        string.push_str(&format!("{:#?}, ", st));
                    }
                    string
                }
                    , z);
            },
            Tokens::None => { continue; },


            _ => { println!("{}", token.to_char()); }
        }
    }

}

fn throw_error(error: Exception) {
    eprintln!("Exception: {:#?}", error.error);
    std::process::exit(1);
}
