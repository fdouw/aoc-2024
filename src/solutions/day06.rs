use std::collections::HashSet;
use std::io::Read;

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

    // Part 1
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

    // Part 2
    let mut obstructions = HashSet::new();
    let mut trace = HashSet::new();
    for block in visited.iter() {
        let mut pos = start.clone();
        trace.clear();
        obstacles[block.1 as usize][block.0 as usize] = true;
        loop {
            match pos.jump(width as isize, height as isize, &obstacles) {
                None => break,
                Some(p) => {
                    if trace.contains(&p) {
                        obstructions.insert(block);
                        break;
                    } else {
                        trace.insert(p.clone());
                        let mut next = p.rotate_step();
                        if obstacles[next.y as usize][next.x as usize] {
                            next = p.rotate180_step()
                        }
                        pos = next;
                    }
                }
            }
        }
        obstacles[block.1 as usize][block.0 as usize] = false;
    }
    // show(&obstacles, &obstructions, &start);

    (visited.len().to_string(), obstructions.len().to_string())
}

#[allow(dead_code)]
fn show(obstacles: &Vec<Vec<bool>>, obstructions: &HashSet<&(isize, isize)>, pos: &Position) {
    let obstructs: HashSet<_> = obstructions
        .iter()
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect();
    let guard = (pos.x as usize, pos.y as usize);
    for (y, row) in obstacles.iter().enumerate() {
        for (x, obst) in row.iter().enumerate() {
            if *obst {
                print!("#");
            } else if obstructs.contains(&(x, y)) {
                print!("O");
            } else if (x, y) == guard {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
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
    fn jump(&self, width: isize, height: isize, obstacles: &Vec<Vec<bool>>) -> Option<Position> {
        let mut x = self.x;
        let mut y = self.y;
        let dx = DIR[self.d as usize].0;
        let dy = DIR[self.d as usize].1;
        loop {
            x += dx;
            y += dy;
            if !(0 <= x && x < width && 0 <= y && y < height) {
                return None;
            } else if obstacles[y as usize][x as usize] {
                return Some(Position {
                    x: x - dx,
                    y: y - dy,
                    d: self.d,
                });
            }
            // Keep going
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
