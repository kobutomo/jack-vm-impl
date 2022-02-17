#[derive(Debug)]
pub enum ArithmeticIns {
    Add = 0,
    Sub = 1,
    Neg = 2,
}

#[derive(Debug)]
pub enum LogicalIns {
    Eq = 0,
    Gt = 1,
    Lt = 2,
    And = 3,
    Or = 4,
    Not = 5,
}
#[derive(Debug)]
pub enum Instruction {
    ArithmeticIns(ArithmeticIns),
    LogicalIns(LogicalIns),
    Push { arg1: String, arg2: String },
    Pop { arg1: String, arg2: String },
}

pub fn translate(instruction: Instruction) -> String {
    let mut ret = String::new();
    match instruction {
        // (add | sub | neg) x y
        Instruction::ArithmeticIns(a_ins) => {
            // y POP して D に代入
            ret += "@SP\nM=M-1\nA=M\nD=M\n";
            match a_ins {
                ArithmeticIns::Add | ArithmeticIns::Sub => {
                    // スタックポインタを1減らして A にスタックの先頭アドレスを代入(x の位置)
                    ret += "@SP\nM=M-1\n@SP\nA=M\n";
                    match a_ins {
                        // x = x + y
                        ArithmeticIns::Add => ret += "M=D+M\n",
                        // x = y - x
                        ArithmeticIns::Sub => ret += "M=M-D\n",
                        _ => {}
                    }
                }
                // y = -y
                ArithmeticIns::Neg => ret += "M=-D\n",
            }
            // SP++
            ret += "@SP\nM=M+1\n"
        }
        Instruction::Push { arg1, arg2 } => {
            if &arg1[..] == "constant" {
                ret += "@";
                ret += &arg2[..];
                ret += "\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
            };
        }
        _ => {}
    };
    ret
}
