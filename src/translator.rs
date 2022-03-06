#[derive(Debug)]
pub struct Translator {
    eq_count: i16,
    lt_count: i16,
    gt_count: i16,
    call_count: i16,
}

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
pub enum FlowIns {
    Label = 0,
    Goto = 1,
    IfGoto = 2,
}

#[derive(Debug)]
pub enum Instruction {
    ArithmeticIns(ArithmeticIns),
    LogicalIns(LogicalIns),
    Push(ArgsWithTwo),
    Pop(ArgsWithTwo),
    FlowIns(FlowIns, String),
    DefFunc(String, i16),
    CallFunc(String, i16),
    RetFunc,
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

impl Translator {
    pub fn new() -> Translator {
        Translator {
            eq_count: 0,
            gt_count: 0,
            lt_count: 0,
            call_count: 0,
        }
    }

    pub fn generate_boostrap(&self) -> String {
        let mut ret = String::new();
        ret += "@261\n";
        ret += "D=A\n";
        ret += "@SP\n";
        ret += "M=D\n";
        ret += "@Sys.init\n";
        ret += "0;JMP\n";
        ret
    }

    pub fn translate(&mut self, instruction: Instruction, filename: &str) -> String {
        let filename = filename.split(".").nth(0).unwrap();
        match instruction {
            // (add | sub | neg) x y
            Instruction::ArithmeticIns(a_ins) => Self::translate_arithmetic_ins(a_ins),
            Instruction::LogicalIns(l_ins) => Self::translate_logical_ins(self, l_ins, filename),
            Instruction::Push(args) => Self::translate_push(args, filename),
            Instruction::Pop(args) => Self::translate_pop(args, filename),
            Instruction::FlowIns(ins, arg) => Self::translate_flow_ins(ins, arg),
            Instruction::DefFunc(name, n_local) => Self::translate_def_func(name, n_local),
            Instruction::CallFunc(name, n_arg) => Self::translate_call_func(self, name, n_arg),
            Instruction::RetFunc => Self::translate_return_func(),
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

    fn translate_logical_ins(&mut self, ins: LogicalIns, filename: &str) -> String {
        let mut ret = String::new();
        match ins {
            LogicalIns::Eq | LogicalIns::Gt | LogicalIns::Lt => {
                let label_suffix: String;
                match ins {
                    LogicalIns::Eq => {
                        self.eq_count += 1;
                        label_suffix = filename.to_owned() + ".EQ" + &self.eq_count.to_string()[..]
                    }
                    LogicalIns::Gt => {
                        self.gt_count += 1;
                        label_suffix = filename.to_owned() + ".GT" + &self.gt_count.to_string()[..]
                    }
                    LogicalIns::Lt => {
                        self.lt_count += 1;
                        label_suffix = filename.to_owned() + ".LT" + &self.lt_count.to_string()[..]
                    }
                    _ => return String::new(),
                }
                let true_label = "TRUE".to_owned() + &label_suffix[..];
                let end_label = "END".to_owned() + &label_suffix[..];
                // pop D
                ret += POP_STACK;
                ret += "@SP\n";
                ret += "M=M-1\n";
                ret += "A=M\n";
                ret += "D=M-D\n";
                ret = ret + "@" + &true_label[..] + "\n";
                match ins {
                    LogicalIns::Eq => ret += "D;JEQ\n",
                    LogicalIns::Gt => ret += "D;JGT\n",
                    LogicalIns::Lt => ret += "D;JLT\n",
                    _ => {}
                }
                // false
                ret += "D=0\n";
                ret += PUSH_STACK;
                ret = ret + "@" + &end_label[..] + "\n";
                ret += "0;JMP\n";
                // true
                ret = ret + "(" + &true_label[..] + ")\n";
                ret += "D=-1\n";
                ret += PUSH_STACK;
                ret = ret + "(" + &end_label[..] + ")\n";
            }
            LogicalIns::And | LogicalIns::Or => {
                // pop D
                ret += POP_STACK;
                ret += "@SP\n";
                ret += "M=M-1\n";
                ret += "A=M\n";
                match ins {
                    LogicalIns::And => ret += "D=D&M\n",
                    LogicalIns::Or => ret += "D=D|M\n",
                    _ => {}
                }
                ret += PUSH_STACK;
            }
            LogicalIns::Not => {
                // pop D
                ret += POP_STACK;
                ret += "D=!D\n";
                ret += PUSH_STACK;
            }
        }
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
            "local" => Self::push_segment(args.arg2, "LCL"),
            "argument" => Self::push_segment(args.arg2, "ARG"),
            "this" => Self::push_segment(args.arg2, "THIS"),
            "that" => Self::push_segment(args.arg2, "THAT"),
            "pointer" => Self::push_reg(args.arg2, "3"),
            "temp" => Self::push_reg(args.arg2, "5"),
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
            "local" => Self::pop_segment(args.arg2, "LCL"),
            "argument" => Self::pop_segment(args.arg2, "ARG"),
            "this" => Self::pop_segment(args.arg2, "THIS"),
            "that" => Self::pop_segment(args.arg2, "THAT"),
            "pointer" => Self::pop_reg(args.arg2, "3"),
            "temp" => Self::pop_reg(args.arg2, "5"),
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

    fn translate_flow_ins(ins: FlowIns, arg: String) -> String {
        match ins {
            FlowIns::Label => "(".to_owned() + &arg[..] + ")\n",
            FlowIns::Goto => Self::translate_goto(arg),
            FlowIns::IfGoto => Self::translate_if_goto(arg),
        }
    }

    fn translate_goto(arg: String) -> String {
        let mut ret = String::new();
        ret += "@";
        ret += &arg[..];
        ret += "\n";
        ret += "0;JMP\n";
        ret
    }

    fn translate_if_goto(arg: String) -> String {
        let mut ret = String::new();
        ret += POP_STACK;
        ret += "@";
        ret += &arg[..];
        ret += "\n";
        ret += "D;JNE\n";
        ret
    }

    fn translate_def_func(name: String, n_local: i16) -> String {
        let mut ret = String::new();
        ret += &Self::translate_flow_ins(FlowIns::Label, name)[..];
        for _ in 0..n_local {
            ret += "@0\n";
            ret += "D=A\n";
            ret += PUSH_STACK;
        }
        ret
    }

    fn translate_call_func(&mut self, name: String, n_arg: i16) -> String {
        self.call_count += 1;
        let ret_addr = "return_address.".to_owned() + &self.call_count.to_string()[..];
        let mut ret = String::new();
        // push return-address
        ret += "@";
        ret += &ret_addr[..];
        ret += "\n";
        ret += "D=A\n";
        ret += PUSH_STACK;
        // レジスタ退避
        // LCL
        ret += "@LCL\n";
        ret += "D=M\n";
        ret += PUSH_STACK;
        // ARG
        ret += "@ARG\n";
        ret += "D=M\n";
        ret += PUSH_STACK;
        // THIS
        ret += "@THIS\n";
        ret += "D=M\n";
        ret += PUSH_STACK;
        // THAT
        ret += "@THAT\n";
        ret += "D=M\n";
        ret += PUSH_STACK;
        // ARG の設定
        ret += "@";
        ret += &(n_arg + 5).to_string()[..];
        ret += "\n";
        ret += "D=A\n";
        ret += "@SP\n";
        ret += "D=M-D\n";
        ret += "@ARG\n";
        ret += "M=D\n";
        // LCL の設定
        ret += "@SP\n";
        ret += "D=M\n";
        ret += "@LCL\n";
        ret += "M=D\n";
        // jump
        ret = ret + "@" + &name[..] + "\n";
        ret += "0;JMP\n";
        ret += &Self::translate_flow_ins(FlowIns::Label, ret_addr)[..];
        ret
    }

    fn translate_return_func() -> String {
        let mut ret = String::new();
        // LCLの保存
        ret += "@LCL\n";
        ret += "D=M\n";
        ret += "@R13\n";
        ret += "M=D\n";
        // リターンアドレスを保存
        ret += "@5\n";
        ret += "D=A\n";
        ret += "@R13\n";
        // A=LCL-5
        ret += "A=M-D\n";
        ret += "D=M\n";
        ret += "@R14\n";
        ret += "M=D\n";
        // 戻り値の設定
        ret += POP_STACK;
        ret += "@ARG\n";
        ret += "A=M\n";
        ret += "M=D\n";
        // SP の復元
        ret += "D=A+1\n";
        ret += "@SP\n";
        ret += "M=D\n";
        // レジスタ復元
        // THAT
        ret += "@1\n";
        ret += "D=A\n";
        ret += "@R13\n";
        // A=LCL-1
        ret += "A=M-D\n";
        ret += "D=M\n";
        ret += "@THAT\n";
        ret += "M=D\n";
        // THIS
        ret += "@2\n";
        ret += "D=A\n";
        ret += "@R13\n";
        // A=LCL-2
        ret += "A=M-D\n";
        ret += "D=M\n";
        ret += "@THIS\n";
        ret += "M=D\n";
        // ARG
        ret += "@3\n";
        ret += "D=A\n";
        ret += "@R13\n";
        // A=LCL-3
        ret += "A=M-D\n";
        ret += "D=M\n";
        ret += "@ARG\n";
        ret += "M=D\n";
        // LCL
        ret += "@4\n";
        ret += "D=A\n";
        ret += "@R13\n";
        // A=LCL-4
        ret += "A=M-D\n";
        ret += "D=M\n";
        ret += "@LCL\n";
        ret += "M=D\n";
        // goto
        ret += "@R14\n";
        ret += "A=M\n";
        ret += "0;JMP\n";
        ret
    }
}
