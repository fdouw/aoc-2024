use priority_queue::DoublePriorityQueue;
use std::hash::Hash;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let maze: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = maze
        .iter()
        .enumerate()
        .find_map(|(y, l)| match l.iter().enumerate().find(|e| *e.1 == 'S') {
            Some((x, _)) => Some((x, y)),
            None => None,
        })
        .unwrap();
    println!("{start:?}");

    let state = State {
        x: start.0,
        y: start.1,
        d: 1,
    }; // EAST = 1
    let mut pq = DoublePriorityQueue::new();
    pq.push(state, 0);
    let mut part1 = 0;

    while !pq.is_empty() {
        let (current, score) = pq.pop_min().unwrap();
        if maze[current.y][current.x] == 'E' {
            // Found the finish
            part1 = score;
            break;
        } else if maze[current.y][current.x] == '#' {
            // Don't walk into a wall
            continue;
        } else {
            let nxt = current.step();
            match pq.get(&nxt) {
                Some((_state, priority)) => {
                    if *priority > score + 1 {
                        pq.change_priority(&nxt, score + 1);
                    }
                }
                None => {
                    pq.push(nxt, score + 1);
                }
            };
            let nxt = current.turn(1);
            match pq.get(&nxt) {
                Some((_state, priority)) => {
                    if *priority > score + 1000 {
                        pq.change_priority(&nxt, score + 1000);
                    }
                }
                None => {
                    pq.push(nxt, score + 1000);
                }
            };
            let nxt = current.turn(3);
            match pq.get(&nxt) {
                Some((_state, priority)) => {
                    if *priority > score + 1000 {
                        pq.change_priority(&nxt, score + 1000);
                    }
                }
                None => {
                    pq.push(nxt, score + 1000);
                }
            };
        }
    }

    (part1.to_string(), String::from("<not yet implemented>"))
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    x: usize,
    y: usize,
    d: usize,
}

impl State {
    fn step(&self) -> Self {
        Self {
            x: (self.x as isize + DIRS[self.d].0) as usize,
            y: (self.y as isize + DIRS[self.d].1) as usize,
            d: self.d,
        }
    }
    fn turn(&self, amount: usize) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: (self.d + amount) % 4,
        }
    }
}

#[test]
fn test_part1_small() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    assert_eq!(solve(input.to_string(), false).0, "7036");
}

#[test]
fn test_part1_large() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    assert_eq!(solve(input.to_string(), false).0, "11048");
}
