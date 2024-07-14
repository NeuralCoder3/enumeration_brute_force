use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter, Debug, Eq, PartialEq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Div,

    And,
    Xor,
    Or,
    Shr,
    Ule,
    Ugt,
    Uge,
}

#[derive(Clone, EnumIter, Debug, Eq, PartialEq, Hash)]
pub enum UnOp {
    Neg,
    Not,
}

lazy_static! {
    pub static ref CONSTANTS: Vec<i32> = vec![1, 31, 2];
}

// operand = constant or a number representing a previous value
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Operand {
    pub value: i32,
    pub is_constant: bool,
}

// instruction: either binop and 2 operands or unop and 1 operand
// union of binop and unop
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Instruction {
    Binary {
        op: BinOp,
        op1: Operand,
        op2: Operand,
    },
    Unary {
        op: UnOp,
        op1: Operand,
    },
}

pub type Program = Vec<Instruction>;

pub fn execute_instruction(inst: &Instruction, operands: &Vec<i32>) -> i32 {
    match inst {
        Instruction::Binary { op, op1, op2 } => {
            let val1 = if op1.is_constant {
                op1.value
            } else {
                operands[op1.value as usize]
            };
            let val2 = if op2.is_constant {
                op2.value
            } else {
                operands[op2.value as usize]
            };

            let res = match op {
                BinOp::Add => val1 + val2,
                BinOp::Sub => val1 - val2,
                BinOp::And => val1 & val2,
                BinOp::Xor => val1 ^ val2,
                BinOp::Or => val1 | val2,
                BinOp::Shr => val1 >> val2,
                BinOp::Ule => {
                    if val1 <= val2 {
                        1
                    } else {
                        0
                    }
                }
                BinOp::Ugt => {
                    if val1 > val2 {
                        1
                    } else {
                        0
                    }
                }
                BinOp::Uge => {
                    if val1 >= val2 {
                        1
                    } else {
                        0
                    }
                }
                BinOp::Div => {
                    if val2 == 0 {
                        0
                    } else {
                        val1 / val2
                    }
                }
            };

            res
        }
        Instruction::Unary { op, op1 } => {
            let val1 = if op1.is_constant {
                op1.value
            } else {
                operands[op1.value as usize]
            };

            match op {
                UnOp::Neg => -val1,
                UnOp::Not => !val1,
            }
        }
    }
}

pub fn simulate_program(program: &Program, input: (i32, i32)) -> i32 {
    let mut operands = vec![input.0, input.1];
    for inst in program {
        // println!("    {:?}", operands);
        let res = execute_instruction(inst, &operands);
        operands.push(res);
    }
    *operands.last().unwrap()
}

pub fn show_operand(op: &Operand) -> String {
    if op.is_constant {
        format!("{}", op.value)
    } else {
        format!("o{}", op.value)
    }
}

pub fn show_instruction(inst: &Instruction) -> String {
    match inst {
        Instruction::Binary { op, op1, op2 } => {
            format!("{:?}({}, {})", op, show_operand(op1), show_operand(op2))
        }
        Instruction::Unary { op, op1 } => {
            format!("{:?}({})", op, show_operand(op1))
        }
    }
}

pub fn print_program(program: &Program) {
    for inst in program {
        println!("{}", show_instruction(inst));
    }
}

pub fn possible_instructions(depth: usize) -> Vec<Instruction> {
    let mut instructions = vec![];

    for unop in UnOp::iter() {
        // do not try out constants => unop on constant is always constant => useless

        // try out operands
        for i in 0..(depth + 2) {
            instructions.push(Instruction::Unary {
                op: unop.clone(),
                op1: Operand {
                    value: i as i32,
                    is_constant: false,
                },
            });
        }
    }

    for binop in BinOp::iter() {
        let mut operands = vec![];
        // operand, operand
        for i in 0..(depth + 2) {
            for j in 0..(depth + 2) {
                operands.push((
                    Operand {
                        value: i as i32,
                        is_constant: false,
                    },
                    Operand {
                        value: j as i32,
                        is_constant: false,
                    },
                ));
            }
        }
        // operand, constant
        for i in 0..(depth + 2) {
            for constant in CONSTANTS.iter() {
                operands.push((
                    Operand {
                        value: i as i32,
                        is_constant: false,
                    },
                    Operand {
                        value: *constant,
                        is_constant: true,
                    },
                ));
            }
        }
        // constant, operand => useless
        // constant, constant => useless

        for (op1, op2) in operands {
            // for shr only operand, constant
            if binop == BinOp::Shr && !op2.is_constant {
                continue;
            }
            // div by 0
            if binop == BinOp::Div && op2.is_constant && op2.value == 0 {
                continue;
            }
            instructions.push(Instruction::Binary {
                op: binop.clone(),
                op1: op1,
                op2: op2,
            });
        }
    }

    return instructions;
}
