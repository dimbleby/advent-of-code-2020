use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Policy {
    letter: char,

    // Ugh, part two made these names less helpful!
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct ParsePolicyError;

impl From<std::num::ParseIntError> for ParsePolicyError {
    fn from(_err: std::num::ParseIntError) -> Self {
        ParsePolicyError
    }
}

impl FromStr for Policy {
    type Err = ParsePolicyError;

    // Expects input like "3-7 x".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(" ");

        let bounds = words.next().ok_or(ParsePolicyError)?;
        let mut counts = bounds.split("-");
        let min: usize = counts.next().ok_or(ParsePolicyError)?.parse()?;
        let max: usize = counts.next().ok_or(ParsePolicyError)?.parse()?;

        let letter = words
            .next()
            .and_then(|w| w.chars().next())
            .ok_or(ParsePolicyError)?;

        let parsed = Self { letter, min, max };
        Ok(parsed)
    }
}

impl Policy {
    fn matches_part_one(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count();
        self.min <= count && count <= self.max
    }

    fn matches_part_two(&self, password: &str) -> bool {
        let c1 = password
            .chars()
            .nth(self.min - 1)
            .expect("password too short!");
        let c2 = password
            .chars()
            .nth(self.max - 1)
            .expect("password too short!");
        (c1 == self.letter) ^ (c2 == self.letter)
    }
}

// Expects lines like "2-4 n: npct".
fn parse_line(line: &str) -> (Policy, String) {
    let mut parts = line.split(": ");
    let policy: Policy = parts.next().expect("no policy!").parse().expect("bad policy!");
    let password = parts.next().expect("no password!");
    (policy, password.to_owned())
}

pub(crate) fn day02() {
    let input = File::open("data/day02.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let tries: Vec<(Policy, String)> = buffered
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(&line))
        .collect();

    let valid_count = tries
        .iter()
        .filter(|(policy, password)| policy.matches_part_one(password))
        .count();
    println!("Part one answer is {}", valid_count);

    let valid_count = tries
        .iter()
        .filter(|(policy, password)| policy.matches_part_two(password))
        .count();
    println!("Part two answer is {}", valid_count);
}
