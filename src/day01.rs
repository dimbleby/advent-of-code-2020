use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let input = File::open("data/day01.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let mut expenses: Vec<u32> = buffered
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    expenses.sort_unstable();

    let (expense1, expense2) = find_pair_summing_to(2020, &expenses).expect("No solution!");
    let product = expense1 * expense2;
    println!("Part one answer is {}", product);

    for (index, expense1) in expenses.iter().enumerate() {
        let remainder = 2020 - expense1;
        let tail = &expenses[index + 1..];
        if let Some((expense2, expense3)) = find_pair_summing_to(remainder, tail) {
            let product = expense1 * expense2 * expense3;
            println!("Part two answer is {}", product);
            break;
        }
    }
}

// Input expenses must be sorted.
fn find_pair_summing_to(target: u32, expenses: &[u32]) -> Option<(u32, u32)> {
    let mut lower = 0;
    let mut upper = expenses.len() - 1;
    while lower < upper {
        let expense1 = expenses[lower];
        let expense2 = expenses[upper];
        match expense1 + expense2 {
            total if total == target => return Some((expense1, expense2)),
            total if total < target => lower += 1,
            _ => upper -= 1,
        }
    }
    None
}
