use core::fmt;
use std::{collections::HashMap, convert::Infallible};
use lexgen_util::LexerError;

pub mod mylexer;
pub mod io;
mod types;
pub use types::*;

mod parsehelpers;
mod parseinstruction;
mod parsedata;

type ParserResult = Result<(Loc, Token), ParserError>;

pub const DATA_ADDRESS_OFFSET:u32 = 0x10000000;
pub const TEXT_ADDRESS_OFFSET:u32 = 0x00000000;

#[derive(Debug, PartialEq)]
pub struct Loc{
    row:u32,
    col:u32
}

#[derive(Debug, PartialEq)]
pub enum ParserError{
    CodeOutsideSegment,
    InvalidToken(Loc),
    Incomplete(Loc),
    End,
    Empty,
    NegativeSpace(Loc),
}
    

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CodeOutsideSegment => write!(f, "File must start with segment anotation!"),
            Self::InvalidToken(loc) => write!(f, "Invalid token: line {} column {}!", loc.row, loc.col),
            Self::Incomplete(loc) => write!(f, "Incomplete instruction/assignment: line {} column {}!", loc.row, loc.col),
            Self::End => write!(f, "End of input reached!"), 
            Self::Empty => write!(f, "No valid tokens found!"),
            Self::NegativeSpace(loc) => write!(f, "Number cannot be negative: line {} column {}!", loc.row, loc.col),       
        }
    }
}

fn parse(input_buffer: &str) -> Result<ProgramTree, ParserError> {
    // create a program tree structure to output
    let mut tree = ProgramTree{
        instructions: Vec::new(),
        data: Vec::new(),
        id_map: HashMap::new(),
    };

    let mut current_segment = Bl::Data;    

    // create a lexer iterator to recognize tokens in the input
    let mut lexer
        = mylexer::Lexer::new(&input_buffer); 

    // the first segment annotation has to be treated separately
    current_segment = match lexer.next() {
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
