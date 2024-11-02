use greg::*;
use std::env;
use std::fs;
use std::process::exit;

/// # Greg the assembler
fn main() {
    // argument handling
    let args: Vec<String> = env::args().collect(); //take in two filenames (input, output)
    let (input_files, output_file, offset) = parse_args(args);

    let mut program_listing = String::new();

    let mut input_file_sizes:Vec<(String, usize)> = Vec::new();


    for file in input_files {
        let file_contents = match fs::read_to_string(&file) {
            Err(e) => panic!("Error reading input: {e}"),
            Ok(str) => str,
        };

        // save the input file name and length
        input_file_sizes.push(
            (file, file_contents.lines().count())
        );

        program_listing.push_str(&file_contents);
        program_listing.push('\n');
    }

    // assemble file, panicking on errors
    let hex = match assemble(&program_listing, offset) {
        Ok(hex) => hex,
        Err(e) => {
            match &e {
                AssemblerError::ParserError(ParserError::Incomplete(loc)) |
                AssemblerError::ParserError(ParserError::InvalidToken(loc)) |
                AssemblerError::ParserError(ParserError::NegativeSpace(loc)) |
                AssemblerError::ParserError(ParserError::CodeOutsideSegment(loc)) => {
                    let mut file = String::new();
                    let mut row = loc.row;
                    for (file0, lines) in input_file_sizes {
                        file = file0;
                        if row > (lines as u32) {
                            row -= lines as u32;
                        }
                        else {
                            break
                        }
                    }
                    panic!("{}: file: {} line: {} column: {}", e, file, row+1, loc.col);

                }

                _ => panic!("{}", e)
            }
        }
    };

    // print out assembled binary
    io::print_to_file(&output_file, hex);
}

/// parse arguments given to the fucntion, exit with usage hint if something is not right
fn parse_args(args: Vec<String>) -> (Vec<String>, String, u32) {
    let mut infiles: Vec<String> = Vec::new();
    let mut outfile = "a.hex".to_string(); //output file defaults to "a"
    let mut offset = 0x400;

    let mut args = args.iter();

    args.next(); // get rid of first argument (name of program)

    // mandatory: acqure one argument for input file
    let arg = match args.next() {
        Some(str) => str,
        None => usage_hint(),
    };
    infiles.push(arg.to_owned());

    // loop over the rest of the arguments
    // break out o the loop once arguments run out
    while let Some(arg) = args.next() {
        match &arg[..] {
            // if -o option is invoked, capture outfile name and break out of the loop
            "-o" => {
                outfile = match args.next() {
                    Some(str) => str.to_owned(),
                    None => usage_hint(),
                };
            }
            "-p" => {
                let temp = match args.next() {
                    Some(str) => str.to_owned(),
                    None => usage_hint(),
                };
                offset = match temp.parse::<u32>() {
                    Ok(str) => str.to_owned(),
                    Err(_) => usage_hint(),
                };
            }
            // otherwise keep rading input files
            _ => infiles.push(arg.to_owned()),
        }
    }

    (infiles, outfile, offset)
}

/// # Usage hint
///
/// display usage hint and exit if wrong number of arguments was read
fn usage_hint() -> ! {
    println!("-------------------------- greg the compiler - v3.0.1 -------------------------");
    println!("| Usage:                                                                      |");
    println!("| greg [infile1] [infile2] ... -o [outfile] -p [physical memory .data offset] |");
    println!("| infile1: First input file. Mandatory.                                       |");
    println!("| infile*: Additional input fils. Linked with main file during assembly.      |");
    println!("| Arguments:                                                                  |");
    println!("| -o | output file name - defaults to \"a.hex\"                                 |");
    println!("| -p | offset of .data segment in physical memory - defaults to 0x400         |");
    println!("-------------------------------------------------------------------------------");
    exit(1);
}
