/// static variable memory addresses start here
pub const DATA_ADDRESS_OFFSET: u32 = 0x10000000;

/// instruction memory addresses start here
pub const TEXT_ADDRESS_OFFSET: u32 = 0x00000000;

/// handles printing out files
pub mod io;

mod types;
pub use types::*;

/// handles parsing and lexing the program listing
pub mod parser;
pub use parser::ParserError;

/// handles linking the various .data and .text segments together
pub mod linker;
pub use linker::LinkerError;

/// handles converting the abstract program representation to binary data
pub mod printer;

#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    ParserError(ParserError),
    LinkerError(LinkerError),
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ParserError(e) => write!(f, "Error: {e}"),
            Self::LinkerError(e) => write!(f, "Error: {e}"),
        }
    }
}

/// main function of the library - takes in a program listing, outputs a binary
///
/// The program listing should be all the files of the program concatenated together,
/// starting with the main function.
///
/// # Panics:
///
/// Should only panic if there is a bug.
pub fn assemble(listing: &str, offset: u32) -> Result<String, AssemblerError> {
    let mut tree = match parser::parse(listing) {
        Ok(tree) => tree,
        Err(e) => return Err(AssemblerError::ParserError(e)),
    };

    tree = match linker::link(tree) {
        Ok(tree) => tree,
        Err(e) => return Err(AssemblerError::LinkerError(e)),
    };

    let binary = printer::print_binary(tree);

    let hex = printer::print_hex(binary, offset);

    Ok(hex)
}

#[cfg(test)]
mod tests {
    use parser::Loc;
    use std::vec;

    use crate::*;

    #[test]
    fn integration_test_1() {
        let text = "
.data
    constant: .word 125
    buffer: .space 8
    display: .addr 0xffff0000

.text

main:
    la $t1, display
    la $t2, constant
    la $s1, buffer

    addi $t0, $zero, 7 #initialize t0
    loop_start:
        beq $t0, $zero, loop_end
            addi $t0, $t0, -1
            sw $t0, $t1
        beq $zero, $zero, loop_start
    loop_end:
    add $zero, $zero, $zero

    push $t0
    pop $t0

    ja $s0, loop_start
        ";
        let binary = ProgramBinary {
            data: vec![125, 0, 0, 0, 0, 0, 0, 0, 0],
            instructions: vec![
                0xdd00_1000,
                0xdf00_8000,
                0xd100_0000,
                0xc110_0014,
                0x7e10_0000,
                0xd100_ffff,
                0xca10_0000,
                0xd100_1000,
                0xcb10_0000,
                0xd100_1000,
                0xc710_0004,
                0xc900_0007,
                0x8090_0010,
                0xc990_ffff,
                0xf09a_0000,
                0x8000_fff4,
                0x4000_0000,
                0xcff0_fffc,
                0xf09f_0000,
                0xe90f_0000,
                0xcff0_0004,
                0xd100_0000,
                0xc110_0030,
                0x7610_0000,
            ],
        };

        assert_eq!(
            assemble(text, 0x400).unwrap(),
            printer::print_hex(binary, 0x400)
        );
    }

    #[test]
    fn integration_test_errors() {
        assert_eq!(
            assemble("", 0x400),
            Err(AssemblerError::ParserError(ParserError::Empty))
        );

        assert_eq!(
            assemble("&", 0x400),
            Err(AssemblerError::ParserError(ParserError::InvalidToken(
                Loc { row: 0, col: 0 }
            )))
        );

        assert_eq!(
            assemble(".text\naddi $t1, 0", 0x400),
            Err(AssemblerError::ParserError(ParserError::Incomplete(Loc {
                row: 1,
                col: 10
            })))
        );

        assert_eq!(
            assemble(".text main:\nla $t1, id", 0x400),
            Err(AssemblerError::LinkerError(LinkerError::UnknownIdentifier(
                "id".to_string()
            )))
        );

        assert_eq!(
            assemble("la $t1, id", 0x400),
            Err(AssemblerError::ParserError(
                ParserError::CodeOutsideSegment(Loc { row: 0, col: 0 })
            ))
        );

        assert_eq!(
            assemble(".text add $t0, $zero, $zero", 0x400),
            Err(AssemblerError::LinkerError(LinkerError::UnknownIdentifier(
                "main".to_string()
            )))
        )
    }
}
