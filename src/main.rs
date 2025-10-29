// entry point into the program

// input fileName.vm => fileName.asm

// constructs a Parser to handle the input file
// construct a code_writer to handle the output file
use std::env;
use vm_translator::vm_translator::{extract_file_name, translate};


fn main() {
    const MAX_NO_OF_FILES: u16 = 10;
    let args: Vec<String> = env::args().collect();
    // args.len() - 1, because the first argument is a reference to the target program
    if args.len() - 1 > MAX_NO_OF_FILES.into() {
        panic!(
            "Too many files, Expected Max number of files is {}, found {}",
            MAX_NO_OF_FILES,
            args.len()
        );
    }
    if args.len() - 1 == 0 {
        panic!("Expected atleast one .vm file: Found none!");
    }
    for arg in args.iter().skip(1) {
        let file_name = extract_file_name(&arg);
        println!("Assembling file : {}.asm",file_name);
        let result = translate(&arg);
        match result {
            Ok(_) => println!("Assembling successful ✅ Check output/{}.asm",file_name),
            Err(e) => println!("Failed to Assemble {}.vm : {}",file_name,e)
        }
    }
    
}
