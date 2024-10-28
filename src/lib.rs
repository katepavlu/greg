use core::fmt;
use std::{any::Any, collections::HashMap, convert::Infallible};
use lexgen_util::LexerError;

pub mod mylexer;
pub mod io;
mod types;
pub use types::*;

pub const DATA_ADDRESS_OFFSET:u32 = 0x10000000;
pub const TEXT_ADDRESS_OFFSET:u32 = 0x00000000;

#[derive(Debug)]
struct Loc{
    row:u32,
    col:u32
}

#[derive(Debug)]
pub enum ParserError{
    CodeOutsideSegment,
    InvalidToken(Loc),
    Incomplete(Loc),
    End,
    Empty,
}
    

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CodeOutsideSegment => write!(f, "File must start with segment anotation!"),
            Self::InvalidToken(loc) => write!(f, "Invalid token: line {} column {}!", loc.row, loc.col),
            Self::Incomplete(loc) => write!(f, "Incomplete instruction/assignment: line {} column {}!", loc.row, loc.col),
            Self::End => write!(f, "End of input reached!"), 
            Self::Empty => write!(f, "No valid tokens found"),          
        }
    }
}

struct ParserState {
    segment: Bl,
}

fn parse(input_buffer: &str) -> Result<ProgramTree, ParserError> {
    // create a program tree structure to output
    let mut tree = ProgramTree{
        instructions: Vec::new(),
        data: Vec::new(),
        id_map: HashMap::new(),
    };

    let mut state = ParserState{segment: Bl::Data};    

    // create a lexer iterator to recognize tokens in the input
    let mut lexer
        = mylexer::Lexer::new(&input_buffer); 

    // the first segment annotation has to be treated separately
    state.segment = match lexer.next() {
        None => {
            return Err(ParserError::Empty
    );
        },
        Some(Ok( (_, Token::Block(Bl::Data), _) )) => Bl::Data,
        Some(Ok( (_, Token::Block(Bl::Text), _) )) => Bl::Text,
        Some(_) => return Err(ParserError::CodeOutsideSegment),
    };
    
    loop{


        
    }//let (loc, token)

    Ok(tree)
}

type LexerResult = Result<(lexgen_util::Loc, Token, lexgen_util::Loc), LexerError<Infallible>>;
type ParserResult = Result<(Loc, Token), ParserError>;


/// # Parse single token
/// 
/// converts lexer error types to parser errors 
/// 
/// converts lexer output types to parser input types
fn parse_token(ret: LexerResult  ) -> ParserResult {
    match ret {
        Ok((l, t, _)) => Ok((Loc{row: l.line, col: l.col},t)),
        Err(LexerError{location:l, kind: _}) => Err(ParserError::InvalidToken(Loc{row: l.line, col:l.col}))
    }
}

/// # Read single token
/// 
/// wraps parse_token 
/// 
/// returns ParserError::IncompleteInstruction(Loc) if EOF is encountered
fn read_token(ret: Option<LexerResult>) -> ParserResult {
    let ret = match ret {
        Some(x) => x,
        None => return Err(ParserError::End),
    };

    parse_token(ret) 
}

/// # Select token
/// 
/// returns ParseError::IncompleteInstruction(Loc) if the next token is not the selected token
fn sel_token(ret: Option<LexerResult>, selection: Token) -> ParserResult {
    match read_token(ret)? {
        (loc, token) if token.type_id() == selection.type_id() => Ok((loc,token)),
        (loc, _) => Err(ParserError::Incomplete(loc)),
    }
}

/// # Get register
/// 
/// uses sel_token to read a register number. Forwards errors
fn get_register(ret: Option<LexerResult>) -> Result<u8, ParserError> {
    match sel_token(ret, Token::Register(0))? {
        (_, Token::Register(x)) => Ok(x),
        _ => panic!("getting register failed") // this should never happen
    }
}

/// # Get immediate
/// 
/// uses sel_token to read an immediate. Forwards errors
fn get_immediate(ret: Option<LexerResult>) -> Result<i64, ParserError> {
    match sel_token(ret, Token::Immediate(0))? {
        (_, Token::Immediate(x)) => Ok(x),
        _ => panic!("getting immediate failed") // this should never happen
    }
}

/// # Get identifier or immediate
/// 
/// uses sel_token to read an immediate. Forwards errors
fn get_identifier_or_imm(ret: Option<LexerResult>)
     -> Result<(String, i64), ParserError> 
{
    match read_token(ret)? {
        (_, Token::Identifier(str)) => Ok((str, 0)),
        (_, Token::Immediate(x)) => Ok((String::new(), x)),
        (loc, _) => Err(ParserError::Incomplete(loc)),
    }
}

type Lexer<'a> = mylexer::Lexer_<'a, std::str::Chars<'a>, ()>;
fn get_instruction(
    instruction: Instr, identifier: String, lexer:&mut Lexer<'_>, address: &mut u32 )
     -> Result<Vec<InstructionNode>, ParserError> 
{
    let mut return_vector:Vec<InstructionNode> = Vec::new();
    
    let mut op = instruction;
    let mut rd = 0;
    let mut ra = 0;
    let mut rb = 0;
    let mut imm = 0;
    let mut identifier = identifier;
    let mut imm_identifier = String::new();
    
    match op {
        Instr::And | Instr::Or | Instr::Xor | Instr::Add | Instr::Sub | Instr::Cmp => {

            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            ra = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            rb = get_register(lexer.next())?;


        },
        Instr::Not => (),

        Instr::J => (),
        Instr::Beq => (),
        Instr::Bne => (),

        Instr::Sl => (),
        Instr::Sr => (),

        Instr::Addi => (),
        Instr::Lui => (),

        Instr::Lw => (),
        Instr::Sw => (),

        Instr::La => (),
        Instr::Ja => (),
    };

    return_vector.push(
        InstructionNode{
            op, rd, ra, rb, imm, identifier, imm_identifier, address: *address
        }
    );
    *address += 4;

    Ok(return_vector)
}