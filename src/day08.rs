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

    fn run_to_completion(&mut self) -> bool {
        let program_length = self.instructions.len();
        let mut visited: HashSet<usize> = HashSet::new();
        while visited.insert(self.instruction_pointer) {
            // Apparently the instruction pointer never goes negative, or further out of bounds.
            if self.instruction_pointer == program_length {
                return true;
            }
            self.step();
        }

        false
    }
}

pub(crate) fn day08() {
    let input = std::fs::read_to_string("data/day08.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer = Computer::new(instructions);
    let terminates = computer.run_to_completion();
    assert!(!terminates);
    println!("Part one answer is {}", computer.accumulator);

    for index in 0..computer.instructions.len() {
        if computer.corrupt(index) {
            computer.reset();
            if computer.run_to_completion() {
                println!("Part two answer is {}", computer.accumulator);
                break;
            }

            // Undo the damage.
            computer.corrupt(index);
        }
    }
}
