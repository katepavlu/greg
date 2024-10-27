use std::collections::HashMap;

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

pub struct InstructionNode {
    pub op: Instr,
    pub rd: u8,
    pub ra: u8,
    pub rb: u8,
    pub imm: i64,
    pub identifier: String,
    pub imm_identifier: String,
    pub address: u32,
}

pub struct DataNode {
    pub identifier: String,
    pub address: u32,
    pub block: Bl,
    pub data: i64,
}

pub struct ProgramTree {
    pub instructions: Vec<InstructionNode>,
    pub data: Vec<DataNode>,
    pub id_map: HashMap<String, u32>,
}