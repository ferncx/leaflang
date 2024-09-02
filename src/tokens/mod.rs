
pub mod token {
    use strum_macros::EnumTryAs;


    #[derive(Debug, PartialEq, Clone, EnumTryAs)]
    pub enum Tokens {
        LParen,
        RParen,
        LBracket,
        RBracket,
        LCurly,
        RCurly,
        QuotationMark,
        //     contents
        String(String),
        //          name    args         return type
        FunctionDef(String, Vec<Variable>, Types),
        Invalid,
        FunctionCall(String, Vec<Variable>),
        Return((Option<Variable>, String)),
        Semicolon,
        None // handy to avoid options
    }


    impl Tokens {
        pub fn from_char(c: char) -> Tokens {
            match c {
                '(' => Tokens::LParen,
                ')' => Tokens::RParen,
                '[' => Tokens::LBracket,
                ']' => Tokens::RBracket,
                '{' => Tokens::LCurly,
                '}' => Tokens::RCurly,
                '"' => Tokens::QuotationMark,
                ';' => Tokens::Semicolon,
                _ => Tokens::Invalid
            }
        }
        pub fn to_char(&self) -> char {
            match self {
                Tokens::LParen => '(',
                Tokens::RParen => ')',
                Tokens::LBracket => '[',
                Tokens::RBracket => ']',
                Tokens::LCurly => '{',
                Tokens::RCurly => '}',
                Tokens::QuotationMark => '"',
                Tokens::Invalid => 'i',
                Tokens::FunctionDef(_, _, _) => 'f',
                Tokens::Semicolon => ';',
                _ => ' '
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Types {
        Int,
        Float,
        String,
        Bool,
        Void,
        Invalid,
        Placeholder // used when we don't know the type yet (type inference). will be replaced later on, should never be a final type
    }

    impl Types {
        pub fn parse_type(s: &str) -> Types {
            match s {
                "i64" | "i32" | "u8" | "u16" => Types::Int,
                "f32" | "f64" => Types::Float,
                "string" => Types::String,
                "bool" => Types::Bool,
                "void" => Types::Void,
                _ => Types::Invalid
            }
        }
        pub fn to_string(&self) -> String {
            match self {
                Types::Int => "int".to_string(),
                Types::Float => "float".to_string(),
                Types::String => "string".to_string(),
                Types::Bool => "bool".to_string(),
                Types::Void => "void".to_string(),
                Types::Invalid => "invalid".to_string(),
                Types::Placeholder => "placeholder".to_string(),
            }
        }
        
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Variable {
        pub t: Types,
        pub n: String, // Name
        pub v: String // converted later, can't use Any
    }

    #[derive(Debug, PartialEq)]
    pub enum Exceptions {
        InvalidToken,
        InvalidType
    }

    pub struct Exception {
        pub error: Exceptions,
        pub message: String,
        pub character_pos: i64,
    }
}