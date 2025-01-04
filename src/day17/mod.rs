use itertools::Itertools;

use crate::{solver::Solver, utils::generate_benchmark};

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<char> for OpCode {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Adv,
            '1' => Self::Bxl,
            '2' => Self::Bst,
            '3' => Self::Jnz,
            '4' => Self::Bxc,
            '5' => Self::Out,
            '6' => Self::Bdv,
            '7' => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    op_code: OpCode,
    operand: u8,
}

#[derive(Default)]
struct Output(Vec<u8>);

impl Output {
    fn output(&self) -> String {
        self.0
            .iter()
            .map(|v| char::from_digit(*v as u32, 8).unwrap())
            .join(",")
    }

    fn push(&mut self, v: u8) {
        self.0.push(v);
    }
}

#[derive(Clone, Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    program: Vec<Instruction>,
    raw_program: Vec<u8>,
}

impl Computer {
    fn new(file: &str) -> Self {
        let mut lines = file.lines();

        let (_, register_a) = lines.next().unwrap().split_once(": ").unwrap();
        let register_a = register_a.parse().unwrap();

        let (_, register_b) = lines.next().unwrap().split_once(": ").unwrap();
        let register_b = register_b.parse().unwrap();

        let (_, register_c) = lines.next().unwrap().split_once(": ").unwrap();
        let register_c = register_c.parse().unwrap();

        // Ignore empty line.
        lines.next();

        let (_, program) = lines.next().unwrap().split_once(": ").unwrap();
        let raw_program = program.split(',').map(|c| c.parse().unwrap()).collect();
        let program: Vec<_> = program.chars().step_by(2).collect();
        let program = program
            .chunks(2)
            .map(|instruction| {
                let (op_code, operand) = (
                    instruction[0].into(),
                    instruction[1].to_digit(8).unwrap() as u8,
                );
                Instruction { op_code, operand }
            })
            .collect();

        Self {
            register_a,
            register_b,
            register_c,
            program,
            raw_program,
        }
    }

    fn fetch_instruction(&self, instruction_index: usize) -> Option<Instruction> {
        self.program.get(instruction_index).cloned()
    }

    fn run_program(&mut self) -> Option<Output> {
        let mut output = Output::default();
        let mut instruction_index = 0;

        while let Some(Instruction { op_code, operand }) = self.fetch_instruction(instruction_index)
        {
            match op_code {
                OpCode::Adv => {
                    let combo_operand = self.fetch_combo_operand(operand);

                    self.register_a >>= combo_operand;
                }
                OpCode::Bxl => {
                    self.register_b ^= operand as u64;
                }
                OpCode::Bst => {
                    self.register_b = self.fetch_combo_operand(operand) % 8;
                }
                OpCode::Jnz => {
                    if self.register_a != 0 {
                        // We divide by two because we have paired op-codes/operands together.
                        instruction_index = (operand >> 1) as usize;
                        continue;
                    }
                }
                OpCode::Bxc => {
                    self.register_b ^= self.register_c;
                }
                OpCode::Out => {
                    let combo_operand = self.fetch_combo_operand(operand);
                    let out = (combo_operand % 8) as u8;

                    output.push(out);
                }
                OpCode::Bdv => todo!(), // Unused instruction.
                OpCode::Cdv => {
                    let combo_operand = self.fetch_combo_operand(operand);

                    self.register_c = self.register_a >> combo_operand;
                }
            }

            instruction_index += 1;
        }

        Some(output)
    }

    fn fetch_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!("reserved operand"),
            _ => unreachable!("operands are 3 bits"),
        }
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut computer = Computer::new(file);
        println!("Output is: {}", computer.run_program().unwrap().output());
    }

    // Part 2 is not a generic solution. It works specifically for the given program in the input, which is:
    //  1) bst: b = a % 8
    //  2) bxl: b = b ^ 1
    //  3) cdv: c = a >> b
    //  4) bxl: b = b ^ 5
    //  5) bxc: b = b ^ c
    //  6) out: print(b % 8)
    //  7) adv: a = a >> 3
    //  8) jnz: if a == 0 => halt | else => back to 1)
    //
    // We essentially reverse engineer the program. We start with the fact that register A must be
    // 0 at the end of the execution of the program in order for the program to halt. As the previous
    // instruction (7) right-shifted register A by 3 (aka integer division by 8), then we know register A
    // must have been between 0 and 7 inclusively at that point. We can therefore explore what the output
    // in (6) would be for each possible A in [0:7]. We know that the last output of the program must be 0,
    // so any output that is not 0 can be discarded. Otherwise, any value of A in [0:7] that results in a 0
    // output must be further further backtracked. We now check that steps (1) through (6) will generate the right
    // second-to-last output, which in our case is 3. We know that to have made it in the last iteration of the
    // program with register A equal to X in [0:7], the previous iteration had to right-shift register A by 3,
    // so we recursively know that in the previous iteration register A had a value in [X << 3: X << 3 + 7].
    // We repeat this over and over again until we have backtracked all the way to a register A that generates
    // the whole output. We do this exploration using DFS (though BFS would have worked equally), and keeping track
    // of all potential solutions.
    fn solve_part2(file: &str) {
        let computer = Computer::new(file);
        let mut valid_as = Vec::new();

        let mut potential_candidates = vec![(0..8 as u64, computer.raw_program)];
        while let Some((possible_as, mut program)) = potential_candidates.pop() {
            let target_output = program.pop().unwrap();

            for possible_a in possible_as {
                let mut b = possible_a % 8;
                b ^= 1;
                let c = possible_a >> b;
                b ^= 5;
                b ^= c;

                let out = (b % 8) as u8;
                if out != target_output {
                    continue;
                }

                if program.is_empty() {
                    valid_as.push(possible_a);
                    continue;
                }

                // This value of A matches with the current last output, but we have to explore further
                // to make sure the rest of the output can also be generated from it.
                let new_possible_a = possible_a << 3;
                potential_candidates.push((new_possible_a..new_possible_a + 8, program.clone()));
            }
        }

        valid_as.sort();
        println!("Valid values for register A are: {:?}", valid_as);
    }
}

generate_benchmark!(day17);
