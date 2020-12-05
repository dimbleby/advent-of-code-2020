use std::collections::HashSet;

// It's just a binary number with a complicated description...
fn decode_pass(text: &str) -> u32 {
    text.chars().fold(0, |value, c| match c {
        'F' | 'L' => 2 * value,
        'B' | 'R' => 1 + 2 * value,
        _ => panic!("bad pass"),
    })
}

pub(crate) fn day05() {
    let input = std::fs::read_to_string("data/day05.txt").unwrap();
    let seat_ids: HashSet<u32> = input.lines().map(decode_pass).collect();

    let &max_id = seat_ids.iter().max().unwrap();
    println!("Part one answer is {}", max_id);

    let &min_id = seat_ids.iter().min().unwrap();
    let my_seat = (min_id..max_id).find(|id| !seat_ids.contains(id)).unwrap();
    println!("Part two answer is {}", my_seat);
}
