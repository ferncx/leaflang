pub mod token {

    #[derive(Debug, PartialEq)]
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
        FunctionDef(String, Vec<(Types, String)>, Types),
        Invalid,
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
                _ => ' '
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Types {
        Int,
        Float,
        String,
        Bool,
        Void,
        Invalid
    }

    impl Types {
        pub fn parse_type(s: &str) -> Types {
            match s {
                "int" => Types::Int,
                "float" => Types::Float,
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
                Types::Invalid => "invalid".to_string()
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Exceptions {
        InvalidToken,
        InvalidType
    }

    pub struct Exception {
        pub error: Exceptions,
        pub character_pos: i64,
    }
}