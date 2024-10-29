use crate::ProgramTree;
use crate::types::*;

#[derive(Debug, PartialEq)]
enum PrinterError{
    PseudoinstructionNotHandled(Instr),
    InvalidBlock(Bl),
}

impl std::fmt::Display for PrinterError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PseudoinstructionNotHandled(instr) => 
                write!(f, "Pseudoinstruction not handled: {:?}. This is a parser bug", instr), 
            Self::InvalidBlock(bl) => 
                write!(f, "Invalid block: {:?}. This is a parser bug", bl),      
        }
    }
}

fn print_binary(tree: ProgramTree) -> Result<ProgramBinary, PrinterError> {

    let mut binary = ProgramBinary{
        data: Vec::new(),
        instructions: Vec::new(),
    };

    for datanode in tree.data {
        match datanode.block{
            Bl::Addr => (),
            Bl::Word => binary.data.push(datanode.data as u32),
            Bl::Space => {
                for _i in 0..datanode.num {
                    binary.data.push(0);
                }
            }
            b => return Err(PrinterError::InvalidBlock(b)),
        }
    }

    for instrnode in tree.instructions {

        // assemble instruction
        let mut instruction = 0;
        instruction |= (match instrnode.op { // add opcode
            Instr::And => 0b0000,
            Instr::Or  => 0b0001,
            Instr::Xor => 0b0010,
            Instr::Not => 0b0011,

            Instr::Add => 0b0100,
            Instr::Sub => 0b0101,
            Instr::Cmp => 0b0110,

            Instr::J   => 0b0111,
            Instr::Beq => 0b1000,
            Instr::Bne => 0b1001,

            Instr::Sl  => 0b1010,
            Instr::Sr  => 0b1011,
            Instr::Addi=> 0b1100,
            Instr::Lui => 0b1101,

            Instr::Lw =>  0b1110,
            Instr::Sw =>  0b1111,
            i => return Err(PrinterError::PseudoinstructionNotHandled(i)),
        }) << 28;

        instruction |= ((instrnode.rd as u32) & 0b1111) << 24; // add Rd
        instruction |= ((instrnode.ra as u32) & 0b1111) << 20; // add Ra
        instruction |= ((instrnode.rb as u32) & 0b1111) << 16; // add Rb

        instruction |= (instrnode.imm as u32) & 0xFFFF; // add immediate

        binary.instructions.push(instruction);
    }

    Ok(binary)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn printer_test() {
        let tree = ProgramTree{
            data: vec![
                DataNode{
                    identifier: "number".to_string(),
                    address: 0x1000_0004,
                    block: Bl::Word,
                    data: 5,
                    num: 1,
                }
            ],
            instructions: vec![
                InstructionNode{
                    op: Instr::Lui, 
                    rd: 1, ra: 0, rb: 0,
                    imm: 0x1000, identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(), address: 0,
                },
                InstructionNode{
                    op: Instr::Addi,
                    rd: 1, ra: 5, rb: 0,
                    imm:0x0004, identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(), address: 4,
                },
                InstructionNode{
                    op: Instr::Beq, 
                    rd:0, ra: 1, rb: 6,
                    imm:-4, identifier: "".to_string(),
                    imm_identifier: "beginning".to_string(), address: 8,
                },
            ],
        };

        let bin = ProgramBinary{
            data: vec![5],
            instructions: vec![
                0xD100_1000,
                0xC150_0004,
                0x8016_FFFC
            ]
        };

        assert_eq!(print_binary(tree).unwrap(), bin);

    }
}