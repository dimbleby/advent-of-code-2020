use std::fmt;
use std::fmt::Display;

fn decrement(max: usize, n: usize) -> usize {
    let mut dec = n - 1;
    if dec == 0 {
        dec = max
    };
    dec
}

struct Cups {
    size: usize,
    current: usize,
    chain: Vec<usize>,
}

impl Cups {
    fn new(input: &[usize], big: bool) -> Self {
        let input_len = input.len();
        let size = if big { 1_000_000 } else { input_len };

        // One too big so that we can 1-index.
        let mut chain = vec![0usize; size + 1];

        for n in 0..input_len {
            let value = input[n];
            let successor = input[(n + 1) % input_len];
            chain[value] = successor;
        }

        let current = input[0];

        if big {
            let &last = input.last().unwrap();
            chain[last] = input_len + 1;
            for (n, next) in chain.iter_mut().enumerate().skip(input_len + 1) {
                *next = n + 1;
            }
            chain[size] = current;
        }

        Self {
            size,
            current,
            chain,
        }
    }

    fn step(&mut self) {
        // Remove the first three cups.
        let a = self.chain[self.current];
        let b = self.chain[a];
        let c = self.chain[b];
        self.chain[self.current] = self.chain[c];

        // Select the destination cup.
        let mut destination = decrement(self.size, self.current);
        while destination == a || destination == b || destination == c {
            destination = decrement(self.size, destination);
        }

        // Insert the removed cups.
        self.chain[c] = self.chain[destination];
        self.chain[destination] = a;

        // Select a new current cup.
        self.current = self.chain[self.current];
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current)?;
        let mut this = self.chain[self.current];
        while this != self.current {
            write!(f, " {}", this)?;
            this = self.chain[this];
        }
        Ok(())
    }
}

pub(crate) fn day23() {
    let input = [2usize, 5, 3, 1, 4, 9, 8, 6, 7];
    let mut cups = Cups::new(&input, false);
    for _ in 0..100 {
        cups.step();
    }

    // Rotate the string yourself...
    println!("Part one answer is (almost) {}", cups);

    let mut cups = Cups::new(&input, true);
    for _ in 0..10_000_000 {
        cups.step();
    }
    let a = cups.chain[1];
    let b = cups.chain[a];
    println!("Part two answer is {}", a * b);
}
