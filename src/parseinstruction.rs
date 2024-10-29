use crate::mylexer;
use crate::parsehelpers::*;
use super::*;

fn parse_instruction(
    instruction: Instr, mut identifier: String, lexer:&mut Lexer<'_>, address: &mut u32 )
     -> Result<Vec<InstructionNode>, ParserError> 
{
    let mut return_vector:Vec<InstructionNode> = Vec::new();
    
    let mut op = instruction;
    let mut rd = 0;
    let mut ra = 0;
    let mut rb = 0;
    let mut imm = 0;
    let mut imm_identifier = String::new();
    
    match op {
        Instr::And | Instr::Or | Instr::Xor | Instr::Add | Instr::Sub | Instr::Cmp => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            ra = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            rb = get_register(lexer.next())?;
        },
        Instr::Not | Instr::J => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            ra = get_register(lexer.next())?;
        },
        Instr::Sl | Instr::Sr | Instr::Addi => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            ra = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            imm = get_immediate(lexer.next())?;
        },

        Instr::Beq | Instr::Bne => {
            ra = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            rb = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            (imm_identifier, imm) = get_identifier_or_imm(lexer.next())?;
        },

        Instr::Lui => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            imm = get_immediate(lexer.next())?;
        },

        Instr::Lw => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            rb = get_register(lexer.next())?;
        },
        Instr::Sw => {
            ra = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            rb = get_register(lexer.next())?;
        },

        Instr::La => { // la is a pseudoinstruction that is composed of two others
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            (imm_identifier, imm) = get_identifier_or_imm(lexer.next())?;

            return_vector.push(
                InstructionNode{
                    op: Instr::Lui,
                    rd: 1, ra: 0, rb: 0,
                    imm: imm >> 16,
                    identifier,
                    imm_identifier: imm_identifier.clone(),
                    address: *address
                }
            );

            *address += 4;
            op = Instr::Addi;
            ra = 1;
            imm = imm & 0xffff;
            identifier = String::new();
        },

        Instr::Ja => {
            rd = get_register(lexer.next())?;
            sel_token(lexer.next(), Token::Comma)?;
            (imm_identifier, imm) = get_identifier_or_imm(lexer.next())?;

            return_vector.push(
                InstructionNode{
                    op: Instr::Lui,
                    rd: 1, ra: 0, rb: 0,
                    imm: imm >> 16,
                    identifier,
                    imm_identifier: imm_identifier.clone(),
                    address: *address
                }
            );

            *address += 4;

            return_vector.push(
                InstructionNode{
                    op: Instr::Addi,
                    rd: 1, ra: 1, rb: 0,
                    imm: imm & 0xffff,
                    identifier: String::new(),
                    imm_identifier: imm_identifier,
                    address: *address
                }
            );

            *address += 4;

            op = Instr::J;
            ra = 1;
            identifier = String::new();
            imm = 0;
            imm_identifier = String::new();
        },
    };

    return_vector.push(
        InstructionNode{
            op, rd, ra, rb, imm, identifier, imm_identifier, address: *address
        }
    );
    *address += 4;

    Ok(return_vector)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;
    #[test]
    fn instruction() {
        let input = "$1, $2, $3 ";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::Add, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![InstructionNode{
                op:Instr::Add,
                rd:1,
                ra:2,
                rb:3,
                imm:0,
                identifier: "loop".to_string(),
                imm_identifier: String::new(),
                address:0,
            }]
        );

        let input = "$1, $2, 0xf";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::Addi, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![InstructionNode{
                op:Instr::Addi,
                rd:1,
                ra:2,
                rb:0,
                imm:15,
                identifier: "loop".to_string(),
                imm_identifier: String::new(),
                address:0,
            }]
        );

        let input = "$1, $2, loop1";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::Beq, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![InstructionNode{
                op:Instr::Beq,
                rd:0,
                ra:1,
                rb:2,
                imm:0,
                identifier: "loop".to_string(),
                imm_identifier: "loop1".to_string(),
                address:0,
            }]
        );

        let input = "$8, loop1";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::La, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![
            InstructionNode{
                op:Instr::Lui,
                rd:1,
                ra:0,
                rb:0 ,
                imm:0,
                identifier: "loop".to_string(),
                imm_identifier: "loop1".to_string(),
                address:0,
            },
            InstructionNode{
                op:Instr::Addi,
                rd:8,
                ra:1,
                rb:0,
                imm:0,
                identifier: "".to_string(),
                imm_identifier: "loop1".to_string(),
                address:4,
            }]
        );

        let input = "$8, 0x12345678";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::La, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![
            InstructionNode{
                op:Instr::Lui,
                rd:1,
                ra:0,
                rb:0 ,
                imm:0x1234,
                identifier: "loop".to_string(),
                imm_identifier: "".to_string(),
                address:0,
            },
            InstructionNode{
                op:Instr::Addi,
                rd:8,
                ra:1,
                rb:0,
                imm:0x5678,
                identifier: "".to_string(),
                imm_identifier: "".to_string(),
                address:4,
            }]
        );

        let input = "$8, loop1";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::Ja, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![
            InstructionNode{
                op:Instr::Lui,
                rd:1,
                ra:0,
                rb:0,
                imm:0,
                identifier: "loop".to_string(),
                imm_identifier: "loop1".to_string(),
                address:0,
            },
            InstructionNode{
                op:Instr::Addi,
                rd:1,
                ra:1,
                rb:0,
                imm:0,
                identifier: "".to_string(),
                imm_identifier: "loop1".to_string(),
                address:4,
            },
            InstructionNode{
                op:Instr::J,
                rd:8,
                ra:1,
                rb:0,
                imm:0,
                identifier: "".to_string(),
                imm_identifier: "".to_string(),
                address:8,
            }]
        );
        
        let input = "$8, 0x12345678";
        let mut address = 0;
        let mut lexer = mylexer::Lexer::new(input);

        assert_eq!(
            parse_instruction(Instr::Ja, "loop".to_string(), &mut lexer, &mut address).unwrap(),
            vec![
            InstructionNode{
                op:Instr::Lui,
                rd:1,
                ra:0,
                rb:0 ,
                imm:0x1234,
                identifier: "loop".to_string(),
                imm_identifier: "".to_string(),
                address:0,
            },
            InstructionNode{
                op:Instr::Addi,
                rd:1,
                ra:1,
                rb:0,
                imm:0x5678,
                identifier: "".to_string(),
                imm_identifier: "".to_string(),
                address:4,
            },
            InstructionNode{
                op:Instr::J,
                rd:8,
                ra:1,
                rb:0,
                imm:0,
                identifier: "".to_string(),
                imm_identifier: "".to_string(),
                address:8,
            }]
        );

    }
}