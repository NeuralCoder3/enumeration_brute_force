#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, process::exit, time::Instant};
// for random number generation
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter, Debug, PartialEq)]
enum BinOp {
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

#[derive(Clone, EnumIter, Debug)]
enum UnOp {
    Neg,
    Not,
}

lazy_static! {
    static ref CONSTANTS: Vec<i32> = vec![1, 31, 2];
}

// operand = constant or a number representing a previous value
#[derive(Clone, Debug)]
struct Operand {
    value: i32,
    is_constant: bool,
}

// instruction: either binop and 2 operands or unop and 1 operand
// union of binop and unop
#[derive(Clone, Debug)]
enum Instruction {
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

type Program = Vec<Instruction>;


fn execute_instruction(inst: &Instruction, operands: &Vec<i32>) -> i32 {
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

fn simulate_program(program: &Program, input: (i32, i32)) -> i32 {
    let mut operands = vec![input.0, input.1];
    for inst in program {
        // println!("    {:?}", operands);
        let res = execute_instruction(inst, &operands);
        operands.push(res);
    }
    *operands.last().unwrap()
}

fn print_program(program: &Program) {
    fn print_operand(op: &Operand) -> String {
        if op.is_constant {
            format!("{}", op.value)
        } else {
            format!("o{}", op.value)
        }
    }
    for inst in program {
        match inst {
            Instruction::Binary { op, op1, op2 } => {
                println!("{:?}({}, {})", op, print_operand(op1), print_operand(op2));
            }
            Instruction::Unary { op, op1 } => {
                println!("{:?}({})", op, print_operand(op1));
            }
        }
    }
}




static mut programs: Vec<Program> = vec![];
// A simple parser would be easier and shorter

fn generate_programs() {
    unsafe {
        // we reserve the non-constant values 0 and 1 for inputs
        // so 2 would refer to the result of the first operation
        // 3 would refer to the result of the second operation
        // etc.

        // hackers delight

        // P1(x) := turn off the rightmost 1-bit in x
        programs.push(vec![
            // o1 = sub x 1
            // operand 2
            Instruction::Binary {
                op: BinOp::Sub,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: true,
                },
            },
            // res = and x o1
            // operand 3
            Instruction::Binary {
                op: BinOp::And,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 2,
                    is_constant: false,
                },
            },
        ]);

        // P2(x) := check if unsigned x is a power of 2
        programs.push(vec![
            // o1 = add x 1
            // operand 2
            Instruction::Binary {
                op: BinOp::Add,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: true,
                },
            },
            // res = and x o1
            // operand 3
            Instruction::Binary {
                op: BinOp::And,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 2,
                    is_constant: false,
                },
            },
        ]);

        // ...

        // P15(x,y) := ceil of average of two integers without overflow
        programs.push(vec![
            // o1 = or x y
            // operand 2
            Instruction::Binary {
                op: BinOp::Or,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: false,
                },
            },
            // o2 = xor x y
            // operand 3
            Instruction::Binary {
                op: BinOp::Xor,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: false,
                },
            },
            // o3 = shr o2 1
            // operand 4
            Instruction::Binary {
                op: BinOp::Shr,
                op1: Operand {
                    value: 3,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: true,
                },
            },
            // res = sub o1 o3
            // operand 5
            Instruction::Binary {
                op: BinOp::Sub,
                op1: Operand {
                    value: 2,
                    is_constant: false,
                },
                op2: Operand {
                    value: 4,
                    is_constant: false,
                },
            },
        ]);

        // P16(x,y) := max of two integers 
        // Found first correct program after 44 seconds
        // Neg(o0)
        // Add(o1, o2)
        // Shr(o3, 31)
        // Div(o3, o4)
        // Add(o1, o5)
        // Found 100 correct programs after 99 seconds
        programs.push(vec![
            // o2 = xor x y
            Instruction::Binary {
                op: BinOp::Xor,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: false,
                },
            },
            // o3 = uge x y
            Instruction::Binary {
                op: BinOp::Uge,
                op1: Operand {
                    value: 0,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: false,
                },
            },
            // o4 = neg o3 
            Instruction::Unary {
                op: UnOp::Neg,
                op1: Operand {
                    value: 3,
                    is_constant: false,
                },
            },
            // o5 = and o2 o4
            Instruction::Binary {
                op: BinOp::And,
                op1: Operand {
                    value: 2,
                    is_constant: false,
                },
                op2: Operand {
                    value: 4,
                    is_constant: false,
                },
            },
            // res = xor o5 y
            Instruction::Binary {
                op: BinOp::Xor,
                op1: Operand {
                    value: 5,
                    is_constant: false,
                },
                op2: Operand {
                    value: 1,
                    is_constant: false,
                },
            },
        ]);

        // ...

        // P20(x) := next higher unsigned number with the same number of 1-bits
        // programs.push(vec![
        //     // o1 = neg x
        //     // o2 = and x o1
        //     // o3 = add x o2
        //     // o4 = xor x o2
        //     // o5 = shr o4 2
        //     // o6 = div o5 o2
        //     // res = or o6 o3

        //     // for our case, increment each o
        //     // o2 = neg x
        //     Instruction::Unary {
        //         op: UnOp::Neg,
        //         op1: Operand {
        //             value: 0,
        //             is_constant: false,
        //         },
        //     },
        //     // o3 = and x o2
        //     Instruction::Binary {
        //         op: BinOp::And,
        //         op1: Operand {
        //             value: 0,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 2,
        //             is_constant: false,
        //         },
        //     },
        //     // // o4 = add x o3
        //     Instruction::Binary {
        //         op: BinOp::Add,
        //         op1: Operand {
        //             value: 0,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 3,
        //             is_constant: false,
        //         },
        //     },
        //     // // o5 = xor x o3
        //     // Here is an error in the paper
        //     Instruction::Binary {
        //         op: BinOp::Xor,
        //         op1: Operand {
        //             value: 0,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 4,
        //             is_constant: false,
        //         },
        //     },
        //     // // o6 = div o5 o3
        //     Instruction::Binary {
        //         op: BinOp::Div,
        //         op1: Operand {
        //             value: 5,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 3,
        //             is_constant: false,
        //         },
        //     },
        //     // // o7 = shr o5 2
        //     Instruction::Binary {
        //         op: BinOp::Shr,
        //         op1: Operand {
        //             value: 6,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 2,
        //             is_constant: true,
        //         },
        //     },
        //     // // res = or o7 o4
        //     Instruction::Binary {
        //         op: BinOp::Or,
        //         op1: Operand {
        //             value: 7,
        //             is_constant: false,
        //         },
        //         op2: Operand {
        //             value: 4,
        //             is_constant: false,
        //         },
        //     },

        // ]);

    }
}



fn test() {
    // test out one program that was proposed for P15

    // Neg(o0)
    // Not(o0)
    // Sub(o1, o3)
    // Shr(o4, 1)

    // let p = vec![
    //     Instruction::Unary {
    //         op: UnOp::Neg,
    //         op1: Operand {
    //             value: 0,
    //             is_constant: false,
    //         },
    //     },
    //     Instruction::Unary {
    //         op: UnOp::Not,
    //         op1: Operand {
    //             value: 0,
    //             is_constant: false,
    //         },
    //     },
    //     Instruction::Binary {
    //         op: BinOp::Sub,
    //         op1: Operand {
    //             value: 1,
    //             is_constant: false,
    //         },
    //         op2: Operand {
    //             value: 3,
    //             is_constant: false,
    //         },
    //     },
    //     Instruction::Binary {
    //         op: BinOp::Shr,
    //         op1: Operand {
    //             value: 4,
    //             is_constant: false,
    //         },
    //         op2: Operand {
    //             value: 1,
    //             is_constant: true,
    //         },
    //     },
    // ];

    // let p = unsafe { programs.last().unwrap() };

    // Neg(o0)
    // Add(o1, o2)
    // Shr(o3, 31)
    // Div(o3, o4)
    // Add(o1, o5)
    let p = vec![
        Instruction::Unary {
            op: UnOp::Neg,
            op1: Operand {
                value: 0,
                is_constant: false,
            },
        },
        Instruction::Binary {
            op: BinOp::Add,
            op1: Operand {
                value: 1,
                is_constant: false,
            },
            op2: Operand {
                value: 2,
                is_constant: false,
            },
        },
        Instruction::Binary {
            op: BinOp::Shr,
            op1: Operand {
                value: 3,
                is_constant: false,
            },
            op2: Operand {
                value: 31,
                is_constant: true,
            },
        },
        Instruction::Binary {
            op: BinOp::Div,
            op1: Operand {
                value: 3,
                is_constant: false,
            },
            op2: Operand {
                value: 4,
                is_constant: false,
            },
        },
        Instruction::Binary {
            op: BinOp::Add,
            op1: Operand {
                value: 1,
                is_constant: false,
            },
            op2: Operand {
                value: 5,
                is_constant: false,
            },
        },
    ];

    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 0..5 {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        let output = simulate_program(&p, (x, y));

        println!("P({}, {}) = {}", x, y, output);
        // binary output
        println!("P({:b}, {:b}) = {:b}", x, y, output);
        println!();
    }
}
    
const PRINT_DEPTH : usize = 0;

fn main() {
    generate_programs();

    // test();
    // exit(0);

    // let p = unsafe { programs.first().unwrap() };
    let p = unsafe { programs.last().unwrap() };

    let mut rng = rand::rngs::StdRng::seed_from_u64(0);

    // generate count many random inputs
    let count = 100;
    let max_val = 10000;
    let mut inputs = vec![];
    for _ in 0..count {
        let x = rng.gen_range(0..max_val);
        let y = rng.gen_range(0..max_val);
        let output = simulate_program(p, (x, y));
        inputs.push(((x, y), output));
    }

    let program_length = p.len();

    // generate all possible programs
    // filter if the program is correct for all inputs

    fn explore(
        depth: usize,
        program: &mut Program,
        correct_programs: &mut Vec<Program>,
        inputs: &Vec<((i32, i32), i32)>,
        program_length: usize,
        start: &Instant,
    ) {
        if depth == program_length {
            let mut correct = true;
            for ((x, y), expected_output) in inputs {
                let output = simulate_program(program, (*x, *y));
                if output != *expected_output {
                    correct = false;
                    break;
                }
            }
            if correct {
                if correct_programs.is_empty() {
                    println!(
                        "Found first correct program after {} seconds",
                        start.elapsed().as_secs()
                    );
                    print_program(program);
                    println!();
                }
                if correct_programs.len() % 100 == 0 {
                    println!(
                        "Found {} correct programs after {} seconds",
                        correct_programs.len(),
                        start.elapsed().as_secs()
                    );
                }
                correct_programs.push(program.clone());
            }
            return;
        }


        for unop in UnOp::iter() {
            // do not try out constants => unop on constant is always constant => useless

            if depth <= PRINT_DEPTH {
                println!("Trying out unop {:?} at depth {}", unop, depth);
            }

            // try out operands
            for i in 0..(depth + 2) {
                program.push(Instruction::Unary {
                    op: unop.clone(),
                    op1: Operand {
                        value: i as i32,
                        is_constant: false,
                    },
                });
                explore(
                    depth + 1,
                    program,
                    correct_programs,
                    inputs,
                    program_length,
                    start,
                );
                program.pop();
            }
        }

        for binop in BinOp::iter() {
            if depth <= PRINT_DEPTH {
                println!("Trying out binop {:?} at depth {}", binop, depth);
            }

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
                program.push(Instruction::Binary {
                    op: binop.clone(),
                    op1: op1,
                    op2: op2,
                });
                explore(
                    depth + 1,
                    program,
                    correct_programs,
                    inputs,
                    program_length,
                    start,
                );
                program.pop();
            }
        }
    }

    let start = Instant::now();
    let mut correct_programs = vec![];
    let mut program = vec![];
    explore(
        0,
        &mut program,
        &mut correct_programs,
        &inputs,
        program_length,
        &start,
    );

    println!("Found {} correct programs", correct_programs.len());

    // for p in correct_programs {
    //     print_program(&p);
    //     println!();
    // }

}
