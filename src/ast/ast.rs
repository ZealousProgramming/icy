use crate::token::{
    token::Token,
    token_kind::TokenKind,
};
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);


#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Var(Identifier, Option<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident,
}
