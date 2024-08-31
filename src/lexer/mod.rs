pub mod lexer {
    use std::{fs::File, io::Read, string};
    
    use crate::{throw_custom_error, throw_error, tokens::token::{Exception, Exceptions, Tokens, Types, Variable}};

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
            let possible_include_stmt = self.discern_include_call();
            let possible_function_call = self.discern_function_call();
            
            if let Tokens::FunctionDef(_, _, _) | Tokens::Invalid = possible_function_def { return possible_function_def; }  
            if possible_include_stmt.len() > 1 { 
                self.tokens = self.tokens.clone().into_iter().chain(possible_include_stmt.into_iter()).collect::<Vec<Tokens>>();
                return Tokens::None 
            }
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
                return Variable {t: Types::Void, v: String::new()}
            }
            else if item == "bool" {
                return Variable {t: Types::Bool, v: String::new()}
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

        ///
        /// Effectively parses a .rs file into tokens that the parser and lexer understand.
        /// 
        pub fn discern_include_call(&mut self) -> Vec<Tokens> {
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
                let Ok(mut module) = File::open(&path) 
                else {
                    throw_custom_error(format!("Attempted to include module {}, not found", path));
                    return vec![];
                };
                println!("{} exists!", path);
                // "import" module, recurse through all functions and add their names to defined functions
                let mut tokens: Vec<Tokens> = vec![];
                let mut contents = String::new();
                module.read_to_string(&mut contents);
                // this is VERY hacky
                let mut contents = contents.chars().collect::<Vec<char>>();
                contents.retain(|&x| x != '\n' && x != '\r' && x != ' ');
                let mut index = 0;
                println!("{}", contents.len());
                while index <= contents.len() - 1 {
                    //
                    // VERY similar to discern_function_definition()
                    // breaks down a .rs file's function into understandable tokens
                    //
                    if contents[index as usize] == 'f' && contents[index as usize + 1] == 'n' {
                        
                        // discern function name
                        index += 2;
                        let mut function_name = String::new();
                        while contents[index as usize] != '(' {
                            function_name.push(contents[index as usize]);
                            index += 1
                        }
                        index += 1;

                        println!("{}", function_name);
                        
                        // discern args and arg types
                        let mut args : Vec<Variable> = vec![];
                         while contents[index as usize] != ')' {
                            let mut typebuf = String::new();
                            let mut namebuf = String::new();
                            while contents[index as usize] != ',' && contents[index as usize] != ')' {
                                while contents[index as usize] != ':' {
                                    namebuf.push(contents[index as usize]);
                                    index += 1
                                }
                                index += 1;
                                while contents[index as usize] != ',' && contents[index as usize] != ')' {
                                    typebuf.push(contents[index as usize]);
                                    index += 1
                                }
                                let r#type = Types::parse_type(&typebuf.to_lowercase());
                                let arg = Variable { t: r#type, v: namebuf.clone()};
                                println!("{:#?}", arg);
                                args.push(arg);
                            }
                        }
                        index += 1;
                        // determine return type
                        let mut return_type = Types::Placeholder;
                        if contents[index as usize] != '-' {
                            return_type = Types::Void;
                            let function_def = Tokens::FunctionDef(function_name, args, return_type);
                            self.functions.push(function_def);
                            continue;
                        }
                        index += 3;
                        let mut r#type = String::new();
                        while contents[index as usize] != '>' {
                            r#type.push(contents[index as usize]);
                            index += 1
                        }
                        let return_type = Types::parse_type(&r#type);
                        if let Types::Invalid = return_type {
                            throw_custom_error("Invalid return type in module".to_string());
                        }
                        let function_def = Tokens::FunctionDef(function_name, args, return_type);
                        self.functions.push(function_def.clone());
                        tokens.push(function_def);

                    }
                    else{
                    tokens.push(Tokens::from_char(contents[index as usize]));  
                     }
                     /*
                     Figure out how to tokenize the entire file (maybe move this to its own file?)
                      */
                    index += 1;
                }
                return tokens;
            }
            return vec![];
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