pub(crate) fn day10() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let mut numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    numbers.sort_unstable();

    let (ones, threes) =
        numbers
            .windows(2)
            .fold((1, 1), |(os, ts), pair| match pair[1] - pair[0] {
                1 => (os + 1, ts),
                3 => (os, ts + 1),
                _ => (os, ts),
            });
    println!("Part one answer is {}", ones * threes);

    let &max = numbers.last().unwrap();
    let builtin_adapter = max + 3;
    let mut paths = vec![0u64; builtin_adapter + 1];
    paths[builtin_adapter] = 1;
    for &adapter in numbers.iter().rev() {
        paths[adapter] = paths[adapter + 1] + paths[adapter + 2] + paths[adapter + 3];
    }
    println!("Part two answer is {}", paths[1] + paths[2] + paths[3]);
}
