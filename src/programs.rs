use crate::common::*;

// A simple parser would be easier and shorter
pub fn generate_programs() -> Vec<Program> {
    let mut programs = vec![];
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

    programs
}