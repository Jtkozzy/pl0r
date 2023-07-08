use crate::defs::{Fct, Instruction};
use crate::parser::Parser;
use std::io;

// lit 0,a : load constant a
// opr 0,a : execute operation a
// lod l,a : load varible l,a
// sto l,a : store varible l,a
// cal l,a : call procedure a at level l
// int 0,a : increment t-register by a
// jmp 0,a : jump to a
// jpc 0,a : jump conditional to a

const STACK_SIZE: usize = 501;

//Find base l levels down
fn base(mut l: i32, b: i32, s: &[i32; STACK_SIZE]) -> i32 {
    let mut bl: i32;
    bl = b;
    while l > 0 {
        bl = s[bl as usize];
        l = l - 1;
    }
    bl
}

fn read_i32() -> i32 {
    let mut input = String::new();
    let read_res = io::stdin().read_line(&mut input);
    if read_res.is_err() {
        return 0;
    }
    let ret = input.trim().parse::<i32>();
    if ret.is_err() {
        0
    } else {
        ret.unwrap()
    }
}

pub fn interpret(par: Parser) {
    let mut p: i32; //Program register
    let mut b: i32; //Baseregister
    let mut t: usize; //Topstack register
    let mut i: Instruction;
    let mut s: [i32; STACK_SIZE] = [0; STACK_SIZE];

    println!(" start pl/0");
    t = 0;
    b = 1;
    p = 0;
    s[1] = 0;
    s[2] = 0;
    s[3] = 0;
    loop {
        i = par.code[p as usize];
        p += 1;
        match i.fct {
            Fct::Lit => {
                t += 1;
                s[t] = i.adr;
            }
            Fct::Opr => match i.adr {
                0 => {
                    //return
                    t = (b - 1) as usize;
                    p = s[t + 3];
                    b = s[t + 2]
                }
                1 => {
                    s[t] = -s[t];
                }
                2 => {
                    t = t - 1;
                    s[t] = s[t] + s[t + 1];
                }
                3 => {
                    t = t - 1;
                    s[t] = s[t] - s[t + 1];
                }
                4 => {
                    t = t - 1;
                    s[t] = s[t] * s[t + 1];
                }
                5 => {
                    t = t - 1;
                    s[t] = s[t] / s[t + 1];
                }
                6 => {
                    s[t] = s[t] % 2;
                }
                8 => {
                    t = t - 1;
                    s[t] = if s[t] == s[t + 1] { 1 } else { 0 };
                }
                9 => {
                    t = t - 1;
                    s[t] = if s[t] != s[t + 1] { 1 } else { 0 };
                }
                10 => {
                    t = t - 1;
                    s[t] = if s[t] < s[t + 1] { 1 } else { 0 };
                }
                11 => {
                    t = t - 1;
                    s[t] = if s[t] >= s[t + 1] { 1 } else { 0 };
                }
                12 => {
                    t = t - 1;
                    s[t] = if s[t] > s[t + 1] { 1 } else { 0 };
                }
                13 => {
                    t = t - 1;
                    s[t] = if s[t] <= s[t + 1] { 1 } else { 0 };
                }
                14 => {
                    t = t + 1;
                    s[t] = read_i32();
                }
                15 => {
                    println!("{}", s[t]);
                }
                _ => {}
            },
            Fct::Lod => {
                t += 1;
                let ind = base(i.level, b, &s) + i.adr;
                s[t] = s[ind as usize];
            }
            Fct::Sto => {
                let ind = base(i.level, b, &s) + i.adr;
                s[ind as usize] = s[t];
                t = t - 1;
            }
            Fct::Cal => {
                s[t + 1] = base(i.level, b, &s);
                s[t + 2] = b;
                s[t + 3] = p;
                b = (t + 1) as i32;
                p = i.adr;
            }
            Fct::Int => {
                t = t + i.adr as usize;
            }
            Fct::Jmp => {
                p = i.adr;
            }
            Fct::Jpc => {
                if s[t] == 0 {
                    p = i.adr
                }
                t = t - 1;
            }
        }
        if p == 0 {
            break;
        }
    }
    println!(" end pl/0");
}
