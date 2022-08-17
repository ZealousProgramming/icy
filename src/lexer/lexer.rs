use crate::token::token::{
    Token,
    lookup_indentifier,
};
use crate::token::token_kind::TokenKind;
pub struct Lexer<'a> {
    source: &'a str, // The source code
    pos: usize, // The position from the start of the source
    offset: usize,  // The offset from `pos`
    cc: Option<char>, // Current Character
    line: usize, // The current line in the source code
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
       let mut inst = Self { 
            source,
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
                    '<' => token = Token::new(TokenKind::LAngle, char_str, self.line),
                    '>' => token = Token::new(TokenKind::RAngle, char_str, self.line),
                    '[' => token = Token::new(TokenKind::LBracket, char_str, self.line),
                    ']' => token = Token::new(TokenKind::RBracket, char_str, self.line),
                    ',' => token = Token::new(TokenKind::Comma, char_str, self.line),
                    ';' => token = Token::new(TokenKind::SemiColon, char_str, self.line),
                    ':' => {
                        token = self.peek_match('=', TokenKind::Infer, TokenKind::Colon, ":=", char_str);
                    }
                    
                    // Operators
                    '=' => { 
                        token = self.peek_match('=', TokenKind::EqualsEquals, TokenKind::Equals, "==", char_str);
                    },
                    '!' => { 
                        token = self.peek_match('=', TokenKind::BangEquals, TokenKind::Bang, "!=", char_str);
                    }, 
                    '+' => token = Token::new(TokenKind::Plus, char_str, self.line),
                    '-' => token = Token::new(TokenKind::Minus, char_str, self.line),
                    '*' => token = Token::new(TokenKind::Asterisk, char_str, self.line),
                    '/' => token = Token::new(TokenKind::Slash, char_str, self.line),
                    '?' => token = Token::new(TokenKind::QMark, char_str, self.line),
                    
                    
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


                            token = Token::new(TokenKind::Int(literal.parse::<i64>().unwrap()), literal, self.line);
                            return token;
                        }else {
                            _ = eprintln!("LINE {:?} Unexpected character", self.line);
                            token = Token::new(TokenKind::Illegal, char_str, self.line);
                        }
                    }

                }
            },
            None => {
                return Token::new(TokenKind::Eof, "", self.line); // EOF
            },
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

    fn peek(self: & Self) -> Option<char> {
        if self.offset >= self.source.len() {
            None
        } else {
            self.source.chars().nth(self.offset)
        }
    }

    fn peek_match(self: &mut Self, c: char, match_token_kind: TokenKind, else_token_kind: TokenKind, match_literal: &str, else_literal: &str) -> Token {
        let peek_result = self.peek();
        let has_value = peek_result.is_some();

        if has_value && peek_result.unwrap() == c {
            self.read();
            Token::new(match_token_kind, match_literal, self.line)
        } else {
            Token::new(else_token_kind, else_literal, self.line)
        }
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
            } else {
                found_non_ws = true;
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
    fn kinds() {
        println!("[lexer_tests]: kinds");
        // let delim_and_ops: String = String::from("=+-(){},:\n==:=!!=/*?;<>[]");
        let delim_and_ops: String = String::from("=+-(){},:\n!/*?;<>[]");
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
            // Token::new(TokenKind::EqualsEquals, "==", 2),
            // Token::new(TokenKind::Infer, ":=", 2),
            Token::new(TokenKind::Bang, "!", 2),
            // Token::new(TokenKind::BangEquals, "!=", 2),
            Token::new(TokenKind::Slash, "/", 2),
            Token::new(TokenKind::Asterisk, "*", 2),
            Token::new(TokenKind::QMark, "?", 2),
            Token::new(TokenKind::SemiColon, ";", 2),
            Token::new(TokenKind::LAngle, "<", 2),
            Token::new(TokenKind::RAngle, ">", 2),
            Token::new(TokenKind::LBracket, "[", 2),
            Token::new(TokenKind::RBracket, "]", 2),
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
        println!("[lexer_tests]: basic add");

        let bytes: String = fs::read_to_string("src/tests/icy/add.icy").expect("File could not be found");
        
        let test_tokens: Vec<Token> = vec![
            Token::new(TokenKind::Newline, "\n", 1),
            // var ten = 10
            Token::new(TokenKind::Var, "var", 2),
            Token::new(TokenKind::Ident("ten".to_string()), "ten", 2),
            // Token::new(TokenKind::InferEquals, ":=", 2),
            Token::new(TokenKind::Equals, "=", 2),
            Token::new(TokenKind::Int(10), "10", 2),
            Token::new(TokenKind::Newline, "\n", 2),
            // var five = 5
            Token::new(TokenKind::Var, "var", 3),
            Token::new(TokenKind::Ident("five".to_string()), "five", 3),
            // Token::new(TokenKind::InferEquals, ":=", 3),
            Token::new(TokenKind::Equals, "=", 3),
            Token::new(TokenKind::Int(5), "5", 3),
            Token::new(TokenKind::Newline, "\n", 3),

            Token::new(TokenKind::Newline, "\n", 3),

            // var added = add(five, ten)
            Token::new(TokenKind::Var, "var", 5),
            Token::new(TokenKind::Ident("added".to_string()), "added", 5),
            // Token::new(TokenKind::InferEquals, ":=", 5),
            Token::new(TokenKind::Equals, "=", 5),
            Token::new(TokenKind::Ident("add".to_string()), "add", 5),
            Token::new(TokenKind::LParen, "(", 5),
            Token::new(TokenKind::Ident("five".to_string()), "five", 5),
            Token::new(TokenKind::Comma, ",", 5),
            Token::new(TokenKind::Ident("ten".to_string()), "ten", 5),
            Token::new(TokenKind::RParen, ")", 5),
            Token::new(TokenKind::Newline, "\n", 5),

            Token::new(TokenKind::Newline, "\n", 6),

            // func add(x, y) {
            Token::new(TokenKind::Function, "func", 7),
            Token::new(TokenKind::Ident("add".to_string()), "add", 7),
            Token::new(TokenKind::LParen, "(", 7),
            Token::new(TokenKind::Ident("x".to_string()), "x", 7),
            Token::new(TokenKind::Comma, ",", 7),
            Token::new(TokenKind::Ident("y".to_string()), "y", 7),
            Token::new(TokenKind::RParen, ")", 7),
            Token::new(TokenKind::LBrace, "{", 7),
            Token::new(TokenKind::Newline, "\n", 7),

            // return x + y
            Token::new(TokenKind::Return, "return", 8),
            Token::new(TokenKind::Ident("x".to_string()), "x", 8),
            Token::new(TokenKind::Plus, "+", 8),
            Token::new(TokenKind::Ident("y".to_string()), "y", 8),
            Token::new(TokenKind::Newline, "\n", 8),

            // }
            Token::new(TokenKind::RBrace, "}", 9),
            Token::new(TokenKind::Newline, "\n", 9),

            Token::new(TokenKind::Newline, "\n", 10),
            
            // if(added > 10) {
            Token::new(TokenKind::If, "if", 11),
            Token::new(TokenKind::LParen, "(", 11),
            Token::new(TokenKind::Ident("added".to_string()), "added", 11),
            Token::new(TokenKind::RAngle, ">", 11),
            Token::new(TokenKind::Int(10), "10", 11),
            Token::new(TokenKind::RParen, ")", 11),
            Token::new(TokenKind::LBrace, "{", 11),
            Token::new(TokenKind::Newline, "\n", 11),
            
            // return true
            Token::new(TokenKind::Return, "return", 12),
            Token::new(TokenKind::True, "true", 12),
            Token::new(TokenKind::Newline, "\n", 12),

            // } else {
            Token::new(TokenKind::RBrace, "}", 13),
            Token::new(TokenKind::Else, "else", 13),
            Token::new(TokenKind::LBrace, "{", 13),
            Token::new(TokenKind::Newline, "\n", 13),

            // return false
            Token::new(TokenKind::Return, "return", 14),
            Token::new(TokenKind::False, "false", 14),
            Token::new(TokenKind::Newline, "\n", 14),
            
            // }
            Token::new(TokenKind::RBrace, "}", 15),
            Token::new(TokenKind::Newline, "\n", 15),

            Token::new(TokenKind::Newline, "\n", 16),

            // 10 == 9
            Token::new(TokenKind::Int(10), "10", 17),
            Token::new(TokenKind::EqualsEquals, "==", 17),
            Token::new(TokenKind::Int(9), "9", 17),
            Token::new(TokenKind::Newline, "\n", 17),

            // 10 != 8
            Token::new(TokenKind::Int(10), "10", 18),
            Token::new(TokenKind::BangEquals, "!=", 18),
            Token::new(TokenKind::Int(8), "8", 18),
            Token::new(TokenKind::Newline, "\n", 18),

            // var infer_me_daddy := 1
            Token::new(TokenKind::Var, "var", 19),
            Token::new(TokenKind::Ident("infer_me_daddy".to_string()), "infer_me_daddy", 19),
            Token::new(TokenKind::Infer, ":=", 19),
            Token::new(TokenKind::Int(1), "1", 19),

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
