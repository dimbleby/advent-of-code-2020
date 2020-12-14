use std::collections::HashMap;
use std::str::FromStr;

enum Instruction {
    Mask(u64, u64, String),
    Set(u64, u64),
}

#[derive(Debug)]
struct ParseInstructionError;

impl From<std::num::ParseIntError> for ParseInstructionError {
    fn from(_err: std::num::ParseIntError) -> Self {
        ParseInstructionError
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = if let Some(mask) = s.strip_prefix("mask = ") {
            let onestring = mask.replace("X", "0");
            let ones = u64::from_str_radix(&onestring, 2)?;

            let zerostring = mask.replace("X", "1");
            let zeros = u64::from_str_radix(&zerostring, 2)?;

            let floating = mask.replace("1", "0");
            Instruction::Mask(ones, zeros, floating)
        } else {
            let mut words = s[4..].split("] = ");
            let memory: u64 = words.next().ok_or(ParseInstructionError)?.parse()?;
            let value: u64 = words.next().ok_or(ParseInstructionError)?.parse()?;
            Instruction::Set(memory, value)
        };
        Ok(instruction)
    }
}

#[derive(Default)]
struct Computer {
    memory: HashMap<u64, u64>,
    ones_mask: u64,
    zeros_mask: u64,
    floating: String,
}

fn scatter(xs: &str, value: u64) -> Option<(u64, u64)> {
    let indexes: Vec<_> = xs
        .chars()
        .rev()
        .enumerate()
        .filter(|(_n, c)| c == &'X')
        .map(|(n, _c)| n)
        .collect();
    let count = indexes.len();

    if value >= 1 << count {
        return None;
    }

    let mut ones = 0;
    let mut zeros = 0;
    for (old_bit, &new_bit) in indexes.iter().enumerate() {
        let onezero = if value & (1 << old_bit) != 0 { 1 } else { 0 };
        ones |= onezero << new_bit;
        zeros |= (1 - onezero) << new_bit;
    }
    Some((ones, zeros))
}

impl Computer {
    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(ones, zeros, _) => {
                self.ones_mask = *ones;
                self.zeros_mask = *zeros;
            }
            Instruction::Set(memory, value) => {
                let write = (value | self.ones_mask) & self.zeros_mask;
                self.memory.insert(*memory, write);
            }
        }
    }

    fn step2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(ones, _, floating) => {
                self.floating = floating.clone();
                self.ones_mask = *ones;
            }
            Instruction::Set(memory, value) => {
                for n in 0.. {
                    if let Some((ones, zeros)) = scatter(&self.floating, n) {
                        let address = (*memory | self.ones_mask | ones) & !zeros;
                        self.memory.insert(address, *value);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub(crate) fn day14() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer = Computer::default();
    for instruction in &instructions {
        computer.step(instruction);
    }
    println!("Part one answer is {}", computer.sum());

    let mut computer = Computer::default();
    for instruction in &instructions {
        computer.step2(instruction);
    }
    println!("Part two answer is {}", computer.sum());
}
