use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Tree,
    Space,
}

impl From<char> for Tile {
    fn from(x: char) -> Self {
        match x {
            '#' => Self::Tree,
            '.' => Self::Space,
            _ => panic!("bad tile"),
        }
    }
}

type Row = Vec<Tile>;
type Map = Vec<Row>;

pub(crate) fn day03() {
    let input = File::open("data/day03.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let map: Map = buffered
        .lines()
        .map(|line| line.unwrap().chars().map(Tile::from).collect())
        .collect();

    let part1 = count_trees_on_slope(&map, 3, 1);
    println!("Part one answer is {}", part1);

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2: u32 = slopes
        .iter()
        .map(|(right, down)| count_trees_on_slope(&map, *right, *down))
        .product();
    println!("Part two answer is {}", part2);
}

#[allow(clippy::ptr_arg)]
fn count_trees_on_slope(map: &Map, right: usize, down: usize) -> u32 {
    let mut tree_count = 0;

    let num_rows = map.len();
    let num_columns = map[0].len();

    let mut row = 0;
    let mut column = 0;
    while row < num_rows {
        if map[row][column] == Tile::Tree {
            tree_count += 1;
        }
        column = (column + right) % num_columns;
        row += down;
    }
    tree_count
}
