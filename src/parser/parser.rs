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
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser_inst = Self { 
            lexer, 
            current_token: Token::new(TokenKind::Illegal, "", 0),
            peek_token: Token::new(TokenKind::Illegal, "", 0),
            errors: Vec::new(),
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

    // fn peek_match(self: &mut Self, t: TokenKind) -> bool {
    //     if self.peek_token.kind == t {
    //         self.next();
            
    //         return true;
    //     }

    //     self.peek_error(t);
    //     false
    // }

    fn parse_statement(self: &mut Self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Var => return self.parse_var_statement(),
            _ => None
        }
    }

    fn parse_var_statement(self: &mut Self) -> Option<Statement> {

        match self.peek_token.kind {
            TokenKind::Ident => self.next(),
            _ => {
                self.peek_error(TokenKind::Ident);
                return None
            },
        }

        let ident = Identifier(self.current_token.literal.clone());

        match self.peek_token.kind {
            TokenKind::Equals => self.next(),
            _ => {
                self.peek_error(TokenKind::Equals);
                return None
            },
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

    fn peek_error(self: &mut Self, kind: TokenKind) {
        self.errors.push(format!("expected next token to be {:?}, but found {:?}", kind, self.peek_token.kind));
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
    fn parse_errors() {
        println!("[parser_tests]: errors");

        let bytes: String = fs::read_to_string("src/tests/icy/parsing_errors.icy").expect("File could not be found");

        let mut parser = Parser::new(Lexer::new(bytes.as_str()));

        let _program: Program = parser.parse();

        // Check for Parser Errors
        let errors = parser.errors;

        // for error in errors.iter().enumerate() {
        //     println!("Paser Error: {:?}", error);
        // }

        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn var_binding() {
        println!("[parser_tests]: bindings");

        let bytes: String = fs::read_to_string("src/tests/icy/bindings.icy").expect("File could not be found");

        let mut parser = Parser::new(Lexer::new(bytes.as_str()));

        let program: Program = parser.parse();

        // Check for Parser Errors
        let errors = parser.errors;

        if errors.len() > 0 {
            for error in errors.iter().enumerate() {
                println!("Paser Error: {:?}", error);
            }

            assert_eq!(errors.len(), 0);
        }

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
