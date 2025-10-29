// writes the assembly code that implements the parsed command
// converts CommandType to assembly code

use crate::parser::{CommandType, Operation, Segment};

pub enum MemoryAccess {
    Push,
    Pop,
}

pub struct CodeWriter {
    file_name: String,
    jump_label_id: u32,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        CodeWriter {
            file_name: file_name.to_string(),
            jump_label_id: 0,
        }
    }
    pub fn command_to_assembly(&mut self, command: CommandType) -> String {
        match command {
            CommandType::Arithmetic { operation } => self.write_arithmetic(operation),
            CommandType::Pop { segment, index } => {
                self.write_push_pop(segment, index, MemoryAccess::Pop)
            }
            CommandType::Push { segment, index } => {
                self.write_push_pop(segment, index, MemoryAccess::Push)
            }
            _ => panic!("Dont understand these yet!"),
        }
    }

    pub fn write_arithmetic(&mut self, operation: Operation) -> String {
        match operation {
            Operation::Add | Operation::Sub => {
                let selected_op = if operation == Operation::Add {
                    "+"
                } else {
                    "-"
                };
                let assembly_code =
                    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M{selected_op}D\n@SP\nM=M+1");
                return assembly_code;
            }
            Operation::Neg | Operation::Not => {
                let selected_operation = if operation == Operation::Neg {
                    "-"
                } else {
                    "!"
                };
                let assembly_code = format!("@SP\nA=M\nA=A-1\nM={selected_operation}M");
                return assembly_code;
            }
            Operation::And | Operation::Or => {
                let selected_operation = if operation == Operation::Add {
                    "&"
                } else {
                    "|"
                };
                let assembly_code = format!(
                    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D{selected_operation}M\n@SP\nM=M+1"
                );
                return assembly_code;
            }
            Operation::Eq | Operation::Gt | Operation::Lt => {
                let operation_jmp_directive = if operation == Operation::Eq {
                    "JEQ"
                } else if operation == Operation::Lt {
                    "JLT"
                } else {
                    "JGT"
                };

                let prefix = match operation {
                    Operation::Eq => "EQ",
                    Operation::Gt => "GT",
                    Operation::Lt => "LT",
                    _ => panic!("unsupported operation"),
                };
                let true_jump_label = format!("{prefix}_TRUE_{i}", i = self.jump_label_id);
                let end_jump_label = format!("{prefix}_END_{i}", i = self.jump_label_id);

                self.jump_label_id += 1;
                let assembly_code = format!(
                    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@{true_jump_label}\nD;{operation_jmp_directive}\n@SP\nA=M\nM=0\n@{end_jump_label}\n0;JMP\n({true_jump_label})\n@SP\nA=M\nM=-1\n({end_jump_label})\n@SP\nM=M+1"
                );
                return assembly_code;
            }
            _ => panic!("dunno yet"),
        }
    }

    pub fn write_push_pop(
        &self,
        segment: Segment,
        index: u32,
        memory_access: MemoryAccess,
    ) -> String {
        match memory_access {
            MemoryAccess::Push => match segment {
                Segment::Constant | Segment::Temp => {
                    let mut base = 0;
                    if segment == Segment::Temp {
                        base = 5;
                    }
                    let assembly_code = format!(
                        "@{index_loc}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                        index_loc = index + base
                    );
                    return assembly_code;
                }
                Segment::Static => {
                    let assembly_code = format!(
                        "@{file_name}.{index}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                        file_name = self.file_name
                    );
                    return assembly_code;
                }
                Segment::Pointer => {
                    if !(0..=1).contains(&index) {
                        panic!("Pointer parameter can only be 0 or 1");
                    }
                    let this_or_that = if index == 0 { "THIS" } else { "THAT" };
                    let assembly_code = format!("@{this_or_that}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                    return assembly_code;
                }
                _ => {
                    let assembly_code = format!(
                        "@{register}\nD=M\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                        register = segment.asm_rep(),
                    );
                    return assembly_code;
                }
            },
            MemoryAccess::Pop => {
                // cannot pop constant
                match segment {
                    Segment::Constant => panic!("Cannot POP to constant!"),
                    Segment::Temp => {
                        let temp_base = 5;
                        let assembly_code = format!(
                            "@SP\nM=M-1\n@SP\nA=M\nD=M\n@{index_loc}\nM=D",
                            index_loc = index + temp_base
                        );
                        return assembly_code;
                    }
                    Segment::Static => {
                        let assembly_code = format!(
                            "@SP\nM=M-1\n@SP\nA=M\nD=M\n@{file_name}.{index}\nM=D",
                            file_name = self.file_name
                        );
                        return assembly_code;
                    }
                    Segment::Pointer => {
                        if !(0..=1).contains(&index) {
                            panic!("Pointer parameter can only be 0 or 1");
                        }
                        let this_or_that = if index == 0 { "THIS" } else { "THAT" };
                        let assembly_code =
                            format!("@SP\nM=M-1\n@SP\nA=M\nD=M\n@{this_or_that}\nM=D");
                        return assembly_code;
                    }
                    _ => {
                        let assembly_code = format!(
                            "@{index}\nD=A\n@{register}\nD=D+M\n@R13\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D",
                            register = segment.asm_rep()
                        );
                        return assembly_code;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_push_local_to_assembly() {
        let result = r#"@LCL
D=M
@2
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1"#
            .to_string();
        let command = CommandType::Push {
            segment: Segment::Local,
            index: 2,
        };
        let mut coder = CodeWriter::new("basictest");
        assert_eq!(result, coder.command_to_assembly(command))
    }

    #[test]
    fn test_write_push_arg_to_assembly() {
        let result = r#"@ARG
D=M
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1"#
            .to_string();
        let command = CommandType::Push {
            segment: Segment::Argument,
            index: 3,
        };
        let mut coder = CodeWriter::new("basictest");
        assert_eq!(result, coder.command_to_assembly(command))
    }
}
