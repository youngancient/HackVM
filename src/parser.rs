// parses VM commands into lexical elements
// ignores whitespaces and comments

#[derive(Debug, PartialEq)]
pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Temp,
    Pointer,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}
impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "add" => Operation::Add,
            "sub" => Operation::Sub,
            "neg" => Operation::Neg,
            "eq" => Operation::Eq,
            "and" => Operation::And,
            "or" => Operation::Or,
            "not" => Operation::Not,
            "gt" => Operation::Gt,
            "lt" => Operation::Lt,
            _ => panic!("{} is an invalid operation!", value),
        }
    }
}
impl Segment {
    pub fn asm_rep(&self) -> String {
        match self {
            Segment::Argument => "ARG".to_string(),
            Segment::Local => "LCL".to_string(),
            Segment::That => "THAT".to_string(),
            Segment::This => "THIS".to_string(),
            _ => panic!("sorry no rep yet!"),
        }
    }
    pub fn str_rep(&self) -> &str {
        match self {
            Segment::Argument => "argument",
            Segment::Local => "local",
            Segment::That => "that",
            Segment::This => "this",
            Segment::Constant => "constant",
            Segment::Pointer => "pointer",
            Segment::Temp => "temp",
            Segment::Static => "static",
        }
    }
}

impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        match value {
            "local" => Segment::Local,
            "argument" => Segment::Argument,
            "this" => Segment::This,
            "that" => Segment::That,
            "constant" => Segment::Constant,
            "pointer" => Segment::Pointer,
            "static" => Segment::Static,
            "temp" => Segment::Temp,
            _ => panic!("{} is an invalid segment", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Arithmetic {
        operation: Operation,
    },
    Push {
        segment: Segment,
        index: u32,
    },
    Pop {
        segment: Segment,
        index: u32,
    },
    Label {
        name: String,
    },
    Goto {
        label: String,
    },
    If {
        label: String,
    },
    Function {
        name: String,
        no_of_args: u32,
    },
    Return,
    Call {
        function_name: String,
        no_of_args: u32,
    },
}

// takes an instruction as input, returns a command as output
pub fn parse(instruction: String) -> CommandType {
    let elements = instruction.split(' ').collect::<Vec<&str>>();
    if elements.len() == 1 {
        // then the instruction is either Arithmetic or the instruction with just a single stuff
        return CommandType::Arithmetic {
            operation: Operation::from(elements[0]),
        };
    } else {
        if elements[0] == "push" {
            if let Ok(val) = elements[2].parse::<u32>() {
                return CommandType::Push {
                    segment: Segment::from(elements[1]),
                    index: val,
                };
            } else {
                panic!("{} is an invalid index", elements[2]);
            }
        } else if elements[0] == "pop" {
            if let Ok(val) = elements[2].parse::<u32>() {
                return CommandType::Pop {
                    segment: Segment::from(elements[1]),
                    index: val,
                };
            } else {
                panic!("{} is an invalid index", elements[2]);
            }
        } else if elements[0] == "label" {
            return CommandType::Label {
                name: elements[1].to_string(),
            };
        } else if elements[0] == "function" {
            if let Ok(val) = elements[2].parse::<u32>() {
                return CommandType::Function {
                    name: elements[1].to_string(),
                    no_of_args: val,
                };
            } else {
                panic!("{} is an invalid index", elements[2]);
            }
        } else {
            panic!("{} is an invalid command type", elements[0]);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("push constant 10".to_string()),
            CommandType::Push {
                segment: Segment::Constant,
                index: 10
            }
        );
        assert_eq!(
            parse("pop constant 4".to_string()),
            CommandType::Pop {
                segment: Segment::Constant,
                index: 4
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_index() {
        parse("pop constant d".to_string());
    }
}
