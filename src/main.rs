use greg::*;
use lexgen_util::LexerError;
use lexgen_util::Loc;

use std::convert::Infallible;
use std::env;
use std::fs;
use std::process::exit;
use std::collections::HashMap;

pub use mylexer::*;

#[derive(Debug)]
struct MachineInstruction {
    op: Instr,
    rd: u8,
    ra: u8,
    rb: u8,
    imm: i64,
    identifier: String,
    imm_identifier: String,
    address: u32,
}




fn main() {
    let args: Vec<String> = env::args().collect(); //take in two filenames (input, output)
    if args.len() != 3 {
        usage_hint();
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let input_file_contents = 
        fs::read_to_string(input_file)
        .expect("Error reading input file."); //read input file to a string

    let mut lexer
        = Lexer::new(&input_file_contents); // create a lexer iterator thing to recognize tokens in the input

    let mut address; // this holds the address of the instruction you're currently working on
    let mut instr_identifier_map = HashMap::new(); // hash map for linking identifiers
    let mut mach_instr_vec = Vec::new(); // vector of instructions
    let mut data_segment_vector = Vec::new(); // vector for binary data
    let mut bin_instr_vector = Vec::new(); // vector for binary instruction

    // first, interpret the data segment
    // .data has to be the first token in the file
    let mut token0: Option<Result<(Loc, Token, Loc), LexerError<Infallible>>> = lexer.next();
    match token0 {
        Some(Ok((_,Token::Block(Bl::Data),_))) => (),
        Some(Ok((_,_,_))) => {
            println!("Input file must begin with a data segment");
            exit(1);
        },
        Some(Err(e)) => parse_err(e.location.line, e.location.col),
        None => {
            println!("Input file empty");
            exit(1);
        }
    }

    address = DATA_ADDRESS_OFFSET;


    let mut identifier = String::new();

    // looping over addresses in the data segment
    loop {
        token0 = lexer.next();

        match token0 {
            None => {println!("EOF reached, no text segment found"); exit(1)},
            Some(Err(e)) => parse_err(e.location.line, e.location.col),
            Some(Ok((_,Token::Block(Bl::Text),_))) => break,    // exit loop once you find the text segment

            Some(Ok((_,Token::Identifier(str),_))) => { //if identifier found:
                read_colon(lexer.next()); // next token must be a colon

                identifier = str;

                instr_identifier_map.insert(identifier.clone(), address); // register address name

                continue;
            },
            Some(Ok((_,Token::Block(Bl::Word),_))) => { // if this is a word, 
                let buf = read_immediate(lexer.next()) as u32; // read the word

                data_segment_vector.push(buf);

                address += 4; // increment address
                continue;
            },
            Some(Ok((_,Token::Block(Bl::Space),_))) => {    // if this is a space
                let num = read_immediate(lexer.next()) as u32; // read number of words requested

                for _i in 0..num {
                    let buf:u32 = 0;
                    data_segment_vector.push(buf);
                    address += 4;
                }
            },
            Some(Ok((_,Token::Block(Bl::Addr),_))) => {     // if this is an address
                let addr = read_immediate(lexer.next()) as u32;
                instr_identifier_map.insert(identifier.clone(), addr); // register new expolicitly stated name
                identifier = String::new();
            },
            Some(Ok((loc,_,_))) =>  parse_err(loc.line, loc.col), // if this is anything else, throw error and exit
        }      
    }


    io::print_to_file(&(output_file.clone() + ".data"), data_segment_vector);


    address = TEXT_ADDRESS_OFFSET;

    // looping over instructions: first loop, interpretting
    loop {
        let mut mach_inst = 
            MachineInstruction{
                op:Instr::Add, 
                rd:0, ra:0, rb:0, 
                imm:0, 
                identifier:String::new(), 
                imm_identifier:String::new(),
                address:address,
            }; // create a new instruction

        token0 = lexer.next(); // read a token

        // if the token is an identifier, register it to the next instruction
        match token0 {
            Some(Ok((_,Token::Identifier(str),_))) => {

                mach_inst.identifier = str; // register identifier
                read_colon(lexer.next()); // next token must be a colon

                instr_identifier_map.insert(mach_inst.identifier.clone(), address); // register address name

                token0 = lexer.next();
            },
            _ => (),
        }

        // if you find a : after an identifier, register its address to the hash map

        // if the token is an instruction, register the type and read the registers/immediate that it needs
        match token0 {
            None => {println!("Success."); break},
            Some(Err(e)) => parse_err(e.location.line, e.location.col),
            Some(Ok((l,token,_))) => {

                match token {
                    Token::Instruction(Instr::Add) => {
                        mach_inst.op = Instr::Add;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Addi) => {
                        mach_inst.op = Instr::Addi;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.imm = read_immediate(lexer.next());
                      }
                    Token::Instruction(Instr::And) => {
                        mach_inst.op = Instr::And;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Beq) => {
                        mach_inst.op = Instr::Beq;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        (mach_inst.imm_identifier, mach_inst.imm) = read_identifier_or(lexer.next());
                    }
                    Token::Instruction(Instr::Bne) => {
                        mach_inst.op = Instr::Bne;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        (mach_inst.imm_identifier, mach_inst.imm) = read_identifier_or(lexer.next());
                    }
                    Token::Instruction(Instr::Cmp) => {
                        mach_inst.op = Instr::Cmp;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::J) => {
                        mach_inst.op = Instr::J;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Lui) => {
                        mach_inst.op = Instr::Lui;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.imm = read_immediate(lexer.next());
                    }
                    Token::Instruction(Instr::Lw) => {
                        mach_inst.op = Instr::Lw;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Not) => {
                        mach_inst.op = Instr::Not;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Or) => {
                        mach_inst.op = Instr::Or;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Sl) => {
                        mach_inst.op = Instr::Sl;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.imm = read_immediate(lexer.next());
                    }
                    Token::Instruction(Instr::Sr) => {
                        mach_inst.op = Instr::Sr;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.imm = read_immediate(lexer.next());
                    }
                    Token::Instruction(Instr::Sub) => {
                        mach_inst.op = Instr::Sub;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Sw) => {
                        mach_inst.op = Instr::Sw;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::Xor) => {
                        mach_inst.op = Instr::Xor;
                        mach_inst.rd = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.ra = read_register(lexer.next());
                        read_comma(lexer.next());
                        mach_inst.rb = read_register(lexer.next());
                    }
                    Token::Instruction(Instr::La) => { // la is a pseudoinstruction that is composed of two others
                        let dest_reg = read_register(lexer.next());
                        read_comma(lexer.next());
                        let (id_string,_) = read_identifier_or(lexer.next());

                        mach_inst.op = Instr::Lui;
                        mach_inst.rd = 1;
                        mach_inst.imm_identifier = id_string.clone();

                        mach_instr_vec.push(mach_inst); // push lui to the instruction stack

                        address += 4; // incr address

                        mach_inst = 
                        MachineInstruction{
                            op:Instr::Addi, 
                            rd:dest_reg, ra:1, rb:0, 
                            imm:0, 
                            identifier:String::new(), 
                            imm_identifier:id_string,
                            address:address,
                        }; // create a new instruction (addi)
                    }
                    Token::Instruction(Instr::Ja) => { // ja is a pseudoinstruction that is composed of three others
                        let dest_reg = read_register(lexer.next());
                        read_comma(lexer.next());
                        let (id_string,_) = read_identifier_or(lexer.next());

                        mach_inst.op = Instr::Lui;
                        mach_inst.rd = 1;
                        mach_inst.imm_identifier = id_string.clone();

                        mach_instr_vec.push(mach_inst); // push lui to the instruction stack

                        address += 4; // incr address

                        mach_inst = 
                        MachineInstruction{
                            op:Instr::Addi, 
                            rd:1, ra:1, rb:0, 
                            imm:0, 
                            identifier:String::new(), 
                            imm_identifier:id_string,
                            address:address,
                        }; // create a new instruction (addi)

                        mach_instr_vec.push(mach_inst); // push addi to the instruction stack

                        address += 4; // incr address

                        mach_inst = 
                        MachineInstruction{
                            op:Instr::J, 
                            rd:dest_reg, ra:1, rb:0, 
                            imm:0, 
                            identifier:String::new(), 
                            imm_identifier:String::new(),
                            address:address,
                        }; // create a new instruction (j)

                    }

                    _ => incomplete_instruction(l.line, l.col),
                }  

            },
        }
        // push the instruction onto a stack
        mach_instr_vec.push(mach_inst); 
        // increment the address
        address += 4;
        
    }

    // loop over collected instructions
    for mut instruction in mach_instr_vec {

        if instruction.imm_identifier.len() != 0 {

            // link identifiers
            let target_address = 
                match instr_identifier_map.get(&instruction.imm_identifier) {
                    None => {
                        println!("Identifier not recognized: {}", instruction.imm_identifier);
                        exit(1);
                    },
                    Some(n)=> n.to_owned(),
                };

            match instruction.op {
                Instr::Beq|Instr::Bne => { // beq, bne require an offset if they have an identifier
                    instruction.imm = target_address as i64 - instruction.address as i64;
                },
                Instr::Addi => {
                    instruction.imm = (target_address & 0xffff) as i64;
                }
                Instr::Lui => {
                    instruction.imm = ((target_address & 0xffff0000) >> 16 ) as i64;
                }
                _ => (),
            }

        }

        //println!("{:#?}",instruction); // debug: print instruction

        let mut bin_instr:u32 = 0; // assemble instruction 

        bin_instr |= (match instruction.op { // add opcode
            Instr::And => 0b0000,
            Instr::Or  => 0b0001,
            Instr::Xor => 0b0010,
            Instr::Not => 0b0011,

            Instr::Add => 0b0100,
            Instr::Sub => 0b0101,
            Instr::Cmp => 0b0110,

            Instr::J   => 0b0111,
            Instr::Beq => 0b1000,
            Instr::Bne => 0b1001,

            Instr::Sl  => 0b1010,
            Instr::Sr  => 0b1011,
            Instr::Addi=> 0b1100,
            Instr::Lui => 0b1101,

            Instr::Lw =>  0b1110,
            Instr::Sw =>  0b1111,
            _ => {
                panic!("Pseudoinstruction not handled. This is a bug");
            },
        }) << 28;
        
        bin_instr |= (instruction.rd as u32)<<24 & 0b1111 << 24; // add Rd
        bin_instr |= (instruction.ra as u32)<<20 & 0b1111 << 20; // add Ra
        bin_instr |= (instruction.rb as u32)<<16 & 0b1111 << 16; // add Rb

        bin_instr |= instruction.imm as u32 & 0xFFFF; // add immediate

        bin_instr_vector.push(bin_instr);

    }

    io::print_to_file(&(output_file.clone() + ".instr"), bin_instr_vector);


}

fn usage_hint () {
    println!("Usage:");
    println!("greg infile outfile");
    exit(1);
}

fn parse_err(line:u32, col:u32) {
    println!("Parse error: line {} character {}", line, col);
    exit(1);
}

fn incomplete_instruction(line:u32, col:u32) {
    println!("Incomplete instruction: line {} character {}", line, col);
    exit(1);
}

fn error_reading_number(line:u32, col:u32) {
    println!("Error reading numberL line {} character {}", line, col);
    exit(1);
}

fn read_token(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) -> (Loc,Token) {
    match input {
        None => {
            println!("EOF reached before end of instruction");
            exit(1);
        },
        Some(Err(e)) => {parse_err(e.location.line, e.location.col);exit(1)},
        Some(Ok((l,token,_))) => return (l,token),
    }
}

fn read_register(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) -> u8 {
    let (loc, token) = read_token(input);
    match token {
        Token::Register(n) => return n,
        Token::Err => {
            println!("Invalid register: line {} character {}", loc.line, loc.col);
            exit(1);
        },
        _ => {incomplete_instruction(loc.line, loc.col);exit(1)},
    }
}

fn read_comma(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) {
    let (loc, token) = read_token(input);
    match token {
        Token::Comma => return,
        _ => {incomplete_instruction(loc.line, loc.col);exit(1)},
    }
}

fn read_colon(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) {
    let (loc, token) = read_token(input);
    match token {
        Token::Colon => return,
        _ => {
            println!("Identifier missing colon: line {} character {}", loc.line, loc.col);
            exit(1);
        },
    }
}

fn read_immediate(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) -> i64 {
    let (loc, token) = read_token(input);
    match token {
        Token::Immediate(n) => return n,
        Token::Err => {error_reading_number(loc.line, loc.col);exit(1)},
        _ => {incomplete_instruction(loc.line, loc.col);exit(1)},
    }
}

fn read_identifier_or(input: Option<Result<(Loc, Token, Loc) , LexerError<Infallible>>>) -> (String, i64) {
    let (loc, token) = read_token(input);
    match token {
        Token::Identifier(str) => return (str, 0),
        Token::Immediate(i) => return (String::new(), i),
        Token::Err => {error_reading_number(loc.line, loc.col);exit(1)},
        _ => {incomplete_instruction(loc.line, loc.col);exit(1)},
    }
}