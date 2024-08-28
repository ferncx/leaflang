pub mod token {

    #[derive(Debug)]
    pub enum Tokens {
        LParen,
        RParen,
        LBracket,
        RBracket,
        LCurly,
        RCurly,
        QuotationMark,
        Character(u8),
        String,
        FunctionDef,
        Invalid
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
                'a'..='z' | 'A'..='Z' | '0'..='9' => Tokens::Character(c.to_ascii_lowercase() as u8),
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
                Tokens::Character(c) => *c as char,
                Tokens::String => 's',
                Tokens::FunctionDef => 'f',
                Tokens::Invalid => 'i'
            }
        }
    }
}