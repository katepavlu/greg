use std::{
    fs::File,
    io::prelude::*,
    path::Path,
    process
};

/// Print a binary (u32) buffer to a file specified in filename
pub fn print_to_file(filename:&str, buffer:Vec<u32>) {
    // create a file for the output (data)
    let file_path = Path::new(&filename);

    let mut file_handle = match File::create(&file_path) {
        Err(why) => panic!("Couldn't create file: {}; {}", file_path.display(), why),
        Ok(handle) => handle,
    };

    // save all the gatherred data to it
    for num in buffer {
        match file_handle.write_all(&num.to_be_bytes()) {
            Ok(_) => (),
            Err(_)=> file_cleanup(file_path),
        }
    }

    // flush
    match file_handle.flush()  {
        Ok(_) => (),
        Err(_)=> file_cleanup(file_path),
    };
}

/// cleanup after an io error: delete the created file or display a message if unable
fn file_cleanup(file: &Path)-> ! {
    eprintln!("File write error: {}", file.display());
    std::fs::remove_file(file).expect("File deletion failed. Delete output file and try again.");
    process::exit(1);
}