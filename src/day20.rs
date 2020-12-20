use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
struct Picture {
    // (0, 0) in the top left.
    pixels: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Picture {
    fn new(pixels: HashSet<(usize, usize)>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    fn rotated(&self) -> Self {
        let pixels = self
            .pixels
            .iter()
            .map(|(x, y)| (self.height - 1 - y, *x))
            .collect();
        Picture::new(pixels, self.height, self.width)
    }

    fn flipped(&self) -> Self {
        let pixels = self
            .pixels
            .iter()
            .map(|(x, y)| (self.width - 1 - x, *y))
            .collect();
        Picture::new(pixels, self.width, self.height)
    }

    fn variants(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotated(),
            self.rotated().rotated(),
            self.rotated().rotated().rotated(),
            self.flipped(),
            self.flipped().rotated(),
            self.flipped().rotated().rotated(),
            self.flipped().rotated().rotated().rotated(),
        ]
    }

    fn matches(&self, other: &Picture) -> HashSet<(usize, usize)> {
        let mut used: HashSet<(usize, usize)> = HashSet::new();
        for right_shift in 0..=self.width - other.width {
            for down_shift in 0..=self.height - other.height {
                let targets: HashSet<(usize, usize)> = other
                    .pixels
                    .iter()
                    .map(|(x, y)| (x + right_shift, y + down_shift))
                    .collect();
                if targets.iter().all(|coords| self.pixels.contains(coords)) {
                    used.extend(targets);
                }
            }
        }
        used
    }

    fn row(&self, row: usize) -> String {
        (0..self.width)
            .map(|x| {
                if self.pixels.contains(&(x, row)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect()
    }

    fn column(&self, column: usize) -> String {
        (0..self.height)
            .map(|y| {
                if self.pixels.contains(&(column, y)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect()
    }
}

impl From<&str> for Picture {
    fn from(s: &str) -> Self {
        let mut pixels = HashSet::new();
        for (y, row) in s.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    pixels.insert((x, y));
                };
            }
        }
        let &max_x = pixels.iter().map(|(x, _)| x).max().unwrap();
        let &max_y = pixels.iter().map(|(_, y)| y).max().unwrap();
        Self::new(pixels, max_x + 1, max_y + 1)
    }
}

impl Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            writeln!(f, "{}", self.row(y))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Tile {
    index: usize,
    picture: Picture,
    top: String,
    right: String,
    bottom: String,
    left: String,
}

impl Tile {
    fn new(index: usize, picture: Picture) -> Self {
        let top = picture.row(0);
        let right = picture.column(picture.width - 1);
        let bottom = picture.row(picture.height - 1);
        let left = picture.column(0);
        Self {
            index,
            picture,
            top,
            right,
            bottom,
            left,
        }
    }

    fn variants(&self) -> Vec<Self> {
        let pictures = self.picture.variants();
        pictures
            .into_iter()
            .map(|picture| Tile::new(self.index, picture))
            .collect()
    }
}

impl FromStr for Tile {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let &index = &s[5..9].parse()?;
        let ascii_art = &s[11..];
        let picture = Picture::from(ascii_art);

        let tile = Tile::new(index, picture);
        Ok(tile)
    }
}

#[derive(Default)]
struct Grid {
    // Again, (0, 0) in top left.
    placements: HashMap<(usize, usize), Tile>,
    used: HashSet<usize>,
    first_gap: usize,
}

impl Grid {
    fn fits(&self, (x, y): (usize, usize), tile: &Tile) -> bool {
        // We place tiles in order, so only need to check up and left.
        if y > 0 {
            let top_neighbour = &self.placements[&(x, y - 1)];
            if top_neighbour.bottom != tile.top {
                return false;
            }
        }

        if x > 0 {
            let left_neighbour = &self.placements[&(x - 1, y)];
            if left_neighbour.right != tile.left {
                return false;
            }
        }

        true
    }

    fn place(&mut self, (x, y): (usize, usize), tile: Tile) {
        self.used.insert(tile.index);
        self.placements.insert((x, y), tile);
    }

    fn unplace(&mut self, (x, y): (usize, usize)) {
        let tile = self.placements.remove(&(x, y)).unwrap();
        self.used.remove(&tile.index);
    }

    // It turns out there's never any backtracking once you get the first tile right...
    fn solve(&mut self, tiles: &[Tile]) -> bool {
        let (x, y) = (self.first_gap % 12, self.first_gap / 12);
        let options: Vec<_> = tiles
            .iter()
            .filter(|tile| !self.used.contains(&tile.index) && self.fits((x, y), tile))
            .collect();

        for tile in options {
            self.place((x, y), tile.clone());
            self.first_gap += 1;
            if self.first_gap == 144 {
                // We've filled the grid.
                return true;
            }
            if self.solve(tiles) {
                return true;
            }
            self.first_gap -= 1;
            self.unplace((x, y));
        }
        false
    }

    fn strip_borders(&self) -> Picture {
        let mut pixels: HashSet<(usize, usize)> = HashSet::new();
        let mut y_pixel = 0;
        for y in 0..12 {
            for row in 1..9 {
                let mut x_pixel = 0;
                for x in 0..12 {
                    let tile = &self.placements[&(x, y)];
                    for col in 1..9 {
                        if tile.picture.pixels.contains(&(col, row)) {
                            pixels.insert((x_pixel, y_pixel));
                        }
                        x_pixel += 1;
                    }
                }
                y_pixel += 1;
            }
        }
        Picture::new(pixels, 96, 96)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..12 {
            for row in 0..10 {
                for x in 0..12 {
                    let tile = &self.placements[&(x, y)];
                    write!(f, "{}", tile.picture.row(row))?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub(crate) fn day20() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|block| block.parse::<Tile>().unwrap())
        .map(|tile| tile.variants())
        .flatten()
        .collect();

    let mut grid = Grid::default();
    grid.solve(&tiles);

    let corners = &[(0, 0), (0, 11), (11, 0), (11, 11)];
    let part_one: usize = corners
        .iter()
        .map(|coords| grid.placements[coords].index)
        .product();
    println!("Part one answer is {}", part_one);

    let picture = grid.strip_borders();

    let ascii = "..................#.\n\
                 #....##....##....###\n\
                 .#..#..#..#..#..#...";
    let sea_monster = Picture::from(ascii);
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    for variant in &sea_monster.variants() {
        used.extend(picture.matches(variant));
    }
    let unused = picture.pixels.len() - used.len();
    println!("Part two answer is {}", unused);
}
