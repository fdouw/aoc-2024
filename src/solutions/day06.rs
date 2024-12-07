use std::collections::HashSet;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut obstacles: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    let height = obstacles.len();
    let width = obstacles[0].len();

    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '^' {
                start = (x, y);
                break;
            }
        }
    }
    let start = Position {
        x: start.0 as isize,
        y: start.1 as isize,
        d: Direction::NORTH,
    };

    let mut pos = start.clone();
    let mut visited = HashSet::new();
    visited.insert((pos.x, pos.y));
    loop {
        let mut next = pos.step();
        if !in_area(&next, width, height) {
            break;
        }
        if obstacles[next.y as usize][next.x as usize] {
            next = pos.rotate_step();
            if obstacles[next.y as usize][next.x as usize] {
                next = pos.rotate180_step()
            }
        }
        visited.insert((next.x, next.y));
        pos = next;
    }
    let mut obstructions = 0;
    let mut trace = HashSet::new();
    for block in visited.iter() {
        let mut pos = start.clone();
        trace.clear();
        trace.insert(pos.clone());
        obstacles[block.1 as usize][block.0 as usize] = true;
        loop {
            let mut next = pos.step();
            if !in_area(&next, width, height) {
                break;
            }
            if obstacles[next.y as usize][next.x as usize] {
                next = pos.rotate_step();
                if obstacles[next.y as usize][next.x as usize] {
                    next = pos.rotate180_step()
                }
            }
            if trace.contains(&next) {
                obstructions += 1;
                break;
            }
            trace.insert(next.clone());
            pos = next;
        }
        obstacles[block.1 as usize][block.0 as usize] = false;
    }

    (visited.len().to_string(), obstructions.to_string())
}

const DIR: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn in_area(pos: &Position, width: usize, height: usize) -> bool {
    0 <= pos.x && 0 <= pos.y && pos.x < width as isize && pos.y < height as isize
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 3,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    d: Direction,
}

impl Position {
    fn step(&self) -> Position {
        Position {
            x: self.x + DIR[self.d as usize].0,
            y: self.y + DIR[self.d as usize].1,
            d: self.d,
        }
    }
    fn rotate_step(&self) -> Position {
        let d = match self.d {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };
        Position {
            x: self.x + DIR[d as usize].0,
            y: self.y + DIR[d as usize].1,
            d,
        }
    }
    fn rotate180_step(&self) -> Position {
        let d = match self.d {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        };
        Position {
            x: self.x + DIR[d as usize].0,
            y: self.y + DIR[d as usize].1,
            d,
        }
    }
}
