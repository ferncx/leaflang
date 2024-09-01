pub mod modules;
pub mod lexer {
    use std::{fs::File, io::Read, string};
    
    use crate::{lexer::modules::Module, throw_custom_error, throw_error, tokens::token::{Exception, Exceptions, Tokens, Types, Variable}};

    pub struct Lexer {
       pub contents: Vec<char>,
       pub index: i64,
       pub variables_in_space: Vec<Variable>,
       pub functions: Vec<Tokens>,
       pub tokens: Vec<Tokens>
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
                variables_in_space: vec![],
                functions: vec![],
                tokens: vec![]
            }
        }

        pub fn advance(&mut self, amount: i64) -> bool {
            if (self.index + amount) as usize >= self.contents.len() {
                return false;
            }
            self.index += amount;
            true
        }

        pub fn get_next_token(&mut self) -> Tokens {
            let possible_function_def = self.discern_function_definition();
            self.discern_include_call(); // This will almost always return a None token, as all needed tokens are added in modules.rs
            //let possible_function_call = self.discern_function_call();
            
            if let Tokens::FunctionDef(_, _, _) | Tokens::Invalid = possible_function_def { return possible_function_def; }  
            //if let Tokens::FunctionCall(_, _) | Tokens::Invalid = possible_function_call { return possible_function_call; }  
            
  
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

        pub fn discern_type(name: String, value: String) -> Variable {
            if value.starts_with("\"") && value.ends_with("\"") {
                return Variable { t: Types::String, v: value, n: name };
            }
            else if let Ok(_) = value.parse::<f32>() {
                return Variable { t: Types::Float, v: value, n: name };
            }
            else if let Ok(_) = value.parse::<i32>() {
                return Variable { t: Types::Int, v: value, n: name};
            }
            else if value == "void" {
                return Variable {t: Types::Void, v: value, n: name}
            }
            else if value == "true" || value == "false" {
                return Variable {t: Types::Bool, v: value, n: name}
            }
            else { 
                println!("{}", format!("Could not discern type of {}. Marking as placeholder.", name)); 
                return Variable {t: Types::Placeholder, v: value, n: name}
            }
        }

        // todo: rewrite discern_function_call

        pub fn discern_return(&mut self) -> Tokens {
            let mut to_return = String::new();
            while self.current() != ')' {
                to_return.push(self.current());
                self.advance(1);
            }
            if let Some(v) = self.variables_in_space.iter().find(|v| v.v == to_return) 
            { return Tokens::Return((Some(v.clone()), String::new())) }
            else { return Tokens::Return((None, to_return))};
            // ^ parse this
        }

        ///
        /// Effectively parses a .rs file into tokens that the parser and lexer understand.
        /// 
        pub fn discern_include_call(&mut self) -> Tokens {
            if self.current() == 'i'
            && self.contents[self.index as usize + 1] == 'n'
            && self.contents[self.index as usize + 2] == 'c'
            && self.contents[self.index as usize + 3] == 'l'
            && self.contents[self.index as usize + 4] == 'u'
            && self.contents[self.index as usize + 5] == 'd'
            && self.contents[self.index as usize + 6] == 'e'
            {
                self.advance(8);
                let mut path = String::new();
                while self.current() != '"' {
                    path.push(self.current());
                    self.advance(1);
                }
                println!("{}", path);
                // check if the file actually exists
                let Some(module) = Module::open_module(&path)
                else { throw_custom_error(format!("Attempted to include module {}, not found", path)); return Tokens::Invalid; }; 

                // "import" module by tokenizing it
                let mut module = Module::init(module);
                let tokens = module.tokenize_module();
                self.tokens = self.tokens.clone().into_iter().chain(tokens.into_iter()).collect::<Vec<Tokens>>();
            }
            return Tokens::None;
        }
        
        pub fn discern_function_definition(&mut self) -> Tokens {
            if Tokens::from_char(self.current()) != Tokens::Invalid { return Tokens::None }
            if self.current() == 'f'
            && self.contents[self.index as usize + 1] == 'n'
            && self.contents[self.index as usize + 2] == 'c'
            {
                let mut function_name = String::new();
                let mut args = Vec::new();
                self.advance(3);
                // discern function name
                while self.current() != '(' {
                    println!("{}", self.current());
                    function_name.push(self.current());
                    self.advance(1);
                }
                self.advance(1);
                
                // discern args and arg types
                while self.current() != '[' {
                    let mut stringbuf = String::new();
                    while self.current() != ',' && self.current() != ')' {
                        stringbuf.push(self.current());
                        self.advance(1);
                    }
                    if !stringbuf.is_empty() {
                        let arg = Self::discern_type(stringbuf, String::new());
                        args.push(arg);
                    }
                    self.advance(1);
                }
                // determine return type
                if self.current() != '[' {
                    return Tokens::Invalid;
                }
                self.advance(1);
                let mut stringbuf = String::new();
                while self.current() != ']' {
                    stringbuf.push(self.current());
                    self.advance(1);
                }
                let return_type = Self::discern_type(stringbuf, String::new());
                if self.current() != ']' {
                    return Tokens::Invalid;
                }
                let function_def = Tokens::FunctionDef(function_name, args, return_type.t);
                self.functions.push(function_def.clone());
                return function_def;
            }
            return Tokens::None;
        }
    }
}