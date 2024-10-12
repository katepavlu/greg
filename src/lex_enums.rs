use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"\s+")] // Ignore this regex pattern between tokens
pub enum TokenType {

    #[token("and")]
    #[token("or")]
    #[token("xor")]
    #[token("not")]
    #[token("add")]
    #[token("sub")]
    #[token("cmp")]
    #[token("j")]
    #[token("beq")]
    #[token("bne")]
    #[token("sl")]
    #[token("sr")]
    #[token("addi")]
    #[token("lui")]
    #[token("lw")]
    #[token("sw")]
    Instruction,

    #[token(",")]
    Comma,

    #[regex(r"\$\w+")]
    Register,

    #[regex(r"\w+:")]
    Identifier,

    #[token("//")]
    InlineCommentStart,

    #[token("/*")]
    CommentStart,

    #[token("*/")]
    CommentEnd,

    
}

#[derive(Logos, Debug, PartialEq)]
pub enum InstructionType {

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("xor")]
    Xor,

    #[token("not")]
    Not,

    #[token("add")]
    Add,

    #[token("sub")]
    Sub,

    #[token("cmp")]
    Cmp,

    #[token("j")]
    J,

    #[token("beq")]
    Beq,

    #[token("bne")]
    Bne,

    #[token("sl")]
    Sl,

    #[token("sr")]
    Sr,

    #[token("addi")]
    Addi,

    #[token("lui")]
    Lui,

    #[token("lw")]
    Lw,

    #[token("sw")]
    Sw,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Register {
    
    #[token("$zero")]
    ZeroReg,
    
    #[token("at")]
    AssemblerReg,

    #[token("$v")]
    ReturnValReg,

    #[regex(r"\$a[0-2]")]
    ArgumentReg,

    #[regex(r"\$s[0-2]")]
    SavedReg,

    #[regex(r"\$t[0-3]")]
    TReg,

    #[token("gv")]
    GlobalReg,

    #[token("$ra")]
    ReturnAddressReg,

    #[token("sp")]
    StackPinterReg,

    #[regex(r"\$[0-9]+")]
    NumberReg,
}
