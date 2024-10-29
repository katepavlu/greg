use crate::ProgramTree;
use std::collections::HashMap;
use crate::types::*;

#[derive(Debug, PartialEq)]
pub enum LinkerError{
    UnknownIdentifier(String)
}

impl std::fmt::Display for LinkerError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownIdentifier(id) => write!(f, "Identifier not recognized: {id}"),      
        }
    }
}


pub fn link(mut tree:ProgramTree) -> Result<ProgramTree, LinkerError> {

    let mut map = HashMap::new();

    for datanode in &tree.data {
        if datanode.identifier != "".to_string() {
            map.insert(datanode.identifier.clone(), datanode.address);
        }
    }

    for instrnode in &tree.instructions {
        if instrnode.identifier != "".to_string() {
            map.insert(instrnode.identifier.clone(), instrnode.address);
        }
    }

    for instrnode in &mut tree.instructions {

        if instrnode.imm_identifier != "".to_string() {

            let target_address = 
            match map.get(&instrnode.imm_identifier) {
                Some(n)=> n.to_owned(),
                None => return Err(LinkerError::UnknownIdentifier(instrnode.imm_identifier.clone())),
            };
    
            match instrnode.op {
                Instr::Beq|Instr::Bne => { // beq, bne require an offset if they have an identifier
                    instrnode.imm = target_address as i64 - instrnode.address as i64;
                },
                Instr::Addi => {
                    instrnode.imm = (target_address & 0xffff) as i64;
                }
                Instr::Lui => {
                    instrnode.imm = ((target_address & 0xffff0000) >> 16 ) as i64;
                }
                _ => (),
            }

        }
    }

    Ok(tree)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn linktest() {

        let tree_in = ProgramTree{
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
                    imm:0, identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(), address: 0,
                },
                InstructionNode{
                    op: Instr::Addi,
                    rd: 1, ra: 0, rb: 0,
                    imm:0, identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(), address: 4,
                },
                InstructionNode{
                    op: Instr::Beq, 
                    rd:0, ra: 1, rb: 0,
                    imm:0, identifier: "".to_string(),
                    imm_identifier: "beginning".to_string(), address: 8,
                },
            ],
        };

        let tree_out = ProgramTree{
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
                    rd: 1, ra: 0, rb: 0,
                    imm:0x0004, identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(), address: 4,
                },
                InstructionNode{
                    op: Instr::Beq, 
                    rd:0, ra: 1, rb: 0,
                    imm:-4, identifier: "".to_string(),
                    imm_identifier: "beginning".to_string(), address: 8,
                },
            ],
        };

        let tree_linked = link(tree_in);

        assert_eq!(tree_linked.unwrap(),tree_out);
    }


    #[test]
    fn linkerror() {
        let tree = ProgramTree{
            data: vec![
                DataNode{
                    identifier: "number1".to_string(),
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
                    imm: 0x1000, identifier: "".to_string(),
                    imm_identifier: "number2".to_string(), address: 0,
                }],
        };

        assert_eq!(link(tree), Err(LinkerError::UnknownIdentifier("number2".to_string())))
    }

}
