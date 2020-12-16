use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

impl From<std::num::ParseIntError> for ParseError {
    fn from(_err: std::num::ParseIntError) -> Self {
        ParseError
    }
}

struct Field {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Field {
    fn new(ranges: Vec<RangeInclusive<u64>>) -> Self {
        Self { ranges }
    }

    fn contains(&self, value: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split(": ");
        let _name = sections.next().ok_or(ParseError)?;

        let mut ranges: Vec<RangeInclusive<u64>> = vec![];
        let text_ranges = sections.next().ok_or(ParseError)?.split(" or ");
        for text_range in text_ranges {
            let mut ends = text_range.split('-');
            let lower: u64 = ends.next().ok_or(ParseError)?.parse()?;
            let upper: u64 = ends.next().ok_or(ParseError)?.parse()?;
            let range = lower..=upper;
            ranges.push(range);
        }

        let field = Self::new(ranges);
        Ok(field)
    }
}

struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn new(values: Vec<u64>) -> Self {
        Self { values }
    }

    fn valid(&self, fields: &[Field]) -> bool {
        self.values
            .iter()
            .all(|value| fields.iter().any(|field| field.contains(*value)))
    }

    fn possible_mappings(&self, fields: &[Field]) -> Vec<HashSet<usize>> {
        self.values
            .iter()
            .map(|value| {
                let mut possibilities = HashSet::new();
                for (index, field) in fields.iter().enumerate() {
                    if field.contains(*value) {
                        possibilities.insert(index);
                    }
                }
                possibilities
            })
            .collect()
    }
}

impl FromStr for Ticket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .map(|word| word.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let ticket = Ticket::new(values);
        Ok(ticket)
    }
}

pub(crate) fn day16() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    let mut sections = input.split("\n\n");
    let fields: Vec<Field> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let my_ticket: Ticket = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let nearby_tickets: Vec<Ticket> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut invalid_total = 0;
    for ticket in &nearby_tickets {
        for value in &ticket.values {
            if !fields.iter().any(|field| field.contains(*value)) {
                invalid_total += value;
            }
        }
    }
    println!("Part one answer is {}", invalid_total);

    let mut valid_tickets = nearby_tickets.iter().filter(|ticket| ticket.valid(&fields));
    let base = valid_tickets.next().unwrap().possible_mappings(&fields);
    let mut possibilities = valid_tickets.fold(base, |so_far, ticket| {
        ticket
            .possible_mappings(&fields)
            .iter()
            .zip(so_far)
            .map(|(this, that)| this.intersection(&that).cloned().collect())
            .collect()
    });

    let mut matching: HashMap<usize, usize> = HashMap::new();
    while matching.len() < my_ticket.values.len() {
        let mut next = 0;
        for (value, candidates) in possibilities.iter().enumerate() {
            if candidates.len() == 1 {
                next = *candidates.iter().next().unwrap();
                matching.insert(next, value);
                break;
            }
        }
        for candidates in possibilities.iter_mut() {
            candidates.remove(&next);
        }
    }

    // "departure" fields are the first six.
    let answer: u64 = (0..6).map(|n| my_ticket.values[matching[&n]]).product();
    println!("Part two answer is {}", answer);
}
