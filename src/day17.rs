use std::collections::HashSet;

const STEPS: &[isize] = &[-1, 0, 1];

type Point = (isize, isize, isize, isize);

fn neighbours((x, y, z, w): Point) -> HashSet<Point> {
    STEPS
        .iter()
        .flat_map(|dx| {
            STEPS
                .iter()
                .flat_map(move |dy| STEPS.iter().map(move |dz| (*dx, *dy, *dz)))
        })
        .filter(|(dx, dy, dz)| *dx != 0 || *dy != 0 || *dz != 0)
        .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz, w))
        .collect()
}

fn neighbours2((x, y, z, w): Point) -> HashSet<Point> {
    STEPS
        .iter()
        .flat_map(|dx| {
            STEPS.iter().flat_map(move |dy| {
                STEPS
                    .iter()
                    .flat_map(move |dz| STEPS.iter().map(move |dw| (*dx, *dy, *dz, *dw)))
            })
        })
        .filter(|(dx, dy, dz, dw)| *dx != 0 || *dy != 0 || *dz != 0 || *dw != 0)
        .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
        .collect()
}

#[derive(Clone, Default)]
struct World {
    // We only remember the active cubes.
    cubes: HashSet<Point>,
}

impl World {
    fn insert(&mut self, point: Point) {
        self.cubes.insert(point);
    }

    fn active_neighbours(&self, point: Point, hyper: bool) -> usize {
        let neighbours = if hyper {
            neighbours2(point)
        } else {
            neighbours(point)
        };
        neighbours.intersection(&self.cubes).count()
    }

    fn evolve(&mut self, hyper: bool) {
        let points: HashSet<Point> = self
            .cubes
            .iter()
            .flat_map(|&point| {
                if hyper {
                    neighbours2(point)
                } else {
                    neighbours(point)
                }
            })
            .collect();
        let mut cubes: HashSet<Point> = HashSet::new();
        for point in points {
            let active_neighbours = self.active_neighbours(point, hyper);
            if active_neighbours == 3 || (active_neighbours == 2 && self.cubes.contains(&point)) {
                cubes.insert(point);
            }
        }
        self.cubes = cubes;
    }

    fn active_count(&self) -> usize {
        self.cubes.len()
    }
}

pub(crate) fn day17() {
    let input = std::fs::read_to_string("data/day17.txt").unwrap();
    let mut world = World::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point: Point = (x as isize, y as isize, 0, 0);
            if c == '#' {
                world.insert(point)
            }
        }
    }

    let mut world1 = world.clone();
    for _cycle in 0..6 {
        world1.evolve(false);
    }
    println!("Part one answer is {}", world1.active_count());

    for _cycle in 0..6 {
        world.evolve(true);
    }
    println!("Part two answer is {}", world.active_count());
}
