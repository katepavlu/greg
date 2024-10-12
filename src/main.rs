use lexgen::lexer;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    Comma,
    Colon,
    Instruction(Instruction),
    Register(u8),
    Identifier(String),
    Immediate(i16),
    Err,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
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
    Sw
}

lexer! {
    Lexer -> Token;

    let whitespace = [' ' '\t' '\n'] | "\r\n";

    rule Init {
        $whitespace,

        ','   = Token::Comma,
        ':'   = Token::Colon,

        "and" = Token::Instruction(Instruction::And),
        "or"  = Token::Instruction(Instruction::Or),
        "xor" = Token::Instruction(Instruction::Xor),
        "not" = Token::Instruction(Instruction::Not),
        "add" = Token::Instruction(Instruction::Add),
        "sub" = Token::Instruction(Instruction::Sub),
        "cmp" = Token::Instruction(Instruction::Cmp),
        "j"   = Token::Instruction(Instruction::J),
        "beq" = Token::Instruction(Instruction::Beq),
        "bne" = Token::Instruction(Instruction::Bne),
        "sl"  = Token::Instruction(Instruction::Sl),
        "sr"  = Token::Instruction(Instruction::Sr),
        "addi"= Token::Instruction(Instruction::Addi),
        "lui" = Token::Instruction(Instruction::Lui),
        "lw"  = Token::Instruction(Instruction::Lw),
        "sw"  = Token::Instruction(Instruction::Sw),


        let id_init = ['a'-'z' 'A'-'Z' '_'];
        let id_subseq = $id_init | ['0'-'9'];

        $id_init $id_subseq* => |lexer| {
            let contents = lexer.match_().to_owned();
            lexer.return_(Token::Identifier(contents))
        },

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


        let digit = ['0'-'9'];

        ['+' '-']? $digit+ => |lexer| {
            let contents = lexer.match_().parse::<i16>().unwrap();
            lexer.return_(Token::Immediate(contents))
        },

        "//" => |lexer| lexer.switch(LexerRule::Comment),
        "/*" => |lexer| lexer.switch(LexerRule::MultiComment),

    }

    rule Comment {
        '\n' => |lexer| lexer.switch(LexerRule::Init),
        $ => |lexer| lexer.switch(LexerRule::Init),
        _ => |lexer| lexer.continue_(),
    }

    rule MultiComment {
        "*/" => |lexer| lexer.switch(LexerRule::Init),
        $ => |lexer| lexer.switch(LexerRule::Init),
        _ => |lexer| lexer.continue_(),

    }

}
fn main() {

    let lexer
     = Lexer::new("outer_loop_begin: addi $t0, $zero, -29\n");

    for token in lexer {
        match token {
            Ok(token) => println!("{:#?}", token.1),
            Err(_) => panic!("Something went wrong"),
        }        
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
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Instruction(Instruction::Addi)))); 
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(9))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(0))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Immediate(-256))));

    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Instruction(Instruction::Add)))); 
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(9))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(0))));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Comma)));
    assert_eq!( ignore_pos(lexer.next()), Some(Ok(Token::Register(15))));
}

#[test]
fn lex_asm_comments() {
    let mut lexer
    = Lexer::new("/*addi $t0, $zero, -256*/ //addi $t0, $zero, -256\n //addi $t0, $zero, -256");
    assert_eq!( ignore_pos(lexer.next()), None);
}