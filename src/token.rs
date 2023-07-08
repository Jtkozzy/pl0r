use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,                //Nothing left to read
    WhiteSpace(char),   //All whitespace
    Identifier(String), //Names of variables & procedures
    Number(i32),        //Only integers
    Plus,               //+
    Minus,              //-
    Times,              //*
    Slash,              // /

    Equal,        //=
    NotEqual,     //# (yes this is due to original restriction of one char)
    Less,         // <
    LessEqual,    // [ (another one character odd thing)
    Greater,      // >
    GreaterEqual, // ] (yet another one character odd thing)

    LParen,     // (
    RParen,     // )
    Comma,      // ;
    Semicolon,  // ;
    Period,     // .
    Becomes,    // := (a traditional Pascal assignment)
    BecomesErr, //Becomes started correctly with ':' but did not end in '='. Unused in parser.

    BeginSym, // begin
    EndSym,   // end
    IfSym,    // if
    ThenSym,  // then
    WhileSym, // while
    DoSym,    // do
    CallSym,  // call (for easier parsing versus Identifers (look EBNF))
    ConstSym, // const
    VarSym,   // var
    ProcSym,  // procedure
    OddSym,   //Inbuilt function odd

    WriteSym, // !
    ReadSym,  // ?
}

pub fn sym_relational(sym: &Token) -> bool {
    match sym {
        Token::Equal
        | Token::NotEqual
        | Token::Less
        | Token::GreaterEqual
        | Token::Greater
        | Token::LessEqual => true,
        _ => false,
    }
}

pub fn sym_in_facbegsys(sym: &Token) -> bool {
    match sym {
        Token::Identifier(_) | Token::Number(_) | Token::LParen => true,
        _ => false,
    }
}

pub fn sym_in_declbegsys(sym: &Token) -> bool {
    match sym {
        Token::ConstSym | Token::VarSym | Token::ProcSym => true,
        _ => false,
    }
}

pub fn sym_in_statbegsys_plus_semicolon(sym: &Token) -> bool {
    match sym {
        Token::Identifier(_) | Token::Number(_) | Token::LParen | Token::Semicolon => true,
        _ => false,
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Eof | Token::BecomesErr => write!(f, ""),
            Token::WhiteSpace(c) => write!(f, "{}", c),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Equal => write!(f, "="),
            Token::NotEqual => write!(f, "#"),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "["),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, "]"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::Becomes => write!(f, ":="),
            Token::BeginSym => write!(f, "begin"),
            Token::EndSym => write!(f, "end"),
            Token::IfSym => write!(f, "if"),
            Token::ThenSym => write!(f, "then"),
            Token::WhileSym => write!(f, "while"),
            Token::DoSym => write!(f, "do"),
            Token::CallSym => write!(f, "call"),
            Token::ConstSym => write!(f, "const"),
            Token::VarSym => write!(f, "var"),
            Token::ProcSym => write!(f, "procedure"),
            Token::OddSym => write!(f, "odd"),
            Token::WriteSym => write!(f, "!"),
            Token::ReadSym => write!(f, "?"),
        }
    }
}
