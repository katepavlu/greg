use super::parserhelpers::*;
use super::*;

/// given the block identifier of a data node, parse the rest of it and return the node
pub fn parse_data(
    block: Bl,
    identifier: String,
    lexer: &mut Lexer<'_>,
    address: &mut u32,
) -> Result<DataNode, ParserError> {
    let mut data = 0;
    let mut num: u32 = 1;
    let mut addr = *address;

    match block {
        // form: .word immediate
        Bl::Word => {
            data = get_immediate(lexer.next())?;
            *address += 4;
        }
        // form: .word immediate
        // immediate is restricted to positive values, since it represents a number of words
        Bl::Space => {
            if let (loc, Token::Immediate(i)) = read_token(lexer.next())? {
                if i >= 0 {
                    num = i as u32;
                    *address += 4 * num;
                } else {
                    return Err(ParserError::NegativeSpace(loc));
                }
            };
        }
        // form .word immediate
        // immediate is restricted to positive values, since it represents a memory address
        Bl::Addr => {
            if let (loc, Token::Immediate(i)) = read_token(lexer.next())? {
                if i >= 0 {
                    addr = i as u32;
                } else {
                    return Err(ParserError::NegativeSpace(loc));
                }
            };
        }
        _ => {
            let (loc, _) = read_token(lexer.next())?;
            return Err(ParserError::Incomplete(loc));
        }
    }

    Ok(DataNode {
        identifier,
        address: addr,
        block,
        data,
        num,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data() {
        let input = "12";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_data(Bl::Word, "constant".to_string(), &mut lexer, &mut address).unwrap(),
            DataNode {
                identifier: "constant".to_string(),
                address: 0,
                block: Bl::Word,
                data: 12,
                num: 1,
            }
        );

        let input = "12";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_data(Bl::Space, "constant".to_string(), &mut lexer, &mut address).unwrap(),
            DataNode {
                identifier: "constant".to_string(),
                address: 4,
                block: Bl::Space,
                data: 0,
                num: 12,
            }
        );

        assert_eq!(address, 4 * 13);

        let input = "-12";
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_data(Bl::Space, "constant".to_string(), &mut lexer, &mut address),
            Err(ParserError::NegativeSpace(Loc { row: 0, col: 0 }))
        );

        let input = "0xffff0000";
        address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_data(Bl::Addr, "io".to_string(), &mut lexer, &mut address).unwrap(),
            DataNode {
                identifier: "io".to_string(),
                address: 0xffff0000,
                block: Bl::Addr,
                data: 0,
                num: 1,
            }
        );

        assert_eq!(address, 0);

        let input = "-4";
        let mut lexer = mylexer::Lexer::new(input);
        assert_eq!(
            parse_data(Bl::Addr, "constant".to_string(), &mut lexer, &mut address),
            Err(ParserError::NegativeSpace(Loc { row: 0, col: 0 }))
        );
    }
}
