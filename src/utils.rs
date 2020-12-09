// Input numbers must be sorted.
pub(crate) fn find_pair_summing_to(target: u64, numbers: &[u64]) -> Option<(u64, u64)> {
    let mut lo = 0;
    let mut hi = numbers.len() - 1;
    while lo < hi {
        let number1 = numbers[lo];
        let number2 = numbers[hi];
        match (number1 + number2).cmp(&target) {
            std::cmp::Ordering::Equal => return Some((number1, number2)),
            std::cmp::Ordering::Less => lo += 1,
            std::cmp::Ordering::Greater => hi -= 1,
        }
    }
    None
}
