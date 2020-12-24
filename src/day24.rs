use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "e" => Self::East,
            "se" => Self::SouthEast,
            "sw" => Self::SouthWest,
            "w" => Self::West,
            "nw" => Self::NorthWest,
            "ne" => Self::NorthEast,
            _ => return Err(()),
        };
        Ok(direction)
    }
}

struct Path {
    directions: Vec<Direction>,
}

impl Path {
    fn new(directions: Vec<Direction>) -> Self {
        Self { directions }
    }

    fn destination(&self, start: Coordinate) -> Coordinate {
        self.directions
            .iter()
            .fold(start, |end, &direction| end.step(direction))
    }
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(
            instructions<&[u8], Vec<&[u8]>>,
            many0!(
                complete!(
                    alt!(
                        tag!("e")
                        | tag!("se")
                        | tag!("sw")
                        | tag!("w")
                        | tag!("nw")
                        | tag!("ne")))));
        let (_leftover, parsed) = instructions(s.as_bytes()).map_err(|_| ())?;
        let directions = parsed
            .iter()
            .map(|direction| std::str::from_utf8(direction).unwrap().parse().unwrap())
            .collect();
        let path = Path::new(directions);
        Ok(path)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::East => (1, 0),
            Direction::SouthEast => (1, -1),
            Direction::SouthWest => (0, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, 1),
            Direction::NorthEast => (0, 1),
        };
        Self::new(self.x + dx, self.y + dy)
    }

    fn neighbours(&self) -> Vec<Self> {
        Direction::all()
            .into_iter()
            .map(|direction| self.step(direction))
            .collect()
    }
}

#[derive(Default)]
struct Floor {
    black_tiles: HashSet<Coordinate>,
}

impl Floor {
    fn flip(&mut self, coordinate: Coordinate) {
        if !self.black_tiles.remove(&coordinate) {
            self.black_tiles.insert(coordinate);
        }
    }

    fn count_black(&self) -> usize {
        self.black_tiles.len()
    }

    fn evolve(&mut self) {
        let mut black_neighbours = HashMap::new();
        for coord in &self.black_tiles {
            for neighbour in coord.neighbours() {
                let entry = black_neighbours.entry(neighbour).or_insert(0);
                *entry += 1;
            }
        }

        self.black_tiles = black_neighbours
            .into_iter()
            .filter(|(coord, count)| {
                *count == 2 || (*count == 1 && self.black_tiles.contains(coord))
            })
            .map(|(coord, _count)| coord)
            .collect();
    }
}

pub(crate) fn day24() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();
    let instructions: Vec<Path> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut floor = Floor::default();
    for path in &instructions {
        let start = Coordinate::new(0, 0);
        let end = path.destination(start);
        floor.flip(end);
    }
    let black = floor.count_black();
    println!("Part one answer is {}", black);

    for _day in 0..100 {
        floor.evolve();
    }
    let black = floor.count_black();
    println!("Part two answer is {}", black);
}
