use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => return Err(ParseError),
        };
        Ok(op)
    }
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let op: Operation = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;

        let arg: isize = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;

        let instruction = Self {
            operation: op,
            argument: arg,
        };
        Ok(instruction)
    }
}

struct Computer {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    accumulator: isize,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_pointer = 0;
    }

    fn corrupt(&mut self, position: usize) -> bool {
        let instruction = &mut self.instructions[position];
        match instruction.operation {
            Operation::Nop => {
                instruction.operation = Operation::Jmp;
                true
            }
            Operation::Jmp => {
                instruction.operation = Operation::Nop;
                true
            }
            _ => false,
        }
    }

    fn step(&mut self) {
        let instruction = &self.instructions[self.instruction_pointer];
        match instruction.operation {
            Operation::Nop => self.instruction_pointer += 1,
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.instruction_pointer += 1;
            }
            Operation::Jmp => {
                // yuk
                self.instruction_pointer =
                    (self.instruction_pointer as isize + instruction.argument) as usize
            }
        }
    }

    fn run_to_repeat_point(&mut self) -> isize {
        let mut visited: HashSet<usize> = HashSet::new();
        while visited.insert(self.instruction_pointer) {
            self.step();
        }
        self.accumulator
    }

    fn fix_and_terminate(&mut self) -> isize {
        let mut corrupt_ip: Option<usize> = None;
        let mut reset_acc = 0;

        let program_length = self.instructions.len();
        let mut visited: HashSet<usize> = HashSet::new();

        // We'll execute each instruction at most twice: once on the original program and once on a
        // version where we've fixed a single operation.
        while self.instruction_pointer != program_length {
            if !visited.insert(self.instruction_pointer) && corrupt_ip.is_some() {
                // Found a loop, we've taken a wrong path, back up.
                self.accumulator = reset_acc;
                self.instruction_pointer = corrupt_ip.take().unwrap();
                self.corrupt(self.instruction_pointer);
            } else if corrupt_ip.is_none() && self.corrupt(self.instruction_pointer) {
                // Found a new path to try.
                corrupt_ip = Some(self.instruction_pointer);
                reset_acc = self.accumulator;
            }
            self.step();
        }
        self.accumulator
    }
}

pub(crate) fn day08() {
    let input = std::fs::read_to_string("data/day08.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer = Computer::new(instructions);
    let accumulator = computer.run_to_repeat_point();
    println!("Part one answer is {}", accumulator);

    computer.reset();
    let accumulator = computer.fix_and_terminate();
    println!("Part two answer is {}", accumulator);
}
