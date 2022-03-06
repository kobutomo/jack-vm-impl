use super::translator;

pub fn parse(line: String) -> Option<translator::Instruction> {
    match line.split(" ").nth(0).unwrap() {
        "add" => Some(translator::Instruction::ArithmeticIns(
            translator::ArithmeticIns::Add,
        )),
        "sub" => Some(translator::Instruction::ArithmeticIns(
            translator::ArithmeticIns::Sub,
        )),
        "neg" => Some(translator::Instruction::ArithmeticIns(
            translator::ArithmeticIns::Neg,
        )),
        "eq" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::Eq,
        )),
        "gt" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::Gt,
        )),
        "lt" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::Lt,
        )),
        "and" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::And,
        )),
        "or" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::Or,
        )),
        "not" => Some(translator::Instruction::LogicalIns(
            translator::LogicalIns::Not,
        )),
        "push" => {
            let arg1 = line.split(" ").nth(1).unwrap();
            let arg2 = line.split(" ").nth(2).unwrap();
            Some(translator::Instruction::Push(translator::ArgsWithTwo {
                arg1: arg1.to_owned(),
                arg2: arg2.to_owned(),
            }))
        }
        "pop" => {
            let arg1 = line.split(" ").nth(1).unwrap();
            let arg2 = line.split(" ").nth(2).unwrap();
            Some(translator::Instruction::Pop(translator::ArgsWithTwo {
                arg1: arg1.to_owned(),
                arg2: arg2.to_owned(),
            }))
        }
        "label" => {
            let arg = line.split(" ").nth(1).unwrap();
            Some(translator::Instruction::FlowIns(
                translator::FlowIns::Label,
                arg.to_owned(),
            ))
        }
        "goto" => {
            let arg = line.split(" ").nth(1).unwrap();
            Some(translator::Instruction::FlowIns(
                translator::FlowIns::Goto,
                arg.to_owned(),
            ))
        }
        "if-goto" => {
            let arg = line.split(" ").nth(1).unwrap();
            Some(translator::Instruction::FlowIns(
                translator::FlowIns::IfGoto,
                arg.to_owned(),
            ))
        }
        "function" => {
            let arg1 = line.split(" ").nth(1).unwrap();
            let arg2_str = line.split(" ").nth(2).unwrap();
            let arg2: i16;
            match arg2_str.parse::<i16>() {
                Ok(i) => arg2 = i,
                Err(_) => return None,
            };
            Some(translator::Instruction::DefFunc(arg1.to_owned(), arg2))
        }
        "call" => {
            let arg1 = line.split(" ").nth(1).unwrap();
            let arg2_str = line.split(" ").nth(2).unwrap();
            let arg2: i16;
            match arg2_str.parse::<i16>() {
                Ok(i) => arg2 = i,
                Err(_) => return None,
            };
            Some(translator::Instruction::CallFunc(arg1.to_owned(), arg2))
        }
        "return" => Some(translator::Instruction::RetFunc),
        _ => None,
    }
}
