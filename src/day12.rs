use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_degrees(&self) -> isize {
        match self {
            Self::North => 0,
            Self::East => 90,
            Self::South => 180,
            Self::West => 270,
        }
    }

    fn from_degrees(degrees: isize) -> Self {
        match degrees {
            0 => Self::North,
            90 => Self::East,
            180 => Self::South,
            270 => Self::West,
            _ => panic!("Invalid bearing!"),
        }
    }

    fn add_degrees(&self, turn: isize) -> Self {
        let start = self.to_degrees();
        let end = (start + turn + 360) % 360;
        Self::from_degrees(end)
    }
}

enum Instruction {
    Direction(Direction, isize),
    Forward(isize),
    Right(isize),
    Left(isize),
}

#[derive(Debug)]
struct ParseInstructionError;

impl From<std::num::ParseIntError> for ParseInstructionError {
    fn from(_err: std::num::ParseIntError) -> Self {
        ParseInstructionError
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = &s[..1];
        let amount: isize = s[1..].parse()?;
        let instruction = match action {
            "N" => Self::Direction(Direction::North, amount),
            "E" => Self::Direction(Direction::East, amount),
            "S" => Self::Direction(Direction::South, amount),
            "W" => Self::Direction(Direction::West, amount),
            "F" => Self::Forward(amount),
            "R" => Self::Right(amount),
            "L" => Self::Left(amount),
            _ => return Err(ParseInstructionError),
        };
        Ok(instruction)
    }
}

#[derive(Default)]
struct Position {
    east: isize,
    north: isize,
}

impl Position {
    fn new(east: isize, north: isize) -> Self {
        Self { east, north }
    }

    fn manhattan(&self) -> isize {
        self.east.abs() + self.north.abs()
    }

    fn step(&self, east: isize, north: isize) -> Self {
        Self::new(self.east + east, self.north + north)
    }

    fn move_towards(&self, direction: Direction, amount: isize) -> Self {
        let (east, north) = match direction {
            Direction::North => (0, amount),
            Direction::East => (amount, 0),
            Direction::South => (0, -amount),
            Direction::West => (-amount, 0),
        };
        self.step(east, north)
    }

    fn rotate(&self, amount: isize) -> Self {
        match amount {
            0 => Self::new(self.east, self.north),
            90 => Self::new(self.north, -self.east),
            180 => Self::new(-self.east, -self.north),
            270 => Self::new(-self.north, self.east),
            _ => panic!("bad turn {}", amount),
        }
    }
}

struct Ship {
    position: Position,
    direction: Direction,
    waypoint: Position,
}

impl Ship {
    fn new(position: Position, direction: Direction, waypoint: Position) -> Self {
        Self {
            position,
            direction,
            waypoint,
        }
    }

    fn act(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Direction(direction, amount) => {
                self.position = self.position.move_towards(*direction, *amount)
            }
            Instruction::Forward(amount) => {
                self.position = self.position.move_towards(self.direction, *amount)
            }
            Instruction::Right(amount) => self.direction = self.direction.add_degrees(*amount),
            Instruction::Left(amount) => self.direction = self.direction.add_degrees(-amount),
        }
    }

    fn act2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Direction(direction, amount) => {
                self.waypoint = self.waypoint.move_towards(*direction, *amount)
            }
            Instruction::Forward(amount) => {
                self.position = self
                    .position
                    .step(self.waypoint.east * amount, self.waypoint.north * amount)
            }
            Instruction::Right(amount) => self.waypoint = self.waypoint.rotate(*amount),
            Instruction::Left(amount) => self.waypoint = self.waypoint.rotate(360 - amount),
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        let origin = Position::default();
        let east = Direction::East;
        let waypoint = Position::new(10, 1);
        Self::new(origin, east, waypoint)
    }
}

pub(crate) fn day12() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut ship = Ship::default();
    for instruction in &instructions {
        ship.act(instruction);
    }
    println!("Part one answer is {}", ship.position.manhattan());

    let mut ship = Ship::default();
    for instruction in &instructions {
        ship.act2(instruction);
    }
    println!("Part two answer is {}", ship.position.manhattan());
}
