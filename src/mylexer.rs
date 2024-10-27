use lexgen::lexer;

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

/// # Block annotations
/// 
/// Data - starts data block
/// 
/// Text - starts text block
/// 
/// Addr - selects direct memory access (used for memory mapped peripherals)
/// 
/// Space - selects contigous space of n words (zeroed)
/// 
/// Word - selects one initialised word
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

lexer! {
    ///
    pub Lexer -> Token;

    let whitespace = [' ' '\t' '\n'] | "\r\n";
    let alphanumeric = ['a'-'z' 'A'-'Z' '0'-'9' '_'];

    rule Init {
        $whitespace, //skip whitespace

        ','   = Token::Comma,
        ':'   = Token::Colon,

        '#' => |lexer| lexer.switch(LexerRule::Comment), // hash starts a comment

        // match instruction names if not followed by an alphanumeric characater
        "and" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::And), 
        "or"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Or),
        "xor" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Xor),
        "not" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Not),
        "add" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Add),
        "sub" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Sub),
        "cmp" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Cmp),
        "j"   > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::J),
        "beq" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Beq),
        "bne" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Bne),
        "sl"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Sl),
        "sr"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Sr),
        "addi"> ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Addi),
        "lui" > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Lui),
        "lw"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Lw),
        "sw"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Sw),

        "la"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::La),
        "ja"  > ((_ # $alphanumeric)|$) = Token::Instruction(Instr::Ja),

        // grabbing an identifier
        let id_init = ['a'-'z' 'A'-'Z' '_'];
        let id_subseq = $id_init | ['0'-'9'];

        $id_init $id_subseq* => |lexer| {
            let contents = lexer.match_().to_owned();
            lexer.return_(Token::Identifier(contents))
        },

        // grabbing and parsing a register
        // match if not followed by an alphanumeric characater
        ("$0"|"$zero") > ((_ # $alphanumeric)|$) = Token::Register(0),
        ("$1"|"$at") > ((_ # $alphanumeric)|$) = Token::Register(1),
        ("$2"|"$v") > ((_ # $alphanumeric)|$) = Token::Register(2),
        ("$3"|"$a0") > ((_ # $alphanumeric)|$) = Token::Register(3),
        ("$4"|"$a1") > ((_ # $alphanumeric)|$) = Token::Register(4),
        ("$5"|"$a2") > ((_ # $alphanumeric)|$) = Token::Register(5),
        ("$6"|"$s0") > ((_ # $alphanumeric)|$) = Token::Register(6),
        ("$7"|"$s1") > ((_ # $alphanumeric)|$) = Token::Register(7),
        ("$8"|"$s2") > ((_ # $alphanumeric)|$) = Token::Register(8),
        ("$9"|"$t0") > ((_ # $alphanumeric)|$) = Token::Register(9),
        ("$10"|"$t1") >((_ # $alphanumeric)|$) = Token::Register(10),
        ("$11"|"$t2") > ((_ # $alphanumeric)|$) = Token::Register(11),
        ("$12"|"$t3") > ((_ # $alphanumeric)|$) = Token::Register(12),
        ("$13"|"$gv") > ((_ # $alphanumeric)|$) = Token::Register(13),
        ("$14"|"$ra") > ((_ # $alphanumeric)|$) = Token::Register(14),
        ("$15"|"$sp") > ((_ # $alphanumeric)|$) = Token::Register(15),

        // grabbing and parsing a hex number
        // match if not followed by an alphanumeric characater
        let hexdigit = ['a'-'f' 'A'-'F' '0' - '9'];
        "0x" $hexdigit+ > ((_ # ['g'-'z' 'G'-'Z' '_'])|$)=> |lexer| {
            let contents = lexer.match_();
            let stripped = &contents[2..contents.len()];
            match i64::from_str_radix(stripped, 16) {
                Ok(n) => lexer.return_(Token::Immediate(n)),
                Err(_) => lexer.return_(Token::Err),
            }
        },

        // grabbing a decimal number
        // match if not followed by an alphanumeric characater
        let digit = ['0'-'9' '_'];

        ['+' '-']? $digit+ > ((_ # $id_init)|$) => |lexer| {
            let contents = lexer.match_().parse::<i64>().unwrap();
            lexer.return_(Token::Immediate(contents))
        },

        // match if not followed by an alphanumeric characater
        ".data" > ((_ # $alphanumeric)|$) = Token::Block(Bl::Data), // data block is denoted by .data
        ".text" > ((_ # $alphanumeric)|$) = Token::Block(Bl::Text), // text block starts with .text
        ".word" > ((_ # $alphanumeric)|$) = Token::Block(Bl::Word), // word block starts with .word
        ".space" > ((_ # $alphanumeric)|$) = Token::Block(Bl::Space), // space block starts with .space
        ".addr" > ((_ # $alphanumeric)|$) = Token::Block(Bl::Addr), // address starts with .addr

    }

    // when inside a coment, skip characters untl \n or EOF
    rule Comment {
        '\n' => |lexer| {lexer.reset_match(); lexer.switch(LexerRule::Init)},
        $ => |lexer| lexer.switch(LexerRule::Init),
        _ ,
    }


}

#[cfg(test)]
mod tests {

    use super::*;
    use lexgen_util::{LexerError, LexerErrorKind};

    fn get_value<A, E: std::fmt::Debug, L>(ret: Option<Result<(L, A, L), E>>) -> A {
        ret.map(|res| res.map(|(_, a, _)| a)).unwrap().unwrap()
    }

    // test each token type
    #[test]
    fn token_types() {
        let input = ", : add $zero loop_start 1600 .word";
        let mut lexer = Lexer::new(input);

        assert_eq!(get_value(lexer.next()), Token::Comma);
        assert_eq!(get_value(lexer.next()), Token::Colon);
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Add));
        assert_eq!(get_value(lexer.next()), Token::Register(0));
        assert_eq!(get_value(lexer.next()), Token::Identifier("loop_start".to_string()));
        assert_eq!(get_value(lexer.next()), Token::Immediate(1600));
        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Word));
        assert_eq!(lexer.next() , None);
    }

    // test each instruction type
    #[test]
    fn instructions() {
        let input = "and or xor not add sub cmp j beq bne sl sr addi lui lw sw la ja";
        let mut lexer = Lexer::new(input);

        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::And));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Or));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Xor));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Not));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Add));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Sub));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Cmp));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::J));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Beq));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Bne));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Sl));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Sr));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Addi));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Lui));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Lw));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Sw));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::La));
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Ja));
        assert_eq!(lexer.next() , None);
    }

    // test each register
    #[test]
    fn registers() {
        let input1 = "$0 $1 $2 $3 $4 $5 $6 $7 $8 $9 $10 $11 $12 $13 $14 $15";
        let input2 = "$zero $at $v $a0 $a1 $a2 $s0 $s1 $s2 $t0 $t1 $t2 $t3 $gv $ra $sp";
        let output:Vec<u8> = (0..=15).collect();


        let vec1:Vec<u8> = Lexer::new(input1)
            .map(|res| {
                match res {
                    Ok((_,Token::Register(i), _)) => i,
                    _ => 255,
                }
            })
            .collect();

        let vec2:Vec<u8> = Lexer::new(input2)
            .map(|res| {
                match res {
                    Ok((_,Token::Register(i), _)) => i,
                    _ => 255,
                }
            })
            .collect();

        assert_eq!(vec1, vec2);
        assert_eq!(vec1, output);
       
    }

    // test supported immediate types: signed decimal, unsigned hex
    #[test]
    fn immediate() {
        let input = "0 +0 -0 010 +010 -010 0xdeadbeef 0x7fffffffffffffff";
        let ovec:Vec<i64> = vec![0,0,0,10,10,-10, 0xdeadbeef, 0x7fffffffffffffff];

        let ivec:Vec<i64> = Lexer::new(input)
        .map(|res| {
            match res {
                Ok((_,Token::Immediate(i), _)) => i,
                _ => 255,
            }
        })
        .collect();
        assert_eq!(ovec, ivec);
    }

    // test supported block types
    #[test]
    fn blocks() {
        let input = ".data .text .addr .space .word";
        let mut lexer = Lexer::new(input);

        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Data));
        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Text));
        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Addr));
        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Space));
        assert_eq!(get_value(lexer.next()), Token::Block(Bl::Word));
        assert_eq!(lexer.next() , None);
    }

    // test comments
    #[test]
    fn comments() {
        let input = 
        "#addi $t0, $zero, -256\n
         addi #$t0, $zero, -256\n
         #string\n
         #123465";
        let mut lexer = Lexer::new(input);
        assert_eq!(get_value(lexer.next()), Token::Instruction(Instr::Addi));
        assert_eq!(lexer.next() , None);
    }

    // test malformed tokens
    #[test]
    fn garbled() {
        let malformed_strings = ["&t0", "+ěš", "$t4", "$one", ".home", "[", "𝝀", ".data1", "$t255"];

        for string in malformed_strings {
            let l = Lexer::new(string).next().unwrap();
            if let 
            Err(LexerError{location:_, kind: LexerErrorKind::InvalidToken}) 
                =  l
            {} else {panic!("String {string} not recognized as invalid. {:#?}",l);}
        }

        let valid_strings = ["add1", "addiu", "$t1"];
        for string in valid_strings {
            if let 
            Err(LexerError{location:_, kind: _}) 
            = Lexer::new(string).next().unwrap() 
            {panic!("String {string} recognized as invalid.");}
        }

    }

        

    /*    #[test]
    fn lex_asm_comments() {
        let mut lexer
        = Lexer::new("#addi $t0, $zero, -256\n #addi $t0, $zero, -256\n #string\n #123465");
        assert_eq!( ignore_pos(lexer.next()), None);
    }
    */

}