pub mod parser {
    use crate::tokens::token::Tokens;

    pub struct Parser {
        tokens: Vec<Tokens>,
        index: i64,
        output: String
    }

    impl Parser {
        pub fn init(tokens: Vec<Tokens>) -> Parser {
            Parser {
                tokens,
                index: -1,
                output: String::new()
            }
        }

        pub fn advance(&mut self, amount: i64) -> bool {
            if self.index + amount > (self.tokens.len() - 1) as i64 { return false }
            self.index += amount;
            return true;
        }
        
        pub fn parse_type(type_str: String) -> String {
            match type_str.as_str() {
                "string" => "String".to_string(),
                "int" => "i32".to_string(),
                _ => return type_str
            }
        }

        pub fn parse_current_token(&mut self) -> String {
            let current = &self.tokens[self.index as usize];
            match current {
                Tokens::LParen
                | Tokens::RParen
                | Tokens::LCurly
                | Tokens::RCurly
                | Tokens::QuotationMark
                | Tokens::RBracket
                | Tokens::LBracket
                => { return current.to_char().to_string()},
                Tokens::FunctionDef(x, y, z) => {
                    let args: String = {
                        let mut str = String::new();
                        for arg in y {
                            str.push_str(&format!("{}: {},", &arg.v, Self::parse_type(arg.t.to_string())))
                        };
                        str.strip_suffix(|_: char| true).unwrap_or(&str).to_string()
                    };
                    let return_type = {
                        let ret_type = Self::parse_type(z.to_string());
                        if ret_type == "void" { "".to_string() }
                        else { format!(" -> {}", ret_type)} 
                    };
                    return format!("fn {}({}){}", x , args, return_type)
                },
                Tokens::FunctionCall(x, y) => {
                    let args: String = {
                        let mut str = String::new();
                        for arg in y {
                            str.push_str(&format!("{},", &arg.v))
                        };
                        str.strip_suffix(|_: char| true).unwrap_or(&str).to_string()
                    };
                    return format!("{}({})", x, args )
                }
                _ => { return "".to_string() }
            }
        }
    }
}