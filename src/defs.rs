pub const EX_NOINPUT: i32 = 66;
pub const EX_USAGE: i32 = 64;
pub const EX_DATAERR: i32 = 65;

pub const ID_TABLE_LEN: i32 = 100; //Length of identifier table
pub const ADDR_MAX: i32 = 2047; //Maximum address
pub const MAX_BLOCK_NESTING: i32 = 3; //Maximum depth of block nesting.
pub const CODE_ARR_SIZE: i32 = 2047; //Size of code array
pub const NUM_ERR_MSGS: i32 = 33;
pub const ERR_MSGS: [&str; NUM_ERR_MSGS as usize] = [
    "", //empty to accommodate same numbers as pascal implementation
    //1
    "Use = instead of :=",
    "= must be followed by a number",
    "Identifier must be followed by =",
    "const, var, procedure must be followed by an identifier",
    "Semicolon or comma missing",
    //6
    "Incorrect symbol after procedure declaration",
    "Statement expected",
    "Incorrect symbol after statement part in block",
    "Period expected",
    "Semicolon between statements is missing",
    //11
    "Undeclared identifier",
    "Assignment to constant or procedure is not allowed",
    "Assignment operator := expected",
    "Call must be followed by an identifier",
    "Call of a constant or a variable is meaningless",
    //16
    "then expected",
    "Semicolon or end expected",
    "do expected",
    "Incorrect symbol following statement",
    "Relational operator expected",
    //21
    "Expression must not contain a procedure identifier",
    "Right parenthesis missing",
    "The preceding factor cannot be followed by this symbol",
    "An expression cannot begin with this symbol",
    "",
    //26
    "A read must be followed by an identifier",
    "A read to constant or procedure is meaningless",
    "Unknown relational operator",
    "",
    "This number is too large",
    //31
    "",
    "Block nesting too deep",
];

//Lit 0, a: Load constant a
//Opr 0, a: Execute operation a
//Lod l, a: Load variable l, a (l=level, a=address)
//Sto l, a: Store variable l, a (l=level, a=address)
//Cal l, a: Call procedure a at level l
//Int 0, a: Increment t (top of stack) register by a
//Jmp 0, a: Jump to a
//Jpc 0, a: Jump conditional to a
#[derive(Debug, Copy, Clone)]
pub enum Fct {
    Lit,
    Opr,
    Lod,
    Sto,
    Cal,
    Int,
    Jmp,
    Jpc,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub fct: Fct,   //Function code
    pub level: i32, //Nesting level
    pub adr: i32,   //0..ADDR_MAX
}

pub const NUM_INSTRUCTIONS: usize = 8;
pub const MNEMONICS: [&str; NUM_INSTRUCTIONS] =
    ["lit", "opr", "lod", "sto", "cal", "int", "jmp", "jpc"];
