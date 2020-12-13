// Utilities recovered from advent of code 2019...
fn modular_multiplication(modulus: u64, x: u64, y: u64) -> u64 {
    let mut result = 0;
    let mut a = x;
    let mut b = y;
    while b != 0 {
        if (b % 2) == 1 {
            result = (result + a) % modulus;
        };
        a = (a * 2) % modulus;
        b /= 2;
    }
    result
}

fn modular_inverse(modulus: u64, n: u64) -> u64 {
    let mut a = modulus;
    let mut b = n % modulus;
    let mut x0 = 0;
    let mut x1 = 1;

    // Extended Euclidean algorithm for GCD.
    while b != 0 {
        let quotient = a / b;
        let remainder = a % b;
        a = b;
        b = remainder;

        let temp = x1;
        x1 = (x0 + modular_multiplication(modulus, modulus - quotient, x1)) % modulus;
        x0 = temp;
    }
    assert_eq!(a, 1);

    x0
}

pub(crate) fn day13() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let mut lines = input.lines();
    let now: u64 = lines.next().unwrap().parse().unwrap();
    let timetable = lines.next().unwrap();
    let ids: Vec<u64> = timetable
        .split(',')
        .filter_map(|line| line.parse().ok())
        .collect();

    let (wait, id) = ids.iter().map(|id| (id - (now % id), id)).min().unwrap();
    println!("Part one answer is {}", wait * id);

    let congruences: Vec<(u64, u64)> = timetable
        .split(',')
        .enumerate()
        .filter_map(|(t, id)| id.parse().ok().map(|m| ((m - t as u64 % m) % m, m)))
        .collect();

    // Accumulator is the solution so far, and combined modulus so far.
    //
    // If we didn't have all the machinery lying around from last year, this simple thing would
    // work - and the numbers that we're working with are small enough that it's not even slow.
    //
    //  let (solution, _) = congruences.iter().fold((0, 1u64), |(soln, bigm), (a, m)| {
    //      let soln = (soln..).step_by(bigm as usize).find(|n| n % m == *a).unwrap();
    //      (soln, bigm * m)
    //  });
    let (solution, _) = congruences.iter().fold((0, 1), |(soln, bigm), (a, m)| {
        let bigmi = modular_inverse(*m, bigm);
        let need = (m + a - (soln % m)) % m;
        let soln = soln + need * bigm * bigmi;
        let bigm = bigm * m;
        (soln % bigm, bigm)
    });
    println!("Part two answer is {}", solution);
}
