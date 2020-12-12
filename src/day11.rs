use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use std::iter::successors;

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

    fn is_seat(&self) -> bool {
        match self {
            Self::Floor => false,
            Self::Seat(_) => true,
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

fn neighbour(
    (row, col): (usize, usize),
    (dr, dc): (isize, isize),
    (max_row, max_col): (usize, usize),
) -> Option<(usize, usize)> {
    let new_row: usize = (row as isize + dr).try_into().ok()?;
    let new_col: usize = (col as isize + dc).try_into().ok()?;
    if new_row < max_row && new_col < max_col {
        Some((new_row, new_col))
    } else {
        None
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

    fn occupied_neighbours(&self, row: usize, col: usize) -> usize {
        let bounds = (self.rows, self.columns);
        DIRECTIONS
            .iter()
            .filter_map(|&step| neighbour((row, col), step, bounds))
            .filter(|&(r, c)| self.tiles[r][c].occupied())
            .count()
    }

    fn line(
        &self,
        start: (usize, usize),
        step: (isize, isize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let bounds = (self.rows, self.columns);
        successors(Some(start), move |&s| neighbour(s, step, bounds)).skip(1)
    }

    fn occupied_visible(&self, row: usize, col: usize) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&&step| {
                self.line((row, col), step)
                    .find(|&(r, c)| self.tiles[r][c].is_seat())
                    .map(|(r, c)| self.tiles[r][c].occupied())
                    .unwrap_or(false)
            })
            .count()
    }

    fn step(&mut self) -> bool {
        let mut changed = false;
        let mut new_tiles: Vec<Vec<Tile>> = self.tiles.clone();
        for (row, new_row) in new_tiles.iter_mut().enumerate() {
            for (col, new_tile) in new_row.iter_mut().enumerate() {
                if !self.tiles[row][col].is_seat() {
                    continue;
                }
                let occupied_neighbours = self.occupied_neighbours(row, col);
                match self.tiles[row][col].occupied() {
                    true => {
                        if occupied_neighbours >= 4 {
                            *new_tile = Tile::Seat(false);
                            changed = true;
                        }
                    }
                    false => {
                        if occupied_neighbours == 0 {
                            *new_tile = Tile::Seat(true);
                            changed = true;
                        }
                    }
                }
            }
        }
        self.tiles = new_tiles;
        changed
    }

    fn step2(&mut self) -> bool {
        let mut changed = false;
        let mut new_tiles: Vec<Vec<Tile>> = self.tiles.clone();
        for (row, new_row) in new_tiles.iter_mut().enumerate() {
            for (col, new_tile) in new_row.iter_mut().enumerate() {
                if !self.tiles[row][col].is_seat() {
                    continue;
                }
                let occupied_visible = self.occupied_visible(row, col);
                match self.tiles[row][col].occupied() {
                    true => {
                        if occupied_visible >= 5 {
                            *new_tile = Tile::Seat(false);
                            changed = true;
                        }
                    }
                    false => {
                        if occupied_visible == 0 {
                            *new_tile = Tile::Seat(true);
                            changed = true;
                        }
                    }
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
