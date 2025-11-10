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
    pub fn new() -> Self {
        CodeWriter {
            file_name: String::new(),
            jump_label_id: 0,
        }
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    pub fn write_init(&mut self) -> String {
        let mut assembly_code = String::from("// Initialization code \n");
        // set the stack pointer , SP, to start from 256
        assembly_code.push_str("@256\n");
        assembly_code.push_str("D=A\n");
        assembly_code.push_str("@SP\n");
        assembly_code.push_str("@M=D\n"); // SP = 256
        // call sys.init
        let init_label_id = self.jump_label_id;
        let init_return_label = format!("Sys.init$ret.{}", init_label_id);
        self.jump_label_id += 1;

        // push return label
        assembly_code.push_str(&format!("// call Sys.init$ret.{}\n", init_label_id));
        assembly_code.push_str(&format!("@{init_return_label}\n"));
        assembly_code.push_str(&format!("D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));

        // Push 0 for LCL, ARG, THIS, THAT
        for _ in 0..4 {
            assembly_code.push_str(&format!("@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));
        }

        // ARG = SP - nArgs - 5 (ARG = SP - 0 - 5)
        // SP is currently 261 (256 + 5 pushes). ARG should be 256.
        assembly_code.push_str("@SP\nD=M\n@5\nD=D-A\n@ARG\nM=D\n"); // ARG = SP - 5

        // LCL = SP
        assembly_code.push_str("@SP\nD=M\n@LCL\nM=D\n"); // LCL = SP

        // goto Sys.init
        assembly_code.push_str("@Sys.init\n0;JMP\n");

        // (return_label)
        assembly_code.push_str(&format!("({init_return_label})\n"));

        assembly_code
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
            CommandType::Label { name } => self.write_label(&name),
            CommandType::Goto { label } => self.write_goto(&label),
            CommandType::If { label } => self.write_if(&label),
            CommandType::Function { name, no_of_args } => self.write_function(&name, no_of_args),
            CommandType::Call {
                function_name,
                no_of_args,
            } => self.write_call(&function_name, no_of_args),
            CommandType::Return => self.write_return(),
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
                    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D{selected_op}M\n@SP\nM=M+1");
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
                        file_name = self.file_name.to_lowercase()
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
                            file_name = self.file_name.to_lowercase()
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

    pub fn write_label(&self, label: &str) -> String {
        // adds a label to the assembly code
        format!("({label})")
    }

    pub fn write_goto(&self, label: &str) -> String {
        // jump unconditionally to a label
        format!("@{label}\n0;JMP")
    }

    pub fn write_if(&self, label: &str) -> String {
        // pops the top most value off the stack to D
        // if D != 0 (i.e if true) jump to label
        format!("@SP\nM=M-1\nA=M\nD=M\n@{label}\nD;JNE")
    }

    // this function does following
    // declares the function label and initializes local variables
    pub fn write_function(&self, function_name: &str, no_of_args: u32) -> String {
        let mut assembly_code = String::new();
        // add function label
        assembly_code.push_str(&format!("({function_name})\n"));
        // push 0 no_of_arg times
        for _ in 0..no_of_args {
            assembly_code.push_str(&format!("@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));
        }
        assembly_code
    }

    // this function does the following
    // pushes the return address (a unique label)
    // pushes the current values of LCL, ARG, THIS, THAT (the old frame)
    // repositions ARG : ARG = SP - no_of_args - 5 (basically shifting it down)
    // jump to the function: goto function_label
    pub fn write_call(&mut self, function_name: &str, no_of_args: u32) -> String {
        // generate a unique return label
        let return_label = format!("{}$ret.{}", function_name, no_of_args);
        self.jump_label_id += 1;

        let mut assembly_code = String::new();
        // push the return label on the stack
        assembly_code.push_str(&format!("@{return_label}\n"));
        assembly_code.push_str(&format!("D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));

        // save the callers frame : LCL, ARG, THIS, THAT
        for register in ["LCL", "ARG", "THIS", "THIS"] {
            assembly_code.push_str(&format!("@{register}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));
        }
        // reposition ARG: ARG = SP - no_of_args - 5
        assembly_code.push_str(&format!("@SP\nD=M\n")); // ARG = SP
        assembly_code.push_str(&format!("@{no_of_args}\nD=D-A\n")); // ARG = SP - no_of_args
        assembly_code.push_str(&format!("@5\nD=D-A\n")); // ARG = SP - no_of_args - 5
        assembly_code.push_str(&format!("@{function_name}\n0;JMP\n")); // jump to function_label

        assembly_code.push_str(&format!("({return_label})"));

        assembly_code
    }
    // the return command tears down the Stack frame and restores the caller's state. It does the following:
    // FRAME = LCL
    // *RET = FRAME - 5
    // *ARG = pop() -> pop result value in Argument0
    // SP = ARG + 1 -> repositions the stack
    // Restore THAT, THIS, ARG, LCL (Restores the caller's segment pointers)
    // goto RET (Jumps back to the caller)

    pub fn write_return(&self) -> String {
        let mut assembly_code = String::from("// return\n");

        // R13 is FRAME, R14 is RET (Return Address)

        // FRAME = LCL (R13 = LCL)
        assembly_code.push_str("@LCL\nD=M\n@R13\nM=D\n"); // R13 (FRAME) = LCL

        // RET = *(FRAME - 5) (R14 = Return Address)
        assembly_code.push_str("@5\nA=D-A\nD=M\n@R14\nM=D\n"); // R14 (RET) = *(LCL-5)

        // *ARG = pop() (Pop return value and place it where ARG points)
        assembly_code.push_str("@SP\nM=M-1\nA=M\nD=M\n"); // D = popped value (return value)
        assembly_code.push_str("@ARG\nA=M\nM=D\n"); // *ARG = D

        // SP = ARG + 1
        assembly_code.push_str("@ARG\nD=M\n@SP\nM=D+1\n"); // SP = ARG + 1

        // Restore THAT, THIS, ARG, LCL
        for (i, reg) in ["THAT", "THIS", "ARG", "LCL"].iter().enumerate() {
            // FRAME - (i + 1)
            assembly_code.push_str(&format!("@R13\nAM=M-1\nD=M\n@{reg}\nM=D\n"));
        }

        // goto RET (Jumps back to the caller's return address)
        assembly_code.push_str("@R14\nA=M\n0;JMP");

        assembly_code
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
        let mut coder = CodeWriter::new();
        coder.set_file_name("basictest");
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
        let mut coder = CodeWriter::new();
        coder.set_file_name("basictest");
        assert_eq!(result, coder.command_to_assembly(command))
    }
}
