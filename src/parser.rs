use core::fmt;
use lexgen_util::LexerError;
use std::convert::Infallible;

// helper functions for the parser
mod parserhelpers;
use parserhelpers::*;

// parser for one instruction node
mod parseinstruction;
use parseinstruction::*;

// parser for one data node
mod parsedata;
use parsedata::*;

// lexer for the pierogi assembler language
pub mod mylexer;

use crate::types::*;

use super::{DATA_ADDRESS_OFFSET, TEXT_ADDRESS_OFFSET};

type ParserResult = Result<(Loc, Token), ParserError>;

/// # Location
/// indicates where in the file a token/error was encountered
#[derive(Debug, PartialEq)]
pub struct Loc {
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    CodeOutsideSegment(Loc),
    InvalidToken(Loc),
    Incomplete(Loc),
    End,
    Empty,
    NegativeSpace(Loc),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CodeOutsideSegment(loc) => write!(
                f,
                "Code outside of segment annotation: line {} column {}",
                loc.row, loc.col
            ),
            Self::InvalidToken(loc) => {
                write!(f, "Invalid token: line {} column {}", loc.row, loc.col)
            }
            Self::Incomplete(loc) => {
                write!(f, "Invalid statement: line {} column {}", loc.row, loc.col)
            }
            Self::End => write!(f, "End of input reached prematurely"),
            Self::Empty => write!(f, "No valid tokens found"),
            Self::NegativeSpace(loc) => write!(
                f,
                "Number cannot be negative: line {} column {}",
                loc.row, loc.col
            ),
        }
    }
}

/// parse a program listing, creating an abstract program tree
/// with two branches (data and instructions).
pub fn parse(input_buffer: &str) -> Result<ProgramTree, ParserError> {
    // create a program tree structure to output
    let mut tree = ProgramTree {
        instructions: Vec::new(),
        data: Vec::new(),
    };

    let mut current_segment;

    // create a lexer iterator to recognize tokens in the input
    let mut lexer = mylexer::Lexer::new(input_buffer);

    // the first segment annotation has to be treated separately
    current_segment = match lexer.next() {
        None => return Err(ParserError::Empty),
        Some(Ok((_, Token::Block(Bl::Data), _))) => Bl::Data,
        Some(Ok((_, Token::Block(Bl::Text), _))) => Bl::Text,
        Some(Ok((l, _, _))) => {
            return Err(ParserError::CodeOutsideSegment(Loc {
                row: l.line,
                col: l.col,
            }))
        }
        Some(Err(LexerError {
            location: l,
            kind: _,
        })) => {
            return Err(ParserError::InvalidToken(Loc {
                row: l.line,
                col: l.col,
            }))
        }
    };

    let mut data_address = DATA_ADDRESS_OFFSET;
    let mut instr_address = TEXT_ADDRESS_OFFSET;
    let mut identifier;

    // loop over the input until your reach an error or its end
    'outer: loop {
        let token = read_token(lexer.next());

        // exit loop on end of stream
        let (mut loc, mut token) = match token {
            Err(ParserError::End) => break 'outer,
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        // read identifier if present
        if let Token::Identifier(str) = token {
            identifier = str;
            sel_token(lexer.next(), Token::Colon)?;

            (loc, token) = read_token(lexer.next())?;
        } else {
            identifier = String::new();
        }

        match current_segment {
            Bl::Data => {
                match token {
                    Token::Block(Bl::Text) => current_segment = Bl::Text,
                    Token::Block(Bl::Data) => continue,
                    Token::Block(b) => {
                        // if inside data segment: try to parse one data line
                        let node = parse_data(b, identifier, &mut lexer, &mut data_address)?;
                        tree.data.push(node);
                    }
                    _ => return Err(ParserError::Incomplete(loc)),
                }
            }
            Bl::Text => {
                match token {
                    Token::Block(Bl::Data) => current_segment = Bl::Data,
                    Token::Block(Bl::Text) => continue,
                    Token::Instruction(i) => {
                        // if inside text segment: try to parse one instruction
                        let mut nodes =
                            parse_instruction(i, identifier, &mut lexer, &mut instr_address)?;
                        tree.instructions.append(&mut nodes);
                    }
                    _ => return Err(ParserError::Incomplete(loc)),
                }
            }
            _ => return Err(ParserError::CodeOutsideSegment(loc)),
        }
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsetest() {
        let input = "
        .data
            buffer: .space 4
            number: .word 5
        .text
            beginning:
            add $1, $zero, $zero
            beq $1, $zero, beginning";

        let tree = parse(input).unwrap();

        assert_eq!(
            tree,
            ProgramTree {
                data: vec![
                    DataNode {
                        identifier: "buffer".to_string(),
                        address: DATA_ADDRESS_OFFSET,
                        block: Bl::Space,
                        data: 0,
                        num: 4,
                    },
                    DataNode {
                        identifier: "number".to_string(),
                        address: DATA_ADDRESS_OFFSET + 16,
                        block: Bl::Word,
                        data: 5,
                        num: 1,
                    }
                ],
                instructions: vec![
                    InstructionNode {
                        op: Instr::Add,
                        rd: 1,
                        ra: 0,
                        rb: 0,
                        imm: 0,
                        identifier: "beginning".to_string(),
                        imm_identifier: "".to_string(),
                        address: TEXT_ADDRESS_OFFSET,
                    },
                    InstructionNode {
                        op: Instr::Beq,
                        rd: 0,
                        ra: 1,
                        rb: 0,
                        imm: 0,
                        identifier: "".to_string(),
                        imm_identifier: "beginning".to_string(),
                        address: TEXT_ADDRESS_OFFSET + 4,
                    },
                ],
            }
        );
    }
}
