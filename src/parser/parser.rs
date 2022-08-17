use crate::token::{
    token_kind::TokenKind,
    token::Token
};
use crate::lexer::lexer::Lexer;
use crate::ast::ast::{
    Program,
    Statement, Identifier,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser_inst = Self { 
            lexer, 
            current_token: Token::new(TokenKind::Illegal, "", 0),
            peek_token: Token::new(TokenKind::Illegal, "", 0),
        };

        parser_inst.next();
        parser_inst.next();

        return parser_inst;
    }

    pub fn parse(self: &mut Self) -> Program {
        let mut program = Program::new();

       loop {
            if self.current_token.kind != TokenKind::Eof {
                if let Some(s) = self.parse_statement() {
                    program.statements.push(s);
                }

                self.next();
            } else {
                break;
            }
        }


        program
    }


    fn next(self: &mut Self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    fn peek_match(self: &mut Self, t: TokenKind) -> bool {
        if self.peek_token.kind == t {
            self.next();
            
            return true;
        }

        false
    }

    fn parse_statement(self: &mut Self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Var => return self.parse_var_statement(),
            _ => None
        }
    }

    fn parse_var_statement(self: &mut Self) -> Option<Statement> {

        match self.peek_token.kind {
            TokenKind::Ident(_) => self.next(),
            _ => return None,
        }

        let ident = Identifier(self.current_token.literal.clone());

        match self.peek_token.kind {
            TokenKind::Equals => self.next(),
           _ => return None,
        }

        loop {
            if self.current_token.kind != TokenKind::Newline {
                self.next();
            } else {
                break;
            }
        }

        Some(Statement::Var(ident, None))
    }
}

// ----- Tests -----

#[cfg(test)]
mod parser_tests {
    use std::fs;
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;
    use crate::ast::ast::{
        Program,
        Statement
    };

    #[test]
    fn var_binding() {
        println!("[parser_tests]: bindings");

        let bytes: String = fs::read_to_string("src/tests/icy/bindings.icy").expect("File could not be found");

        let mut parser = Parser::new(Lexer::new(bytes.as_str()));

        let program: Program = parser.parse();

        let tests = vec![
            "bind_me_daddy".to_string(),
            "tenner".to_string(),
            "virus_nomnom".to_string()
        ];

        for (index, tt) in tests.iter().enumerate() {
            match &program.statements[index] {
                Statement::Var(ident, _) => assert_eq!(&ident.0, tt),
                //_ => assert!(false),
            }
        }

    }
}
