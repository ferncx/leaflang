pub mod lexer {
    use std::string;

    use crate::{throw_error, tokens::token::{Exception, Exceptions, Tokens, Types}};

    pub struct Lexer {
       pub contents: Vec<char>,
       pub index: i64,
    }

    impl Lexer {
        pub fn current(&self) -> char {
            self.contents[self.index as usize]
        }

        pub fn init(contents: String) -> Lexer {
            let mut contents: Vec<char> = contents.chars().collect::<Vec<char>>();
            contents.retain(|&x| x != '\n' && x != '\r' && x != ' ');
            Lexer {
                contents,
                index: -1,
            }
        }

        pub fn advance(&mut self, amount: i64) -> bool {
            if (self.index + amount) as usize >= self.contents.len() {
                return false;
            }
            self.index += amount;
            true
        }

        pub fn skip_whitespace(&mut self) {
            while self.current() == ' ' /* whitespace ASCII */ {
                self.advance(1);
            }
        }

        pub fn get_next_token(&mut self) -> Tokens {
            let possible_function = self.discern_function();
            if let Tokens::FunctionDef(_,_,_) = possible_function { return possible_function; }
            else if let Tokens::Invalid = possible_function { return Tokens::Invalid; }       
            Tokens::from_char(self.current())     
        }

        pub fn discern_string(&mut self) -> Option<String> {
            // this has to be ran AFTER the lexer identifies a quotation mark.
            let mut string = String::new();
            while self.current() != '"' {
                self.advance(1);
                string.push(self.current());
            }
            Some(string)
        }

        pub fn discern_type(&mut self) -> Types {
            let mut type_construction = String::new();
            self.advance(1); // we are currently on an @ when this calls, so skip it so that the while loop actually executes
            while self.current() != '@' {
                
                type_construction.push(self.current());
                self.advance(1);
            }
            let res = Types::parse_type(&type_construction);
            if res == Types::Invalid {
                throw_error(Exception {
                    error: Exceptions::InvalidType,
                    character_pos: self.index as i64
                });
                return Types::Invalid;
            }
            self.advance(1);
            res

        }
        pub fn discern_function(&mut self) -> Tokens {
            if self.current() == 'f'
            && self.contents[self.index as usize + 1] == 'n'
            && self.contents[self.index as usize + 2] == 'c'
            {
                let mut function_name = String::new();
                let mut args = Vec::new();
                self.advance(3);
                // discern function name
                while self.current() != '(' {
                    function_name.push(self.current());
                    self.advance(1);
                }
                self.advance(1);
                // discern args and arg types
                while self.current() != '[' {
                    let r#type = self.discern_type();
            
                    let mut arg: (Types, String) = (r#type, String::new());
                    let mut stringbuf = String::new();
                    while self.current() != ',' && self.current() != ')' {
                        stringbuf.push(self.current());
                        self.advance(1);
                    }
                    arg.1 = stringbuf;
                    args.push(arg);
                    self.advance(1);
                }
                // determine return type
                if self.current() != '[' {
                    return Tokens::Invalid;
                }
                self.advance(1);
                let return_type = self.discern_type();
                if self.current() != ']' {
                    return Tokens::Invalid;
                }
                println!("{:#?} {:#?} {:#?}", function_name, args, return_type);
                return Tokens::FunctionDef(function_name, args, return_type);
            }
            return Tokens::None;
        }
    }
}