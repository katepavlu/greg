pub mod mylexer;
pub mod myio;

use std::{collections::HashMap, convert::Infallible};

use lexgen_util::{LexerError, LexerErrorKind, Loc};

/// # Token types
/// 
/// #TODO explain what Err does
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Comma,
    Colon,
    Instruction(Instr),
    Register(u8),
    Identifier(String),
    Immediate(i64),
    Block(Bl),
    Err,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bl {
    Data,
    Text,
    Addr,
    Space,
    Word,
}

/// # Instructions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    And, Or, Xor, Not,
    Add, Sub, Cmp,
    J, Beq, Bne,
    Sl, Sr,
    Addi, Lui,
    Lw, Sw,

    La, Ja,
}

struct InstructionNode {
    op: Instr,
    rd: u8,
    ra: u8,
    rb: u8,
    imm: i64,
    identifier: String,
    imm_identifier: String,
    address: u32,
}

struct DataNode {
    identifier: String,
    address: u32,
    block: Bl,
    data: i64,
}

struct ProgramTree {
    instructions: Vec<InstructionNode>,
    data: Vec<DataNode>,
    id_map: HashMap<String, u32>,
}

struct ParserState {
    segment: Bl,
}

fn parse(input_buffer: &str) -> ProgramTree {
    // create a lexer iterator to recognize tokens in the input
    let mut lexer
        = mylexer::Lexer::new(&input_buffer); 

    // create a program tree structure to output
    let mut tree = ProgramTree{
        instructions: Vec::new(),
        data: Vec::new(),
        id_map: HashMap::new(),
    };

    let mut state = ParserState{segment: Bl::Data};    

    // the first token has to be treated separately
    state.segment = match lexer.next() {
        None => panic!("Input file contains no valid input."),
        Some(Ok( (_, Token::Block(Bl::Data), _) )) => Bl::Data,
        Some(Ok( (_, Token::Block(Bl::Text), _) )) => Bl::Text,
        Some(_) => panic!("Input file must start with segment annotation."),
    };

    for ret in lexer {
        //let (loc, token)
    }

    tree
}


/// # Parse single token
/// 
/// # Panics
/// panics on LexerError, prints out location
fn parse_token(ret: Result<(Loc, Token, Loc), LexerError<Infallible>>) -> (Loc, Token) {
    match ret {
        Ok((l, t, _)) => (l,t),
        Err(LexerError{location:l, kind: _}) => {
            panic!("Could not parse: line {} column {}.", l.line, l.col)
        }
    }
}