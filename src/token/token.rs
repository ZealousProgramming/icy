use std::fmt;
use phf::{phf_map, Map};

use super::token_kind::TokenKind;

#[allow(dead_code)]
static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map!{
    "func" => TokenKind::Function,
    "var" => TokenKind::Var,
    "void" => TokenKind::Void,
    "return" => TokenKind::Return,
};

pub fn lookup_indentifier(token_indent: &str) -> TokenKind {
    if let Some(tok) = KEYWORDS.get(token_indent).cloned() {
        tok
    } else {
        TokenKind::Ident
    }
}


pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, literal: &str, line: usize) -> Self {
        Self {
            kind,
            literal: String::from(literal),
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {}", self.line, self.kind, self.literal)
    }
}