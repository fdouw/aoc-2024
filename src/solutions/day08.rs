use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                '.' => continue,
                _ => antennas.entry(c).or_default().push((x, y)),
            }
        }
    }
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    // Part 1
    let mut antinodes = HashSet::new();
    for (_c, locations) in antennas.iter() {
        for (a, b) in locations.iter().tuple_combinations() {
            let dx = b.0 as isize - a.0 as isize;
            let dy = b.1 as isize - a.1 as isize;
            let x = a.0 as isize - dx;
            let y = a.1 as isize - dy;
            if 0 <= x && x < width && 0 <= y && y < height {
                antinodes.insert((x, y));
            }
            let x = b.0 as isize + dx;
            let y = b.1 as isize + dy;
            if 0 <= x && x < width && 0 <= y && y < height {
                antinodes.insert((x, y));
            }
        }
    }
    let part1 = antinodes.len();

    // Part 2
    antinodes.clear();
    for (_c, locations) in antennas {
        for (a, b) in locations.iter().tuple_combinations() {
            let dx = b.0 as isize - a.0 as isize;
            let dy = b.1 as isize - a.1 as isize;
            let mut x = a.0 as isize;
            let mut y = a.1 as isize;
            while 0 <= x && x < width && 0 <= y && y < height {
                antinodes.insert((x, y));
                x -= dx;
                y -= dy;
            }
            x = b.0 as isize;
            y = b.1 as isize;
            while 0 <= x && x < width && 0 <= y && y < height {
                antinodes.insert((x, y));
                x += dx;
                y += dy;
            }
        }
    }
    let part2 = antinodes.len();

    (part1.to_string(), part2.to_string())
}
