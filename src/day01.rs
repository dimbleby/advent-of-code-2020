use crate::utils::find_pair_summing_to;

pub(crate) fn day01() {
    let input = std::fs::read_to_string("data/day01.txt").unwrap();
    let mut expenses: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
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
