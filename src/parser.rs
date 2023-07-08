use crate::scanner::{next_sym, Scanner};
use crate::token::*;
use crate::{defs::*, parse_error, scan_error};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ObjType {
    Constant,
    Variable,
    Procedure,
}

const POS_NOT_FOUND: i32 = 0;

#[derive(Clone, Debug)]
struct ObjDesc {
    name: String,
    kind: ObjType,
    val_or_lev: i32,
    adr: i32,
}

pub struct Parser {
    s: Scanner,
    sym: Token,
    table: Vec<ObjDesc>,        //Identifier table array (well, vector)
    pub code: Vec<Instruction>, //Code array (well, vector)
    cx: i32,                    //Code allocation index
    line: i32,                  //For code listing output
}

impl Parser {
    pub fn new(src: &str) -> Self {
        let s = Scanner::new(src);

        Parser {
            s,
            sym: Token::WhiteSpace(' '),
            table: Vec::with_capacity(ID_TABLE_LEN as usize),
            code: Vec::with_capacity(CODE_ARR_SIZE as usize),
            cx: 0,
            line: 0,
        }
    }
}

pub fn get_one_sym(p: &mut Parser) -> Token {
    let tok = next_sym(&mut p.s);
    if p.line != p.s.line {
        if p.line == 0 {
            //first
            print!("{:5} ", p.cx);
            print!("{tok}");
        } else {
            //others
            print!("{tok}");
            print!("{:5} ", p.cx);
        }
        p.line = p.s.line;
    } else {
        print!("{tok}");
    }
    tok
}

pub fn getsym(p: &mut Parser) {
    loop {
        let tok = get_one_sym(p);
        match tok {
            Token::WhiteSpace(_) => {}
            _ => {
                p.sym = tok;
                break;
            }
        }
    }
}

fn gen(p: &mut Parser, x: Fct, y: i32, z: i32) {
    let cx = p.cx as usize;
    if cx > CODE_ARR_SIZE as usize {
        scan_error(p.s.line, "program too long");
    }
    p.code[cx].fct = x;
    p.code[cx].level = y;
    p.code[cx].adr = z;
    p.cx += 1;
}

fn enter(p: &mut Parser, tx: &mut i32, name: String, k: ObjType, val_or_lev: i32, adr: &mut i32) {
    *tx += 1;
    let t = *tx as usize;
    p.table[t].name = name;
    p.table[t].kind = k;

    match k {
        ObjType::Constant => {
            if val_or_lev > ADDR_MAX {
                parse_error(p.s.line, 30);
            }
            p.table[t].val_or_lev = val_or_lev;
            p.table[t].adr = 0;
        }
        ObjType::Variable => {
            p.table[t].val_or_lev = val_or_lev;
            p.table[t].adr = *adr;
            *adr += 1;
        }
        ObjType::Procedure => {
            p.table[t].val_or_lev = val_or_lev;
            p.table[t].adr = 0;
        }
    }
}

fn position(p: &mut Parser, tx: i32, id: &String) -> i32 {
    let mut i: i32;

    p.table[POS_NOT_FOUND as usize].name = id.clone();
    i = tx;
    while p.table[i as usize].name != *id {
        i = i - 1;
    }
    i
}

fn constdeclaration(p: &mut Parser, tx: &mut i32) {
    match p.sym.clone() {
        Token::Identifier(id) => {
            getsym(p);
            if p.sym == Token::Equal || p.sym == Token::Becomes {
                if p.sym == Token::Becomes {
                    parse_error(p.s.line, 1);
                }
                getsym(p);
                match p.sym {
                    Token::Number(n) => {
                        enter(p, tx, id, ObjType::Constant, n, &mut 0);
                        getsym(p);
                    }
                    _ => parse_error(p.s.line, 2),
                }
            } else {
                parse_error(p.s.line, 3);
            }
        }
        _ => parse_error(p.s.line, 4),
    }
}

fn vardeclaration(p: &mut Parser, lev: i32, tx: &mut i32, dx: &mut i32) {
    match p.sym.clone() {
        Token::Identifier(s) => {
            enter(p, tx, s, ObjType::Variable, lev, dx);
            getsym(p);
        }
        _ => parse_error(p.s.line, 4),
    }
}

fn fct_as_i32(f: Fct) -> i32 {
    match f {
        Fct::Lit => 0,
        Fct::Opr => 1,
        Fct::Lod => 2,
        Fct::Sto => 3,
        Fct::Cal => 4,
        Fct::Int => 5,
        Fct::Jmp => 6,
        Fct::Jpc => 7,
    }
}

fn listcode(p: &Parser, cx0: i32, cx: i32) {
    //List code generated for this block (actually, incrementally for *every* block)
    println!();
    for i in cx0..cx {
        let ind = fct_as_i32(p.code[i as usize].fct);
        println!(
            "{i:>5}{:>5}{:>3}{:>5}",
            MNEMONICS[ind as usize], p.code[i as usize].level, p.code[i as usize].adr
        );
    }
}

fn expression(p: &mut Parser, lev: i32, tx: i32) {
    if p.sym == Token::Plus || p.sym == Token::Minus {
        let addop = p.sym.clone();
        getsym(p);
        term(p, lev, tx);
        if addop == Token::Minus {
            gen(p, Fct::Opr, 0, 1);
        }
    } else {
        term(p, lev, tx);
    }

    while p.sym == Token::Plus || p.sym == Token::Minus {
        let addop = p.sym.clone();
        getsym(p);
        term(p, lev, tx);
        if addop == Token::Plus {
            gen(p, Fct::Opr, 0, 2);
        } else {
            gen(p, Fct::Opr, 0, 3);
        }
    }
}

fn condition(p: &mut Parser, lev: i32, tx: i32) {
    if p.sym == Token::OddSym {
        getsym(p);
        expression(p, lev, tx);
        gen(p, Fct::Opr, 0, 6);
    } else {
        expression(p, lev, tx);
        if !sym_relational(&p.sym) {
            parse_error(p.s.line, 20)
        } else {
            let relop = p.sym.clone();
            getsym(p);
            expression(p, lev, tx);
            match relop {
                Token::Equal => gen(p, Fct::Opr, 0, 8),
                Token::NotEqual => gen(p, Fct::Opr, 0, 9),
                Token::Less => gen(p, Fct::Opr, 0, 10),
                Token::GreaterEqual => gen(p, Fct::Opr, 0, 11),
                Token::Greater => gen(p, Fct::Opr, 0, 12),
                Token::LessEqual => gen(p, Fct::Opr, 0, 13),
                _ => parse_error(p.s.line, 28),
            }
        }
    }
}

fn factor(p: &mut Parser, lev: i32, tx: i32) {
    while sym_in_facbegsys(&p.sym) {
        match p.sym.clone() {
            Token::Identifier(s) => {
                let i = position(p, tx, &s);
                if i == POS_NOT_FOUND {
                    parse_error(p.s.line, 11);
                } else {
                    let v = p.table[i as usize].clone();
                    match v.kind {
                        ObjType::Constant => gen(p, Fct::Lit, 0, v.val_or_lev),
                        ObjType::Variable => gen(p, Fct::Lod, lev - v.val_or_lev, v.adr),
                        ObjType::Procedure => parse_error(p.s.line, 21),
                    }
                }
                getsym(p);
            }
            Token::Number(n) => {
                let mut num = n;
                if num > ADDR_MAX {
                    parse_error(p.s.line, 30);
                    num = 0;
                }
                gen(p, Fct::Lit, 0, num);
                getsym(p);
            }
            Token::LParen => {
                getsym(p);
                expression(p, lev, tx);
                if p.sym == Token::RParen {
                    getsym(p);
                } else {
                    parse_error(p.s.line, 22);
                }
            }
            _ => {}
        }
    }
}

fn term(p: &mut Parser, lev: i32, tx: i32) {
    factor(p, lev, tx);
    while p.sym == Token::Times || p.sym == Token::Slash {
        let mulop = p.sym.clone();
        getsym(p);
        factor(p, lev, tx);
        if mulop == Token::Times {
            gen(p, Fct::Opr, 0, 4);
        } else {
            gen(p, Fct::Opr, 0, 5);
        }
    }
}

fn statement(p: &mut Parser, lev: i32, tx: i32) {
    match p.sym.clone() {
        Token::Identifier(s) => {
            let i = position(p, tx, &s);
            if i == POS_NOT_FOUND {
                parse_error(p.s.line, 11);
            } else {
                let v = p.table[i as usize].clone();
                match v.kind {
                    ObjType::Constant | ObjType::Procedure => {
                        //Assignment to non-variable
                        parse_error(p.s.line, 12);
                    }
                    ObjType::Variable => {
                        getsym(p);
                        if p.sym == Token::Becomes {
                            getsym(p);
                        } else {
                            parse_error(p.s.line, 13);
                        }
                        expression(p, lev, tx);
                        gen(p, Fct::Sto, lev - v.val_or_lev, v.adr);
                    }
                }
            }
        }
        Token::IfSym => {
            getsym(p);
            condition(p, lev, tx);
            if p.sym == Token::ThenSym {
                getsym(p)
            } else {
                parse_error(p.s.line, 16);
            }
            let cx1 = p.cx;
            gen(p, Fct::Jpc, 0, 0);
            statement(p, lev, tx);
            p.code[cx1 as usize].adr = p.cx;
        }
        Token::CallSym => {
            getsym(p);
            match p.sym.clone() {
                Token::Identifier(s) => {
                    let i = position(p, tx, &s);
                    if i == POS_NOT_FOUND {
                        parse_error(p.s.line, 11);
                    } else {
                        let v = p.table[i as usize].clone();
                        match v.kind {
                            ObjType::Procedure => {
                                gen(p, Fct::Cal, lev - v.val_or_lev, v.adr);
                            }
                            _ => parse_error(p.s.line, 15),
                        }
                    }
                    getsym(p);
                }
                _ => parse_error(p.s.line, 14),
            }
        }
        Token::BeginSym => {
            getsym(p);
            statement(p, lev, tx);
            while sym_in_statbegsys_plus_semicolon(&p.sym) {
                if p.sym == Token::Semicolon {
                    getsym(p);
                } else {
                    parse_error(p.s.line, 10);
                }
                statement(p, lev, tx);
            }
            if p.sym == Token::EndSym {
                getsym(p);
            } else {
                parse_error(p.s.line, 17);
            }
        }
        Token::WhileSym => {
            let cx1 = p.cx;
            getsym(p);
            condition(p, lev, tx);
            let cx2 = p.cx as usize;
            gen(p, Fct::Jpc, 0, 0);
            if p.sym == Token::DoSym {
                getsym(p);
            } else {
                parse_error(p.s.line, 18);
            }
            statement(p, lev, tx);
            gen(p, Fct::Jmp, 0, cx1);
            p.code[cx2].adr = p.cx;
        }
        Token::WriteSym => {
            getsym(p);
            expression(p, lev, tx);
            gen(p, Fct::Opr, 0, 15);
        }
        Token::ReadSym => {
            getsym(p);
            match p.sym.clone() {
                Token::Identifier(s) => {
                    let i = position(p, tx, &s);
                    if i == POS_NOT_FOUND {
                        parse_error(p.s.line, 11);
                    } else {
                        gen(p, Fct::Opr, 0, 14);
                        let v = p.table[i as usize].clone();
                        match v.kind {
                            ObjType::Variable => {
                                gen(p, Fct::Sto, lev - v.val_or_lev, v.adr);
                            }
                            _ => parse_error(p.s.line, 27),
                        }
                    }
                    getsym(p);
                }
                _ => parse_error(p.s.line, 26),
            }
        }
        _ => {}
    }
}

pub fn block(p: &mut Parser, lev: i32, mut tx: i32) {
    let mut dx: i32; //data allocation index
    let tx0: i32; //initial table index
    let cx0: i32; //Initial code index

    dx = 3;
    tx0 = tx;
    p.table[tx as usize].adr = p.cx;
    gen(p, Fct::Jmp, 0, 0);
    if lev > MAX_BLOCK_NESTING {
        parse_error(p.s.line, 32);
    }

    loop {
        if p.sym == Token::ConstSym {
            getsym(p);
            loop {
                constdeclaration(p, &mut tx);
                while p.sym == Token::Comma {
                    getsym(p);
                    constdeclaration(p, &mut tx);
                }

                if p.sym == Token::Semicolon {
                    getsym(p);
                } else {
                    parse_error(p.s.line, 5);
                }

                match &p.sym {
                    Token::Identifier(_n) => {}
                    _ => break,
                }
            }
        }

        if p.sym == Token::VarSym {
            getsym(p);
            loop {
                vardeclaration(p, lev, &mut tx, &mut dx);
                while p.sym == Token::Comma {
                    getsym(p);
                    vardeclaration(p, lev, &mut tx, &mut dx);
                }

                if p.sym == Token::Semicolon {
                    getsym(p);
                } else {
                    parse_error(p.s.line, 5);
                }

                match &p.sym {
                    Token::Identifier(_n) => {}
                    _ => break,
                }
            }
        }

        while p.sym == Token::ProcSym {
            getsym(p);

            match p.sym.clone() {
                Token::Identifier(n) => {
                    enter(p, &mut tx, n, ObjType::Procedure, lev, &mut dx);
                    getsym(p);
                }
                _ => parse_error(p.s.line, 4),
            }

            if p.sym == Token::Semicolon {
                getsym(p)
            } else {
                parse_error(p.s.line, 5);
            }

            block(p, lev + 1, tx);

            if p.sym == Token::Semicolon {
                getsym(p);
            } else {
                parse_error(p.s.line, 5);
            }
        }

        if !sym_in_declbegsys(&p.sym) {
            break;
        }
    }

    p.code[p.table[tx0 as usize].adr as usize].adr = p.cx;
    p.table[tx0 as usize].adr = p.cx;
    cx0 = 0;
    gen(p, Fct::Int, 0, dx);
    statement(p, lev, tx);
    gen(p, Fct::Opr, 0, 0); //return
    listcode(p, cx0, p.cx);
}

fn init_vecs(p: &mut Parser) {
    for _i in 0..CODE_ARR_SIZE {
        p.code.push(Instruction {
            fct: Fct::Lit,
            level: 0,
            adr: 0,
        });
    }
    for _i in 0..ID_TABLE_LEN {
        p.table.push(ObjDesc {
            name: "".to_owned(),
            kind: ObjType::Constant,
            val_or_lev: 0,
            adr: 0,
        })
    }
}

pub fn parser_run(mut p: Parser) -> Parser {
    init_vecs(&mut p);
    getsym(&mut p);
    block(&mut p, 0, 0);

    if p.sym != Token::Period {
        println!("End of parser run: {:?}", p.sym);
        parse_error(p.s.line, 9);
    }
    p
}
