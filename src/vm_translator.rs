// bring it together here

use std::{
    fs::{self, File, create_dir_all},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use crate::{code_writer::CodeWriter, parser::parse};

pub fn extract_file_name(file_name_or_path: &str) -> String {
    file_name_or_path
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .and_then(|s| s.split('.').collect::<Vec<&str>>().get(0).copied())
        .unwrap_or("")
        .to_string()
}

fn clean_line(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let no_comment = trimmed.split("//").next().unwrap().trim();
    if no_comment.is_empty() {
        return None;
    }

    Some(no_comment)
}

pub fn translate(file_name_or_path: &str) -> io::Result<bool> {
    let file = File::open(file_name_or_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut output_vec: Vec<String> = Vec::new();
    let file_name = extract_file_name(file_name_or_path);
    let mut code_writer = CodeWriter::new();
    code_writer.set_file_name(file_name.as_str());
    for line in lines {
        if let Some(cleaned_line) = clean_line(&line) {
            let comment = format!("// {}", &cleaned_line);  // translated code comment
            let command = parse(cleaned_line.to_string());
            let assembly_code = code_writer.command_to_assembly(command);
            output_vec.push(comment);
            output_vec.push(assembly_code);
        }
    }
    let output_string = output_vec.join("\n\n");
    if output_string.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Empty output"));
    }
    let dir = Path::new("output");
    if !dir.exists() {
        create_dir_all(dir)?;
    }
    let file_path = dir.join(format!("{}.asm", file_name));
    let mut file = File::create(file_path)?;
    file.write_all(output_string.as_bytes())?;

    Ok(true)
}

pub fn translate_dir(dir_name_or_path: &str) -> io::Result<bool> {
    let dir_path = Path::new(dir_name_or_path);
    let dir_name = dir_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string(); // use dir_name as output file name
    let mut output_vec: Vec<String> = Vec::new();
    let mut code_writer = CodeWriter::new();

    // add Bootstrapping code: intialize the stack and jump to sys.init
    output_vec.push(code_writer.write_init());

    // read and sort .vm files to make sure that Sys.vm is read first
    let mut vm_files: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "vm") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    vm_files.sort_by(|a, b| {
        if a.file_stem().map_or(false, |s| s == "Sys") {
            std::cmp::Ordering::Less
        } else if b.file_stem().map_or(false, |s| s == "Sys") {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    for path in vm_files {
        let file_name_str = path.file_name().unwrap_or_default().to_string_lossy();
        println!("  -> Processing file: {}", file_name_str);

        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
        // get file name using extract_file_name function
        let current_vm_file_name = extract_file_name(&file_name_str);
        code_writer.set_file_name(current_vm_file_name.as_str());
        output_vec.push(format!("\n// --- Start of {} ---", current_vm_file_name));

        for line in lines {
            if let Some(cleaned_line) = clean_line(&line) {
                let comment = format!("// {}", &cleaned_line);      // translated code comment
                let command = parse(cleaned_line.to_string());
                let assembly_code = code_writer.command_to_assembly(command);
                output_vec.push(comment);
                output_vec.push(assembly_code);
            }
        }
        output_vec.push(format!("\n// --- End of {} ---", current_vm_file_name));
    }

    let output_string = output_vec.join("\n\n");
    if output_string.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Empty output"));
    }

    let output_dir = Path::new("output");
    if !output_dir.exists() {
        create_dir_all(output_dir)?;
    }
    let output_file_path = output_dir.join(format!("{}.asm", dir_name));
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(output_string.as_bytes())?;

    Ok(true)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let translate = translate("./input/SimpleAdd.vm");
    }

    #[test]
    fn test_translate_dir(){
        let translate_dir = translate("./input/NestedCall");
    }
}
