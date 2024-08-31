pub mod lexer {
    use std::{fs::File, string};
    
    use crate::{throw_custom_error, throw_error, tokens::token::{Exception, Exceptions, Tokens, Types, Variable}};

    pub struct Lexer {
       pub contents: Vec<char>,
       pub index: i64,
       pub variables_in_space: Vec<Variable>,
       pub functions: Vec<Tokens>
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
                functions: vec![]
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
            let possible_function_call = self.discern_function_call();
            
            if let Tokens::FunctionDef(_, _, _) | Tokens::Invalid = possible_function_def { return possible_function_def; }  
            if let Tokens::FunctionCall(_, _) | Tokens::Invalid = possible_function_call { return possible_function_call; }  
            
  
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

        pub fn discern_type(&mut self, item: String) -> Variable {
            println!("{}", self.current());
            if item.starts_with("\"") && item.ends_with("\"") {
                return Variable { t: Types::String, v: item };
            }
            else if let Ok(_) = item.parse::<f32>() {
                return Variable { t: Types::Float, v: item };
            }
            else if let Ok(_) = item.parse::<i32>() {
                return Variable { t: Types::Int, v: item};
            }
            else if item == "void" {
                return Variable {t: Types::Void, v: item}
            }
            else if item == "bool" {
                return Variable {t: Types::Bool, v: item}
            }
            else { 
                println!("{}", format!("Could not discern type of {}. Marking as placeholder.", item)); 
                return Variable {t: Types::Placeholder, v: item}
            }
        }

        /*
        TODO:
        You have to create namespaces for variables
        This involves making a function
         */
        pub fn discern_function_call(&mut self) -> Tokens {
            if Tokens::from_char(self.current()) != Tokens::Invalid { return Tokens::None }
            if self.contents[self.index as usize - 1] == 'c'
            && self.contents[self.index as usize - 2] == 'n'
            && self.contents[self.index as usize - 3] == 'f'
            {
                return Tokens::None;
            }

            let mut function_name = String::new();
            while self.current() != '(' {
                function_name.push(self.current());
                self.advance(1);
            }
            // ensure this function exists
            if function_name == "ret" {
                self.discern_return();
                return Tokens::None;
            }
            let Some(x) = self.functions
                .iter()
                .find(|function| matches!(function, Tokens::FunctionDef(x, _, _) if *x == function_name))
            else { throw_custom_error(format!("Invalid function: {}", function_name)); return Tokens::Invalid; };
            let x = x.clone().try_as_function_def().unwrap();
            // find the arguments
            let mut arguments : Vec<Variable> = vec![];
            while self.current() != ')' {
                let mut arg = String::new();
                self.advance(1);
                while self.current() != ',' && self.current() != ')' {
                    arg.push(self.current());
                    self.advance(1);
                }
                arguments.push(self.discern_type(arg));
            }
            // strum is the greatest thing on the planet
            // (check arg count)
            if arguments.len() != x.1.len() {
                throw_custom_error(format!("Invalid argument count: {} wants {} args, {} were provided", function_name, x.1.len(), arguments.len())); return Tokens::Invalid;
            }
            return Tokens::FunctionCall(function_name, arguments)
        }

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

        /*pub fn discern_include_call(&mut self) -> Tokens {
            if self.current() == 'i'
            && self.contents[self.index as usize + 1] == 'n'
            && self.contents[self.index as usize + 2] == 'c'
            && self.contents[self.index as usize + 3] == 'l'
            && self.contents[self.index as usize + 4] == 'u'
            && self.contents[self.index as usize + 5] == 'd'
            && self.contents[self.index as usize + 6] == 'e'
            {
                self.advance(7);
                println!("{}", self.current());
                let mut path = String::new();
                while self.current() != '"' {
                    path.push(self.current());
                    self.advance(1);
                }
                println!("{}", path);
                // check if the file actually exists
                if let Err(imported_file) = File::open(&path) {
                    return Tokens::Invalid;
                }
                println!("{} exists!", path);
                // dynamically import module, recurse through all functions and add their names to defined functions
                return Tokens::FunctionCall(path);
            }
            return Tokens::Invalid;
        }
        */
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
                        let arg = self.discern_type(stringbuf);
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
                let return_type = self.discern_type(stringbuf);
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