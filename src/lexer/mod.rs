pub mod lexer {
    use crate::tokens::token::Tokens;

    pub struct Lexer {
       pub contents: Vec<char>,
       pub index: u8,
    }

    impl Lexer {
        pub fn current(&self) -> char {
            self.contents[self.index as usize]
        }

        pub fn init(contents: String) -> Lexer {
            let contents = contents.chars().collect::<Vec<char>>();
            Lexer {
                contents,
                index: 0,
            }
        }

        pub fn advance(&mut self, amount: u8) -> bool {
            if (self.index + amount) as usize >= self.contents.len() {
                return false;
            }
            self.index += amount;
            true
        }

        pub fn skip_whitespace(&mut self) {
            while self.current() == ' ' /* whitespace ASCII */ {
                self.index += 1
            }
        }

        pub fn get_next_token(&mut self) -> Tokens {
            Tokens::from_char(self.current())     
        }
    }
}