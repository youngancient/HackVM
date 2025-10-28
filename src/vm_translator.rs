// bring it together here

use std::{fs::{File, create_dir, create_dir_all}, io::{self, BufRead, Write}, path::Path};

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


pub fn translate(file_name_or_path: &str) -> io::Result<bool>{
    let file = File::open(file_name_or_path)?;
    let reader = io::BufReader::new(file);
    let lines:Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut output_vec:Vec<String> = Vec::new();
    let file_name = extract_file_name(file_name_or_path);
    let mut code_writer = CodeWriter::new(file_name.as_str());
    for line in lines{
        let comment = format!("// {}", &line);
        if let Some(cleaned_line) = clean_line(&line){
            let command = parse(cleaned_line.to_string());
            let assembly_code = code_writer.command_to_assembly(command);
            output_vec.push(comment);
            output_vec.push(assembly_code);
        }
    }
    let output_string = output_vec.join("\n\n");
    if output_string.trim().is_empty(){
        return Err(io::Error::new(io::ErrorKind::Other, "Empty output"));
    }
    let dir = Path::new("output");
    if !dir.exists(){
        create_dir_all(dir)?;
    }
    let file_path = dir.join(format!("{}.asm",file_name));
    let mut file = File::create(file_path)?;
    file.write_all(output_string.as_bytes())?;

    Ok(true)
}




#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_translate(){
        let translate_1 = translate("./input/BasicTest.vm");
    }
}