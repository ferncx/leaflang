use std::{fs::File, io::Read};

use crate::{throw_custom_error, tokens::token::{Tokens, Types, Variable}};

use super::lexer::Lexer;

///
/// A LeafLang module written in Rust.
/// 
pub struct Module {
    contents: Vec<char>,
    index: i64,
    functions: Vec<Tokens>,
    scopes: Vec<(String, Vec<Variable>)>,
    tokens: Vec<Tokens>
}


impl Module {

    /// Initializes the module. As per usual, the file's contents are stripped of whitespace and converted to a character vec.
    pub fn init(mut module: File) -> Module {
        let mut contents = String::new();
        let _ = module.read_to_string(&mut contents);
        let mut contents = contents.chars().collect::<Vec<char>>();
        contents.retain(|&x| x != '\n' && x != '\r' && x != ' ');

        Module {
            contents,
            index: -1,
            functions: vec![ // add some certain stdlib functions
                Tokens::FunctionDef(String::from("println!"), vec![Variable {t: Types::String, n: String::new(), v: String::new() }], Types::Void)
            ],
            tokens: vec![],
            scopes: vec![]
        }
    }

    pub fn tokenize_module(&mut self) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];
        while self.advance(1) {
            match self.check_for_keywords() {
                Some(x) => {
                    match x.as_str() {
                        "fn" => { self.advance(2); tokens.push(self.tokenize_function_definition()); }
                        "let" => {/* variable defs here */}
                        "return" => {/*return stuff here*/}
                        wc => { 
                            tokens.push(self.tokenize_function_call(wc.to_string()));
                        }
                    }
                }
                None => { tokens.push(Tokens::from_char(self.current())) }
            }
        }
        println!("{:#?}", tokens);
        tokens
    }

    /// "Opens" a module by checking to see if it actually exists. Returns the file if so.
    pub fn open_module(path: &str) -> Option<File> {
        let Ok(module) = File::open(&path) 
        else {
            return None;
        };
        Some(module)
    }

    pub fn advance(&mut self, amount: i64) -> bool {
        if (self.index + amount) as usize >= self.contents.len() { return false }
        self.index += amount;
        true
    }

    pub fn current(&self) -> char 
    {
        self.contents[self.index as usize]
    }

    pub fn check_for_keywords(&self) -> Option<String> {
        let keywords = [
            "fn",
            "let",
            "return",
        ];
        let keywords: Vec<String> = keywords
            .into_iter()
            .map(|x| String::from(x))
            .chain(
                self.functions.clone().into_iter().map(|x| x.try_as_function_def().unwrap().0)
            )
            .collect::<Vec<String>>();
        for word in keywords {
            let testcase = String::from_iter(
                self.contents
                [
                    self.index as usize..
                    {
                        if self.index as usize + word.len() <= self.contents.len() {self.index as usize + word.len() } 
                        else {self.index as usize}
                    }
                ].into_iter().collect::<Vec<_>>());
            if testcase == word {
                return Some(word)
            }
        }
        None
    }

    /// Tokenizes a Rust function (definition only)
    pub fn tokenize_function_definition(&mut self) -> Tokens {
        // discern function name
        let mut function_name = String::new();
        while self.current() != '(' {
            function_name.push(self.current());
            self.advance(1);
        }
        self.advance(1);
        // discern args and arg types
        let mut args : Vec<Variable> = vec![];
        while self.current() != ')' {
            let mut typebuf = String::new();
            let mut namebuf = String::new();
            while self.current() != ',' && self.current() != ')' {
                while self.current() != ':' {
                    namebuf.push(self.current());
                    self.advance(1);
                }
                self.advance(1);
                while self.current() != ',' && self.current() != ')' {
                    typebuf.push(self.current());
                    self.advance(1);
                }
                let r#type = Types::parse_type(&typebuf.to_lowercase());
                let arg = Variable { t: r#type, n: namebuf.clone(), v: String::new()};
                args.push(arg);
            }
        }
        // push args to function scope
        if let None = self.scopes.clone().into_iter().find(|s| s.0 == function_name)
        { self.scopes.push((function_name.clone(), vec![])); }
        let mut scopes = self.scopes.clone();
        let mut fscope = scopes.clone().into_iter().find(|s| s.0 == function_name).unwrap(); // can be sure this exists
        fscope.1 = fscope.1.into_iter().chain(args.clone().into_iter()).collect::<Vec<Variable>>();
        let index = scopes.iter().position(|s| s.0 == function_name).unwrap();
        std::mem::swap(&mut scopes[index], &mut fscope);
        self.scopes = scopes;
        self.advance(1);

        // determine return type
        if self.current() != '-' { // you don't have to specify void in rust, so this is necessary
            let return_type = Types::Void;
            let function_def = Tokens::FunctionDef(function_name, args, return_type);
            self.functions.push(function_def.clone());
            self.advance(-1); // dont talk about it
            return function_def;
        }
        self.advance(3);
        let mut r#type = String::new();
        while self.current() != '>' {
            r#type.push(self.current());
            self.advance(1);
        }
        let return_type = Types::parse_type(&r#type);
        if let Types::Invalid = return_type {
            throw_custom_error("Invalid return type in module".to_string());
        }
        let function_def = Tokens::FunctionDef(function_name, args, return_type);
        self.functions.push(function_def.clone());
        self.advance(-1);
        function_def
    }


    pub fn tokenize_function_call(&mut self, function: String) -> Tokens {
        let Some(called_function) = self.functions.clone().into_iter().find(|f| f.clone().try_as_function_def().unwrap().0 == function)
        else { throw_custom_error(format!("Invalid function call in module: {}", function)); return Tokens::Invalid; }; // just to be safe
        let called_function = called_function.try_as_function_def().unwrap();
        // find the arguments
        self.advance(function.len() as i64);
        let mut arguments : Vec<Variable> = vec![];
        while self.current() != ')' {
            let mut arg = String::new();
            self.advance(1);
            while self.current() != ',' && self.current() != ')' {
                arg.push(self.current());
                self.advance(1);
            }
            /* 
            first check if its a literal of some kind (e.x. "hi", 1, true)
            then check if it's a variable in scope
            if its a literal, make a new variable for it (temp, obviously; not stored in scope)
            */
            let var_if_literal = Lexer::discern_type(arg.clone(), arg.clone());
            if let Variable {t: Types::Placeholder, n: _, v: _} = var_if_literal {
                // if we're here, that means this argument was not a literal
                // first thing we need to do is check the function scope for this variable
                // note this code doesn't currently check for the global scope 

                let Some(var) = self.scopes.clone().into_iter().find(|s| s.0 == called_function.0)
                else { throw_custom_error(format!("argument {} does not exist in scope", arg)); return Tokens::Invalid;};

                let Some(var) = var.1.into_iter().find(|v| v.n == arg)
                else { throw_custom_error(format!("argument {} does not exist in scope", arg)); return Tokens::Invalid;};

                arguments.push(var);
                continue;
            }
            // if we're here, that means a literal (i.e. "hello!") was specified.
            // we can just pass `var_if_literal` to the arguments vector
            
            arguments.push(var_if_literal);
        }
        if arguments.clone().into_iter().map(|mut v|{v.n = String::new(); v.v = String::new(); v} ).collect::<Vec<Variable>>() != called_function.1
        {  throw_custom_error(format!("Invalid arguments provided to function {}", called_function.0)); }

        return Tokens::FunctionCall(function, arguments);
    }
}
    



