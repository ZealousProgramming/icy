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
    LAngle,         // <
    RAngle,         // >
    LBracket,       // [
    RBracket,       // ]
    Comma,          // ,
    Colon,          // :
    SemiColon,      // ;

    // Operators
    Equals,         // =
    EqualsEquals,   // ==
    Infer,          // :=
    Bang,           // !
    BangEquals,     // !=
    Plus,           // +
    Minus,          // -
    Slash,          // /
    Asterisk,       // *
    QMark,          // ?

    // Literals
    Ident,     
    String,         // string
    Int,            // int


    // Keywords
    Function,       // Function decl
    Var,            // Variable decl
    Void,           // void
    Return,         // return
    True,           // true
    False,          // false
    If,             // if
    Else,           // Else
    // And,            // and
    // Or,             // or
    // For,            // for


}