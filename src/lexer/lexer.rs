use crate::token::token::{
    Token,
    lookup_indentifier,
};
use crate::token::token_kind::TokenKind;
pub struct Lexer {
    source: String, // The source code
    pos: usize, // The position from the start of the source
    offset: usize,  // The offset from `pos`
    cc: Option<char>, // Current Character
    line: usize, // The current line in the source code
}

impl Lexer {
    pub fn new(source: &str) -> Self {
       let mut inst = Self { 
            source: String::from(source),
            pos: 0,
            offset: 0,
            cc: None,
            line: 1,
        };

        inst.read();

        inst
    }

    pub fn next(self: &mut Self) -> Token {
        let token: Token;

        self.ignore_whitespace();

        match self.cc {
            Some(c) => {
                let mut buff = [0; 4];// 4 bytes to cover for any UTF-8 characters
                let char_str = c.encode_utf8(&mut buff);
                match c {
                    '\n' => token = Token::new(TokenKind::Newline, char_str, self.line),
                    '(' => token = Token::new(TokenKind::LParen, char_str, self.line),
                    ')' => token = Token::new(TokenKind::RParen, char_str, self.line),
                    '{' => token = Token::new(TokenKind::LBrace, char_str, self.line),
                    '}' => token = Token::new(TokenKind::RBrace, char_str, self.line),
                    ',' => token = Token::new(TokenKind::Comma, char_str, self.line),
                    ':' => token = Token::new(TokenKind::Colon, char_str, self.line),
                    
                    // Operators
                    '=' => token = Token::new(TokenKind::Equals, char_str, self.line),
                    '+' => token = Token::new(TokenKind::Plus, char_str, self.line),
                    '-' => token = Token::new(TokenKind::Minus, char_str, self.line),
                    
                    _ => {
                        if self.is_alphabetic(Some(c)) {
                            // Read Identifier
                            let starting_pos = self.pos;

                            while self.is_alphabetic(self.cc) {
                                self.read();
                            }

                            let literal = &self.source[starting_pos..self.pos];

                            token = Token::new(lookup_indentifier(literal), literal, self.line);
                            return token;
                        } else if self.is_numeric(Some(c)) {
                            // Read Number
                            let starting_pos = self.pos;

                            while self.is_numeric(self.cc) {
                                self.read();
                            }

                            let literal = &self.source[starting_pos..self.pos];


                            token = Token::new(TokenKind::Int, literal, self.line);
                            return token;
                        }else {
                            _ = eprintln!("LINE {:?} Unexpected character", self.line);
                            token = Token::new(TokenKind::Illegal, char_str, self.line);
                        }
                    }

                }
            },
            None => token = Token::new(TokenKind::Eof, "", self.line), // EOF
        }
        
        self.read();

        token
    }

    fn read(self: &mut Self) {
        if self.offset >= self.source.len() {
            self.cc = None;
        } else {
            self.cc = self.source.chars().nth(self.offset);
        }

        self.pos = self.offset;
        self.offset += 1;
    }

    fn is_alphabetic(self: &Self, copt: Option<char>) -> bool {
        match copt {
            Some(c) => return c.is_alphabetic() || c == '_',
            None => return false,
        }
    }

    fn is_numeric(self: &Self, copt: Option<char>) -> bool {
        match copt {
            Some(c) => return c.is_numeric(),
            None => return false,
        }
    }

    fn ignore_whitespace(self: &mut Self) {
        let mut found_non_ws = false;
        while !found_non_ws {
            if let Some(c) = self.cc {
                if c == ' ' || c == '\t' || c == '\r' {
                    self.read();
                } else {
                    found_non_ws = true;
                }
            }
        }
    }
    

}



// ----- Tests -----

#[cfg(test)]
mod lexer_tests {
    use std::fs;
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;
    use crate::token::token_kind::TokenKind;

    #[test]
    fn next_token() {
        println!("[lexer_tests]: next_token");
        let delim_and_ops: String = String::from("=+-(){},:\n");
        let test_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Equals, "=", 1),
            Token::new(TokenKind::Plus,   "+", 1),
            Token::new(TokenKind::Minus,  "-", 1),
            Token::new(TokenKind::LParen, "(", 1),
            Token::new(TokenKind::RParen, ")", 1),
            Token::new(TokenKind::LBrace, "{", 1),
            Token::new(TokenKind::RBrace, "}", 1),
            Token::new(TokenKind::Comma,  ",", 1),
            Token::new(TokenKind::Colon,  ":", 1),
            Token::new(TokenKind::Newline, "\n", 1),
        ];

        let mut lexer: Lexer = Lexer::new(&delim_and_ops);

        for (_index, tt) in test_tokens.iter().enumerate() {
            let token = lexer.next();

            assert_eq!(token.kind, tt.kind);
            assert_eq!(token.literal, tt.literal);
        }
    }

    #[test]
    fn basic_add() {
        println!("[lexer_tests]: basic main");

        let bytes: String = fs::read_to_string("src/tests/icy/add.icy").expect("File could not be found");
        
        let test_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Newline, "\n", 1),
            // var ten = 10
            Token::new(TokenKind::Var, "var", 2),
            Token::new(TokenKind::Ident, "ten", 2),
            // Token::new(TokenKind::InferEquals, ":=", 2),
            Token::new(TokenKind::Equals, "=", 2),
            Token::new(TokenKind::Int, "10", 2),
            Token::new(TokenKind::Newline, "\n", 2),
            // var five = 5
            Token::new(TokenKind::Var, "var", 3),
            Token::new(TokenKind::Ident, "five", 3),
            // Token::new(TokenKind::InferEquals, ":=", 3),
            Token::new(TokenKind::Equals, "=", 3),
            Token::new(TokenKind::Int, "5", 3),
            Token::new(TokenKind::Newline, "\n", 3),

            Token::new(TokenKind::Newline, "\n", 3),

            // var added = add(five, ten)
            Token::new(TokenKind::Var, "var", 5),
            Token::new(TokenKind::Ident, "added", 5),
            // Token::new(TokenKind::InferEquals, ":=", 5),
            Token::new(TokenKind::Equals, "=", 5),
            Token::new(TokenKind::Ident, "add", 5),
            Token::new(TokenKind::LParen, "(", 5),
            Token::new(TokenKind::Ident, "five", 5),
            Token::new(TokenKind::Comma, ",", 5),
            Token::new(TokenKind::Ident, "ten", 5),
            Token::new(TokenKind::RParen, ")", 5),
            Token::new(TokenKind::Newline, "\n", 5),

            Token::new(TokenKind::Newline, "\n", 6),

            // func add(x, y) {
            Token::new(TokenKind::Function, "func", 7),
            Token::new(TokenKind::Ident, "add", 7),
            Token::new(TokenKind::LParen, "(", 7),
            Token::new(TokenKind::Ident, "x", 7),
            Token::new(TokenKind::Comma, ",", 7),
            Token::new(TokenKind::Ident, "y", 7),
            Token::new(TokenKind::RParen, ")", 7),
            Token::new(TokenKind::LBrace, "{", 7),
            Token::new(TokenKind::Newline, "\n", 7),

            // return x + y
            Token::new(TokenKind::Return, "return", 8),
            Token::new(TokenKind::Ident, "x", 8),
            Token::new(TokenKind::Plus, "+", 8),
            Token::new(TokenKind::Ident, "y", 8),
            Token::new(TokenKind::Newline, "\n", 8),

            // }
            Token::new(TokenKind::RBrace, "}", 9),

        ];

        let mut lexer: Lexer = Lexer::new(bytes.as_str());

        for (_index, tt) in test_tokens.iter().enumerate() {
            let token = lexer.next();

            // println!("{:?}", token.literal);
            assert_eq!(token.kind, tt.kind);
            assert_eq!(token.literal, tt.literal);
        }
    }
}