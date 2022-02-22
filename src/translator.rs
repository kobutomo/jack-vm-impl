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
    Push(ArgsWithTwo),
    Pop(ArgsWithTwo),
}

#[derive(Debug)]
pub struct ArgsWithTwo {
    pub arg1: String,
    pub arg2: String,
}

/// 操作後のレジスタ
///
/// A: スタックの先頭アドレス
///
/// D: pop した値
const POP_STACK: &str = "@SP\nM=M-1\nA=M\nD=M\n";
/// 操作後のレジスタ
///
/// A: スタックポインタ
///
/// D: push した値
const PUSH_STACK: &str = "@SP\nA=M\nM=D\n@SP\nM=M+1\n";
/// 操作後のレジスタ
///
/// A: スタックポインタ
const INCREMENT_SP: &str = "@SP\nM=M+1\n";
/// 操作後のレジスタ
///
/// A: スタックポインタ
const DECREMENT_SP: &str = "@SP\nM=M-1\n";
/// A に stack の先頭アドレスを代入
///
/// 操作後のレジスタ
///
/// A: スタックの先頭アドレス
const ADDRESSING_SP: &str = "@SP\nA=M\n";

pub fn translate(instruction: Instruction, filename: &str) -> String {
    let filename = filename.split(".").nth(0).unwrap();
    match instruction {
        // (add | sub | neg) x y
        Instruction::ArithmeticIns(a_ins) => translate_arithmetic_ins(a_ins),
        Instruction::Push(args) => translate_push(args, filename),
        Instruction::Pop(args) => translate_pop(args, filename),
        _ => String::new(),
    }
}

fn translate_arithmetic_ins(ins: ArithmeticIns) -> String {
    // y POP して D に代入
    let mut ret = String::from(POP_STACK);
    match ins {
        ArithmeticIns::Add | ArithmeticIns::Sub => {
            // スタックポインタを1減らして A にスタックの先頭アドレスを代入(x の位置)
            ret += DECREMENT_SP;
            ret += ADDRESSING_SP;
            match ins {
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
    ret += INCREMENT_SP;
    ret
}

fn translate_push(args: ArgsWithTwo, filename: &str) -> String {
    match &args.arg1[..] {
        "constant" => {
            let mut ret = String::new();
            ret += "@";
            ret += &(args.arg2 + "\n")[..];
            ret += "D=A\n";
            ret += PUSH_STACK;
            ret
        }
        "local" => push_segment(args.arg2, "LCL"),
        "argument" => push_segment(args.arg2, "ARG"),
        "this" => push_segment(args.arg2, "THIS"),
        "that" => push_segment(args.arg2, "THAT"),
        "pointer" => push_reg(args.arg2, "3"),
        "temp" => push_reg(args.arg2, "5"),
        "static" => {
            let mut ret = String::new();
            ret += "@";
            ret += filename;
            ret += ".";
            ret += &args.arg2[..];
            ret += "\n";
            ret += "D=M\n";
            ret += PUSH_STACK;
            ret
        }
        _ => String::new(),
    }
}

fn push_segment(index: String, seg: &str) -> String {
    let mut ret = String::new();
    ret += "@";
    ret += &index[..];
    ret += "\n";
    ret += "D=A\n"; // D=index
    ret += "@";
    ret += seg;
    ret += "\n";
    ret += "A=D+M\n"; // A=seg+index
    ret += "D=M\n";
    ret += PUSH_STACK;
    ret
}

fn push_reg(index: String, reg_addr: &str) -> String {
    let mut ret = String::new();
    ret += "@";
    ret += &index[..];
    ret += "\n";
    ret += "D=A\n"; // D = index
    ret += "@";
    ret += reg_addr;
    ret += "\n";
    ret += "A=D+A\n"; // A = reg_addr + index
    ret += "D=M\n";
    ret += PUSH_STACK;
    ret
}

fn translate_pop(args: ArgsWithTwo, filename: &str) -> String {
    match &args.arg1[..] {
        "local" => pop_segment(args.arg2, "LCL"),
        "argument" => pop_segment(args.arg2, "ARG"),
        "this" => pop_segment(args.arg2, "THIS"),
        "that" => pop_segment(args.arg2, "THAT"),
        "pointer" => pop_reg(args.arg2, "3"),
        "temp" => pop_reg(args.arg2, "5"),
        "static" => {
            let mut ret = String::new();
            ret += POP_STACK;
            ret += "@";
            ret += filename;
            ret += ".";
            ret += &args.arg2[..];
            ret += "\n";
            ret += "M=D\n";
            ret
        }
        _ => String::new(),
    }
}

fn pop_segment(index: String, seg: &str) -> String {
    let mut ret = String::new();
    ret += POP_STACK;
    ret += "@R13\n";
    ret += "M=D\n"; // RAM[13] = ポップした値
    ret += "@";
    ret += &index[..];
    ret += "\n";
    ret += "D=A\n"; // D = index
    ret += "@";
    ret += seg;
    ret += "\n";
    ret += "A=D+M\n"; // A = seg + index
    ret += "D=A\n";
    ret += "@R14\n";
    ret += "M=D\n"; // RAM[14] = seg + index
    ret += "@R13\n";
    ret += "D=M\n"; // D = RAM[13]
    ret += "@R14\n";
    ret += "A=M\n"; // A = seg + index
    ret += "M=D\n"; // RAM[seg + index] = ポップした値
    ret
}

fn pop_reg(index: String, reg_addr: &str) -> String {
    let mut ret = String::new();
    ret += POP_STACK;
    ret += "@R13\n";
    ret += "M=D\n"; // RAM[13] = ポップした値
    ret += "@";
    ret += &index[..];
    ret += "\n";
    ret += "D=A\n"; // D = index
    ret += "@";
    ret += reg_addr;
    ret += "\n";
    ret += "A=D+A\n"; // A = reg_addr + index
    ret += "D=A\n";
    ret += "@R14\n";
    ret += "M=D\n"; // RAM[14] = reg_addr + index
    ret += "@R13\n";
    ret += "D=M\n"; // D = RAM[13]
    ret += "@R14\n";
    ret += "A=M\n"; // A = reg_addr + index
    ret += "M=D\n"; // RAM[reg_addr + index] = ポップした値
    ret
}
