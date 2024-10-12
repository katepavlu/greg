use logos::Logos;

pub mod lex_enums;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}


fn main() {
    let lex = lex_enums::TokenType::lexer("Loop_start: add $zero, $s0, $s1");

    for result in lex {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(_) => panic!("some error occurred."),
        }
    }

  

}
