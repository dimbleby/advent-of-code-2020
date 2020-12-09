use crate::utils::find_pair_summing_to;

const WINDOW_SIZE: usize = 25;

pub(crate) fn day09() {
    let input = std::fs::read_to_string("data/day09.txt").unwrap();
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut target = 0;
    for (window_start, window) in numbers.windows(WINDOW_SIZE).enumerate() {
        let mut sorted = window.to_vec();
        sorted.sort_unstable();
        target = numbers[window_start + WINDOW_SIZE];
        if find_pair_summing_to(target, &sorted).is_none() {
            println!("Part one answer is {}", target);
            break;
        }
    }

    let run = find_run_summing_to(target, &numbers).unwrap();
    let min = run.iter().min().unwrap();
    let max = run.iter().max().unwrap();
    println!("Part two answer is {}", min + max);
}

fn find_run_summing_to(target: u64, numbers: &[u64]) -> Option<&[u64]> {
    let max_idx = numbers.len();
    let mut lo = 0;
    let mut hi = 0;
    let mut total = 0;
    loop {
        if total == target {
            return Some(&numbers[lo..hi]);
        }

        if total > target {
            total -= numbers[lo];
            lo += 1;
        } else if hi < max_idx {
            total += numbers[hi];
            hi += 1;
        } else {
            return None;
        }
    }
}
