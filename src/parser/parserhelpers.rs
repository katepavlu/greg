pub type LexerResult = Result<(lexgen_util::Loc, Token, lexgen_util::Loc), LexerError<Infallible>>;
pub type Lexer<'a> = mylexer::Lexer_<'a, std::str::Chars<'a>, ()>;

use super::mylexer;
use super::*;

/// # Parse single token
///
/// converts lexer error types to parser errors
///
/// converts lexer output types to parser input types
pub fn parse_token(ret: LexerResult) -> ParserResult {
    match ret {
        Ok((l, t, _)) => Ok((
            Loc {
                row: l.line,
                col: l.col,
            },
            t,
        )),
        Err(LexerError {
            location: l,
            kind: _,
        }) => Err(ParserError::InvalidToken(Loc {
            row: l.line,
            col: l.col,
        })),
    }
}

/// # Read single token
///
/// wraps parse_token
///
/// returns ParserError::IncompleteInstruction(Loc) if EOF is encountered
pub fn read_token(ret: Option<LexerResult>) -> ParserResult {
    let ret = match ret {
        Some(x) => x,
        None => return Err(ParserError::End),
    };

    parse_token(ret)
}

/// # Select token
///
/// returns ParseError::IncompleteInstruction(Loc) if the next token is not the selected token
pub fn sel_token(ret: Option<LexerResult>, selection: Token) -> ParserResult {
    match read_token(ret)? {
        (loc, token) if token == selection => Ok((loc, token)),
        (loc, _) => Err(ParserError::Incomplete(loc)),
    }
}

/// # Get register
///
/// uses sel_token to read a register number. Forwards errors
pub fn get_register(ret: Option<LexerResult>) -> Result<u8, ParserError> {
    match read_token(ret)? {
        (_, Token::Register(x)) => Ok(x),
        (l, _) => Err(ParserError::Incomplete(l)),
    }
}

/// # Get immediate
///
/// uses sel_token to read an immediate. Forwards errors
pub fn get_immediate(ret: Option<LexerResult>) -> Result<i64, ParserError> {
    match read_token(ret)? {
        (_, Token::Immediate(x)) => Ok(x),
        (l, _) => Err(ParserError::Incomplete(l)),
    }
}

/// # Get identifier or immediate
///
/// uses sel_token to read an immediate. Forwards errors
pub fn get_identifier_or_imm(ret: Option<LexerResult>) -> Result<(String, i64), ParserError> {
    match read_token(ret)? {
        (_, Token::Identifier(str)) => Ok((str, 0)),
        (_, Token::Immediate(x)) => Ok((String::new(), x)),
        (loc, _) => Err(ParserError::Incomplete(loc)),
    }
}

#[cfg(test)]
mod tests {
    use super::mylexer;
    use super::*;

    #[test]
    fn reading() {
        let input = ", : add $zero loop_start 1600 .word";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(read_token(lexer.next()).unwrap().1, Token::Comma);
        assert_eq!(read_token(lexer.next()).unwrap().1, Token::Colon);
        assert_eq!(
            read_token(lexer.next()).unwrap().1,
            Token::Instruction(Instr::Add)
        );
        assert_eq!(read_token(lexer.next()).unwrap().1, Token::Register(0));
        assert_eq!(
            read_token(lexer.next()).unwrap().1,
            Token::Identifier("loop_start".to_string())
        );
        assert_eq!(read_token(lexer.next()).unwrap().1, Token::Immediate(1600));
        assert_eq!(read_token(lexer.next()).unwrap().1, Token::Block(Bl::Word));
        assert_eq!(read_token(lexer.next()), Err(ParserError::End));
    }

    #[test]
    fn selecting() {
        let input = ", : add &";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            sel_token(lexer.next(), Token::Comma).unwrap().1,
            Token::Comma
        );
        assert_eq!(
            sel_token(lexer.next(), Token::Colon).unwrap().1,
            Token::Colon
        );
        assert_eq!(
            sel_token(lexer.next(), Token::Colon),
            Err(ParserError::Incomplete(Loc { row: 0, col: 4 }))
        );
        assert_eq!(
            sel_token(lexer.next(), Token::Colon),
            Err(ParserError::InvalidToken(Loc { row: 0, col: 8 }))
        );
    }

    #[test]
    fn register() {
        let input = "$zero \n 12 $one";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(get_register(lexer.next()).unwrap(), 0);
        assert_eq!(
            get_register(lexer.next()),
            Err(ParserError::Incomplete(Loc { row: 1, col: 1 }))
        );
        assert_eq!(
            get_register(lexer.next()),
            Err(ParserError::InvalidToken(Loc { row: 1, col: 4 }))
        );
    }

    #[test]
    fn immediate() {
        let input = "-12 0xf $one";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(get_immediate(lexer.next()).unwrap(), -12);
        assert_eq!(get_immediate(lexer.next()).unwrap(), 15);
        assert_eq!(
            get_register(lexer.next()),
            Err(ParserError::InvalidToken(Loc { row: 0, col: 8 }))
        );
    }

    #[test]
    fn identifier_or_imm() {
        let input = "-12 greg +";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            get_identifier_or_imm(lexer.next()).unwrap(),
            ("".to_owned(), -12)
        );
        assert_eq!(
            get_identifier_or_imm(lexer.next()).unwrap(),
            ("greg".to_owned(), 0)
        );
        assert_eq!(
            get_register(lexer.next()),
            Err(ParserError::InvalidToken(Loc { row: 0, col: 9 }))
        );
    }
}
