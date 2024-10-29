/// # Token types
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

/// # Block annotation types
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

/// # Abstract instruction representation
#[derive(Debug, PartialEq, Eq, Clone)]
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

/// # abstract data block representation
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataNode {
    pub identifier: String,
    pub address: u32,
    pub block: Bl,
    pub data: i64,
    pub num: u32,
}

/// # Abstract representation of the whole program
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProgramTree {
    pub instructions: Vec<InstructionNode>,
    pub data: Vec<DataNode>,
}

/// # Compiled binary
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProgramBinary {
    pub instructions: Vec<u32>,
    pub data: Vec<u32>,
}