use crate::defs::EX_NOINPUT;
use crate::scan_error;
use crate::token::Token;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]

pub struct Scanner {
    source: Vec<char>,
    pub line: i32,
    start: i32,
    current: i32,
    keywords: HashMap<String, Token>,
}

impl Scanner {
    pub fn new(src: &str) -> Scanner {
        let source = match read_to_string(src) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Could not read source file {src}, error {e}");
                std::process::exit(EX_NOINPUT);
            }
        };

        let mut kw: HashMap<String, Token> = HashMap::new();
        kw.insert("begin".to_owned(), Token::BeginSym);
        kw.insert("call".to_owned(), Token::CallSym);
        kw.insert("const".to_owned(), Token::ConstSym);
        kw.insert("do".to_owned(), Token::DoSym);
        kw.insert("end".to_owned(), Token::EndSym);
        kw.insert("if".to_owned(), Token::IfSym);
        kw.insert("odd".to_owned(), Token::OddSym);
        kw.insert("procedure".to_owned(), Token::ProcSym);
        kw.insert("then".to_owned(), Token::ThenSym);
        kw.insert("var".to_owned(), Token::VarSym);
        kw.insert("while".to_owned(), Token::WhileSym);

        Self {
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
            keywords: kw,
        }
    }
}

pub fn next_sym(s: &mut Scanner) -> Token {
    if !is_at_end(s) {
        //We are at the beginning of next lexeme
        s.start = s.current;
        return scan_token(s);
    } else {
        return Token::Eof;
    }
}

fn scan_token(s: &mut Scanner) -> Token {
    let ret: Token;
    let c = advance(s);
    match c {
        '(' => ret = Token::LParen,
        ')' => ret = Token::RParen,
        ',' => ret = Token::Comma,
        '.' => ret = Token::Period,
        '-' => ret = Token::Minus,
        '+' => ret = Token::Plus,
        '*' => ret = Token::Times,
        '#' => ret = Token::NotEqual,
        '=' => ret = Token::Equal,
        ';' => ret = Token::Semicolon,
        '/' => ret = Token::Slash,
        ':' => {
            if _match(s, '=') {
                s.current += 1;
                ret = Token::Becomes;
            } else {
                ret = Token::BecomesErr;
            }
        }
        '<' => ret = Token::Less,
        '>' => ret = Token::Greater,
        '[' => ret = Token::LessEqual,
        ']' => ret = Token::GreaterEqual,
        '!' => ret = Token::WriteSym,
        '?' => ret = Token::ReadSym,
        ' ' | '\r' | '\t' => ret = Token::WhiteSpace(c),
        '\n' => {
            s.line += 1;
            ret = Token::WhiteSpace(c);
        }
        _ => {
            if is_digit(c) {
                let sn = number(s);
                ret = sn;
            } else if is_alpha(c) {
                let si = identifier(s);
                ret = si;
            } else {
                ret = Token::WhiteSpace(' ');
                scan_error(s.line, "unexpected character");
            }
        }
    }
    ret
}

fn peek(s: &Scanner) -> char {
    if is_at_end(s) {
        return '\0';
    }
    s.source[s.current as usize]
}

fn advance(s: &mut Scanner) -> char {
    let i = s.current;
    s.current += 1;
    s.source[i as usize]
}

fn is_at_end(s: &Scanner) -> bool {
    s.current as usize >= s.source.len()
}

fn identifier(s: &mut Scanner) -> Token {
    while is_alphanumeric(peek(s)) {
        advance(s);
    }

    let mut v = vec![];
    for i in s.start..s.current {
        v.push(s.source[i as usize]);
    }
    let st: String = v.iter().collect();
    let mut _type = s.keywords.get(&st);
    let tt: Token;
    if _type.is_none() {
        tt = Token::Identifier(st);
    } else {
        tt = _type.unwrap().clone();
    }
    tt
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn number(s: &mut Scanner) -> Token {
    while is_digit(peek(s)) {
        advance(s);
    }

    let mut v = vec![];
    for i in s.start..s.current {
        v.push(s.source[i as usize]);
    }
    let st: String = v.iter().collect();
    let n: i32 = str::parse::<i32>(&st).unwrap();
    Token::Number(n)
}

fn _match(s: &Scanner, expected: char) -> bool {
    if is_at_end(s) {
        return false;
    }
    if s.source[s.current as usize] != expected {
        return false;
    }
    true
}
