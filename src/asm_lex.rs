use lexgen::lexer;

/* Token enum: top level distinctions */
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Comma,
    Colon,
    Instruction(Instr),
    Register(u8),
    Identifier(String),
    Immediate(i64),
    Block(Bl),
    Err
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bl {
    Data,
    Text,
    Addr,
    Space,
    Word,
}

/* Instr enum: types of instructions */
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    And,
    Or,
    Xor,
    Not,
    Add,
    Sub,
    Cmp,
    J,
    Beq,
    Bne,
    Sl,
    Sr,
    Addi,
    Lui,
    Lw,
    Sw,

    La,
    Ja,
}

lexer! {
    pub Lexer -> Token;

    let whitespace = [' ' '\t' '\n'] | "\r\n";

    rule Init {
        $whitespace, //skip whitespace

        ','   = Token::Comma,
        ':'   = Token::Colon,

        '#' => |lexer| lexer.switch(LexerRule::Comment),

        "and" = Token::Instruction(Instr::And),
        "or"  = Token::Instruction(Instr::Or),
        "xor" = Token::Instruction(Instr::Xor),
        "not" = Token::Instruction(Instr::Not),
        "add" = Token::Instruction(Instr::Add),
        "sub" = Token::Instruction(Instr::Sub),
        "cmp" = Token::Instruction(Instr::Cmp),
        "j"   = Token::Instruction(Instr::J),
        "beq" = Token::Instruction(Instr::Beq),
        "bne" = Token::Instruction(Instr::Bne),
        "sl"  = Token::Instruction(Instr::Sl),
        "sr"  = Token::Instruction(Instr::Sr),
        "addi"= Token::Instruction(Instr::Addi),
        "lui" = Token::Instruction(Instr::Lui),
        "lw"  = Token::Instruction(Instr::Lw),
        "sw"  = Token::Instruction(Instr::Sw),

        "la"  = Token::Instruction(Instr::La),
        "ja"  = Token::Instruction(Instr::Ja),

        /* grabbing an identifier */
        let id_init = ['a'-'z' 'A'-'Z' '_'];
        let id_subseq = $id_init | ['0'-'9'];

        $id_init $id_subseq* => |lexer| {
            let contents = lexer.match_().to_owned();
            lexer.return_(Token::Identifier(contents))
        },

        /* grabbing a register */
        '$' ['a'-'z']?+ ['0'-'9']?+ => |lexer| {
            let contents = lexer.match_();
            let token = 
            match contents {
                "$0"|"$zero" => Token::Register(0),
                "$1"|"$at" => Token::Register(1),
                "$2"|"$v" => Token::Register(2),
                "$3"|"$a0" => Token::Register(3),
                "$4"|"$a1" => Token::Register(4),
                "$5"|"$a2" => Token::Register(5),
                "$6"|"$s0" => Token::Register(6),
                "$7"|"$s1" => Token::Register(7),
                "$8"|"$s2" => Token::Register(8),
                "$9"|"$t0" => Token::Register(9),
                "$10"|"$t1" => Token::Register(10),
                "$11"|"$t2" => Token::Register(11),
                "$12"|"$t3" => Token::Register(12),
                "$13"|"$gv" => Token::Register(13),
                "$14"|"$ra" => Token::Register(14),
                "$15"|"$sp" => Token::Register(15),
                _ => Token::Err,
            };

            lexer.return_(token)
        },


        // grabbing a hex number
        let hexdigit = ['a'-'f' 'A'-'F' '0' - '9'];
        "0x" $hexdigit+ => |lexer| {
            let contents = lexer.match_();
            let stripped = &contents[2..contents.len()];
            match i64::from_str_radix(stripped, 16) {
                Ok(n) => lexer.return_(Token::Immediate(n)),
                Err(_) => lexer.return_(Token::Err),
            }
        },

        // grabbing a decimal number
        let digit = ['0'-'9'];

        ['+' '-']? $digit+ => |lexer| {
            let contents = lexer.match_().parse::<i64>().unwrap();
            lexer.return_(Token::Immediate(contents))
        },

        ".data" = Token::Block(Bl::Data), // data block is denoted by .data
        ".text" = Token::Block(Bl::Text), // text block starts with .text
        ".word" = Token::Block(Bl::Word), // word block starts with .word
        ".space" = Token::Block(Bl::Space), // space block starts with .space
        ".addr" = Token::Block(Bl::Addr), // address starts with .addr

    }

    /* when inside a coment, skip characters untl \n or EOF */
    rule Comment {
        '\n' => |lexer| {lexer.reset_match(); lexer.switch(LexerRule::Init)},
        $ => |lexer| lexer.switch(LexerRule::Init),
        _ ,
    }


}


#[allow(dead_code)]
fn ignore_pos<A, E, L>(ret: Option<Result<(L, A, L), E>>) -> Option<Result<A, E>> {
    ret.map(|res| res.map(|(_, a, _)| a))
}

#[test]
fn lex_asm_basic_tokens() {
    let mut lexer
     = Lexer::new("addi $t0, $zero, -256 \n add $t0, $zero, $sp");
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Instruction(Instr::Addi)))); 
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(9))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(0))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Immediate(-256))));

    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Instruction(Instr::Add)))); 
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(9))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(0))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(15))));
}

#[test]
fn lex_asm_comments() {
    let mut lexer
    = Lexer::new("#addi $t0, $zero, -256\n #addi $t0, $zero, -256\n #string\n #123465");
    assert_eq!( ignore_pos(lexer.next()), None);
}