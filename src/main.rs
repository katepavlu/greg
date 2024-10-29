use greg::*;
use std::env;
use std::fs;
use std::process::exit;

/// # Greg the assembler
fn main() {

    // argument handling
    let args: Vec<String> = env::args().collect(); //take in two filenames (input, output)
    let (input_files, output_file) = parse_args(args);

    let mut program_listing = String::new();

    for file in input_files {
        let file_contents = 
        match fs::read_to_string(file) {
            Err(e) => panic!("Error reading input: {e}"),
            Ok(str) => str,
        };

        program_listing.push_str(&file_contents);
        program_listing.push('\n');
    }

    // assemble file, panicking on errors
    let binary = match assemble(&program_listing) { 
        Ok(bin) => bin,
        Err(e) => panic!("{}", e),
    };

    // print out assembled binary
    io::print_to_file(&(output_file.clone() + ".data"), binary.data);
    io::print_to_file(&(output_file.clone() + ".instr"), binary.instructions);
    
}


/// parse arguments given to the fucntion, exit with usage hint if something is not right
fn parse_args(args: Vec<String>) -> (Vec<String>, String) {
    let mut infiles: Vec<String> = Vec::new();
    let mut outfile = "a".to_string(); //output file defaults to "a"

    let mut args = args.iter();

    args.next(); // get rid of first argument (name of program)

    // mandatory: acqure one argument for input file
    let arg = match args.next() {
        Some(str) => str,
        None => usage_hint(),
    };
    infiles.push(arg.to_owned());

    // loop over the rest of the arguments
    loop{

        // break out o the loop once arguments run out
        let arg = match args.next() {
            Some(str) => str,
            None => break,
        };


        match &arg[..] {
            // if -o option is invoked, capture outfile name and break out of the loop
            "-o" => {
                outfile = match args.next() {
                    Some(str) => str.to_owned(),
                    None => usage_hint(),
                };
                break
            }
            // otherwise keep rading input files
            _ => infiles.push(arg.to_owned()),

        }
    }

    (infiles, outfile)
}

/// # Usage hint
/// 
/// display usage hint and exit if wrong number of arguments was read
fn usage_hint () -> ! {
    println!("Usage:");
    println!("greg [infile1] [infile2] ... -o [outfile]");
    println!("Mandatory argument: infile2. Other arguments optional.");
    println!("Produces [outfile].data and [outfile].instr; outfile defaults to \"a\"");
    exit(1);
}