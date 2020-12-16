use std::collections::HashMap;

struct Game {
    seed: Vec<usize>,
}

impl Game {
    fn new(seed: Vec<usize>) -> Self {
        Self { seed }
    }

    fn iter(&self) -> GameIter {
        GameIter::new(self.seed.iter())
    }
}

impl<'a> IntoIterator for &'a Game {
    type Item = usize;
    type IntoIter = GameIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct GameIter<'a> {
    seed: std::slice::Iter<'a, usize>,
    memory: HashMap<usize, usize>,
    previous: Option<usize>,
    position: usize,
}

impl<'a> GameIter<'a> {
    fn new(seed: std::slice::Iter<'a, usize>) -> Self {
        let memory = HashMap::new();
        let previous = None;
        let position = 0;

        Self {
            seed,
            memory,
            previous,
            position,
        }
    }
}

impl<'a> Iterator for GameIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let value = if let Some(v) = self.seed.next() {
            Some(*v)
        } else if let Some(previous) = self.previous {
            let old_position = self.memory.get(&previous).unwrap_or(&self.position);
            Some(self.position - old_position)
        } else {
            None
        };

        if let Some(previous) = self.previous {
            self.memory.insert(previous, self.position);
        }
        self.previous = value;
        self.position += 1;

        value
    }
}

pub(crate) fn day15() {
    let input = vec![9, 6, 0, 10, 18, 2, 1];
    let game = Game::new(input);
    println!("Part one answer is {}", game.iter().nth(2019).unwrap());
    println!(
        "Part two answer is {}",
        game.iter().nth(29_999_999).unwrap()
    );
}
