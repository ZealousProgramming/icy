#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TokenKind {
    Illegal,
    Eof,

    // Delimiters
    Newline,        // \n
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    Comma,          // ,
    Colon,          // :

    // Operators
    Equals,         // =
    InferEquals,    // :=
    Plus,           // +
    Minus,          // -

    // Literals
    Ident,     
    String,         // string
    Int,            // int


    // Keywords
    Function,       // Function decl
    Var,            // Variable decl
    Void,           // void
    Return,         // return

}