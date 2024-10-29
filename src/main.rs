use greg::*;
use std::env;
use std::fs;
use std::process::exit;


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

    let binary = match assemble(&input_file_contents) {
        Ok(bin) => bin,
        Err(e) => panic!("{}", e),
    };

    io::print_to_file(&(output_file.clone() + ".data"), binary.data);
    io::print_to_file(&(output_file.clone() + ".instr"), binary.instructions);
    
}

fn usage_hint () {
    println!("Usage:");
    println!("greg infile outfile");
    exit(1);
}