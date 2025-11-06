// entry point into the program

// input fileName.vm => fileName.asm

// constructs a Parser to handle the input file
// construct a code_writer to handle the output file
use std::{env, path::Path};
use vm_translator::vm_translator::{extract_file_name, translate, translate_dir};


fn main() {
    const MAX_NO_OF_FILES: u16 = 10;
    let args: Vec<String> = env::args().collect();
    // args.len() - 1, because the first argument is a reference to the target program
    if args.len() - 1 > MAX_NO_OF_FILES.into() {
        eprintln!(
            "Too many files, Expected Max number of files is {}, found {}",
            MAX_NO_OF_FILES,
            args.len()
        );
        std::process::exit(1);
    }
    if args.len() - 1 == 0 {
        eprintln!("Usage: vm_translator <file_or_directory_path> [file_or_directory_path...]");
        std::process::exit(1);
    }
    for arg in args.iter().skip(1) {
        let path = Path::new(arg);
        if path.is_file(){
            let file_name = extract_file_name(&arg);
            println!("Assembling file : {}.vm",file_name);
            let result = translate(&arg);
            match result {
                Ok(_) => println!("Assembling successful ✅ Check output/{}.asm",file_name),
                Err(e) => println!("Failed to Assemble {}.vm : {}",file_name,e)
            }
        } else if path.is_dir(){
            // handle directory 
            let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
            println!("Assembling directory: {}", dir_name);
            match translate_dir(arg) {
                Ok(_) => println!("Directory assembly successful ✅ Check output/{}.asm", dir_name),
                Err(err) => eprintln!("Failed to Assemble directory {}: {}",dir_name,err)
            }

        } else{
            eprintln!("Skipping invalid path: {} (Not a file or directory)",arg);
        }
    }
    
}
