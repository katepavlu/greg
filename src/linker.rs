use crate::types::*;
use crate::ProgramTree;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum LinkerError {
    UnknownIdentifier(String),
    RedefinedIdentifier(String),
}

impl std::fmt::Display for LinkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownIdentifier( id) if *id == *"main" => write!(f, "Main function not found"),
            Self::UnknownIdentifier( id) => write!(f, "Identifier not recognized: \"{id}\""),
            Self::RedefinedIdentifier(id) => {
                write!(f, "Identifier defined more than once: \"{id}\"")
            }
        }
    }
}

/// links all identifiers in the program together,
/// converting them to immediates of their instructions
pub fn link(mut tree: ProgramTree) -> Result<ProgramTree, LinkerError> {
    // linking is performed using a hash map
    let mut map = HashMap::new();

    // if an identifier definition is encountered, it is added to the map
    for datanode in &tree.data {
        if datanode.identifier != *"" {
            // return error if the key was already present
            match map.insert(datanode.identifier.clone(), datanode.address) {
                None => (),
                Some(_) => {
                    return Err(LinkerError::RedefinedIdentifier(
                        datanode.identifier.clone(),
                    ))
                }
            }
        }
    }
    for instrnode in &tree.instructions {
        if instrnode.identifier != *"" {
            match map.insert(instrnode.identifier.clone(), instrnode.address) {
                None => (),
                Some(_) => {
                    return Err(LinkerError::RedefinedIdentifier(
                        instrnode.identifier.clone(),
                    ))
                }
            }
        }
    }

    // then, for each located use of an identifier, the map is checked for valid definitions.
    // If one is not found, the function returns err
    for instrnode in &mut tree.instructions {
        // skip everything that does not use an identifier
        if instrnode.imm_identifier != *"" {
            // read out the address the identifier was defined for
            let target_address = match map.get(&instrnode.imm_identifier) {
                Some(n) => n.to_owned(),
                None => {
                    return Err(LinkerError::UnknownIdentifier(
                        instrnode.imm_identifier.clone(),
                    ))
                }
            };

            match instrnode.op {
                Instr::Beq | Instr::Bne => {
                    // beq, bne require an offset if they have an identifier
                    instrnode.imm = target_address as i64 - instrnode.address as i64;
                }
                Instr::Addi => {
                    // addi adds the bottom half of the address only
                    instrnode.imm = (target_address & 0xffff) as i64;
                }
                Instr::Lui => {
                    // lui adds the top half of the identifier only
                    instrnode.imm = ((target_address & 0xffff0000) >> 16) as i64;
                }
                _ => (),
            }
        }
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests if linking is performed correctly
    #[test]
    fn linktest() {
        let tree_in = ProgramTree {
            data: vec![DataNode {
                identifier: "number".to_string(),
                address: 0x1000_0004,
                block: Bl::Word,
                data: 5,
                num: 1,
            }],
            instructions: vec![
                InstructionNode {
                    op: Instr::Lui,
                    rd: 1,
                    ra: 0,
                    rb: 0,
                    imm: 0,
                    identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(),
                    address: 0,
                },
                InstructionNode {
                    op: Instr::Addi,
                    rd: 1,
                    ra: 0,
                    rb: 0,
                    imm: 0,
                    identifier: "".to_string(),
                    imm_identifier: "number".to_string(),
                    address: 4,
                },
                InstructionNode {
                    op: Instr::Beq,
                    rd: 0,
                    ra: 1,
                    rb: 0,
                    imm: 0,
                    identifier: "".to_string(),
                    imm_identifier: "beginning".to_string(),
                    address: 8,
                },
            ],
        };

        let tree_out = ProgramTree {
            data: vec![DataNode {
                identifier: "number".to_string(),
                address: 0x1000_0004,
                block: Bl::Word,
                data: 5,
                num: 1,
            }],
            instructions: vec![
                InstructionNode {
                    op: Instr::Lui,
                    rd: 1,
                    ra: 0,
                    rb: 0,
                    imm: 0x1000,
                    identifier: "beginning".to_string(),
                    imm_identifier: "number".to_string(),
                    address: 0,
                },
                InstructionNode {
                    op: Instr::Addi,
                    rd: 1,
                    ra: 0,
                    rb: 0,
                    imm: 0x0004,
                    identifier: "".to_string(),
                    imm_identifier: "number".to_string(),
                    address: 4,
                },
                InstructionNode {
                    op: Instr::Beq,
                    rd: 0,
                    ra: 1,
                    rb: 0,
                    imm: -8,
                    identifier: "".to_string(),
                    imm_identifier: "beginning".to_string(),
                    address: 8,
                },
            ],
        };

        let tree_linked = link(tree_in);

        assert_eq!(tree_linked.unwrap(), tree_out);
    }

    // tsts if the appropriate error is returned
    #[test]
    fn linkerror() {
        let tree = ProgramTree {
            data: vec![DataNode {
                identifier: "number1".to_string(),
                address: 0x1000_0004,
                block: Bl::Word,
                data: 5,
                num: 1,
            }],
            instructions: vec![InstructionNode {
                op: Instr::Lui,
                rd: 1,
                ra: 0,
                rb: 0,
                imm: 0x1000,
                identifier: "".to_string(),
                imm_identifier: "number2".to_string(),
                address: 0,
            }],
        };

        assert_eq!(
            link(tree),
            Err(LinkerError::UnknownIdentifier("number2".to_string()))
        )
    }
}
