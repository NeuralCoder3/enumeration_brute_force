#[macro_use]
extern crate lazy_static;

mod common;
mod programs;

use common::*;
use programs::*;

use rand::{Rng, SeedableRng};
use std::time::Instant;

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

    let instructions = possible_instructions(depth);
    for inst in instructions {
        if depth <= PRINT_DEPTH {
            println!("Trying out instruction {:?} at depth {}", inst, depth);
        }
        program.push(inst);
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

const PRINT_DEPTH: usize = 0;

fn main() {
    let programs = generate_programs();

    // test();
    // exit(0);

    // let p = unsafe { programs.first().unwrap() };
    let p = programs.last().unwrap();

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
