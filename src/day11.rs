use std::fmt;
use std::fmt::Display;

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Copy, Clone)]
enum Tile {
    Floor,

    // Is the seat occupied?
    Seat(bool),
}

impl Tile {
    fn occupied(&self) -> bool {
        match self {
            Self::Floor => false,
            Self::Seat(occupied) => *occupied,
        }
    }
}

#[derive(Debug)]
struct ParseTileError;

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Seat(false),
            '#' => Self::Seat(true),
            _ => panic!("bad tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Seat(false) => write!(f, "L"),
            Self::Seat(true) => write!(f, "#"),
        }
    }
}

struct Layout {
    tiles: Vec<Vec<Tile>>,
    columns: usize,
    rows: usize,
}

impl Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Layout {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let columns = tiles[0].len();
        let rows = tiles.len();
        Self {
            tiles,
            columns,
            rows,
        }
    }

    fn occupied_neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        if row > 0 && col > 0 && self.tiles[row - 1][col - 1].occupied() {
            count += 1;
        }
        if row > 0 && self.tiles[row - 1][col].occupied() {
            count += 1;
        }
        if row > 0 && col + 1 < self.columns && self.tiles[row - 1][col + 1].occupied() {
            count += 1;
        }
        if col > 0 && self.tiles[row][col - 1].occupied() {
            count += 1;
        }
        if col + 1 < self.columns && self.tiles[row][col + 1].occupied() {
            count += 1;
        }
        if row + 1 < self.rows && col > 0 && self.tiles[row + 1][col - 1].occupied() {
            count += 1;
        }
        if row + 1 < self.rows && self.tiles[row + 1][col].occupied() {
            count += 1;
        }
        if row + 1 < self.rows && col + 1 < self.columns && self.tiles[row + 1][col + 1].occupied()
        {
            count += 1;
        }
        count
    }

    fn sees_occupied_seat(&self, row: usize, col: usize, direction: &(isize, isize)) -> bool {
        let (mut r, mut c) = (row, col);
        loop {
            if r == 0 && direction.0 < 0 {
                break;
            }
            if r + 1 == self.rows && direction.0 > 0 {
                break;
            }
            if c == 0 && direction.1 < 0 {
                break;
            }
            if c + 1 == self.columns && direction.1 > 0 {
                break;
            }
            r = (r as isize + direction.0) as usize;
            c = (c as isize + direction.1) as usize;
            if let Tile::Seat(occupied) = self.tiles[r][c] {
                return occupied;
            }
        }
        false
    }

    fn occupied_visible(&self, row: usize, col: usize) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&d| self.sees_occupied_seat(row, col, d))
            .count()
    }

    fn step(&mut self) -> bool {
        let mut changed = false;
        let mut new_tiles: Vec<Vec<Tile>> = self.tiles.clone();
        for row in 0..self.rows {
            for col in 0..self.columns {
                let occupied_neighbours = self.occupied_neighbours(row, col);
                match self.tiles[row][col] {
                    Tile::Seat(false) => {
                        if occupied_neighbours == 0 {
                            new_tiles[row][col] = Tile::Seat(true);
                            changed = true;
                        }
                    }
                    Tile::Seat(true) => {
                        if occupied_neighbours >= 4 {
                            new_tiles[row][col] = Tile::Seat(false);
                            changed = true;
                        }
                    }
                    Tile::Floor => {}
                }
            }
        }
        self.tiles = new_tiles;
        changed
    }

    fn step2(&mut self) -> bool {
        let mut changed = false;
        let mut new_tiles: Vec<Vec<Tile>> = self.tiles.clone();
        for row in 0..self.rows {
            for col in 0..self.columns {
                let occupied_visible = self.occupied_visible(row, col);
                match self.tiles[row][col] {
                    Tile::Seat(false) => {
                        if occupied_visible == 0 {
                            new_tiles[row][col] = Tile::Seat(true);
                            changed = true;
                        }
                    }
                    Tile::Seat(true) => {
                        if occupied_visible >= 5 {
                            new_tiles[row][col] = Tile::Seat(false);
                            changed = true;
                        }
                    }
                    Tile::Floor => {}
                }
            }
        }
        self.tiles = new_tiles;
        changed
    }

    fn occupied_count(&self) -> u32 {
        let mut count = 0;
        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.tiles[row][col].occupied() {
                    count += 1;
                }
            }
        }
        count
    }
}

pub(crate) fn day11() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();

    let mut layout = Layout::new(tiles.clone());
    while layout.step() {}
    println!("Part one answer is {}", layout.occupied_count());

    let mut layout = Layout::new(tiles);
    while layout.step2() {}
    println!("Part two answer is {}", layout.occupied_count());
}
