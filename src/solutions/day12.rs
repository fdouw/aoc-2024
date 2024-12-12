use std::collections::HashMap;

use crate::lib_aoc::unionfind::NamedUnionFind;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let north: u8 = 0b1000;
    let east: u8 = 0b0100;
    let south: u8 = 0b0010;
    let west: u8 = 0b0001;

    // Use union find to group
    let mut uf = NamedUnionFind::new();
    let mut grid = Vec::new();
    let mut borders = Vec::new();
    for (y, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        borders.push(Vec::new());
        for (x, c) in line.char_indices() {
            grid[y].push(c);
            borders[y].push(0);
            // Check neighbours
            uf.add((x, y));
            if y == 0 {
                borders[y][x] |= north;
            } else if grid[y - 1][x] == c {
                uf.connect((x, y), (x, y - 1));
            } else {
                borders[y][x] |= north;
                borders[y - 1][x] |= south;
            }
            if x == 0 {
                borders[y][x] |= west;
            } else if grid[y][x - 1] == c {
                uf.connect((x, y), (x - 1, y));
            } else {
                borders[y][x] |= west;
                borders[y][x - 1] |= east;
            }
        }
        // Edge case: we didn't look at the east most border
        borders[y][grid[y].len() - 1] |= east;
    }
    // Edge case: we didn't look at the south most border
    borders[grid.len() - 1].iter_mut().for_each(|b| *b |= south);

    let count_sides = |x: usize, y: usize| -> u32 {
        let mut count = 0;
        let sides = borders[y][x];
        if sides & north == north {
            if sides & west == west || borders[y][x - 1] & north != north {
                count += 1;
            }
        }
        if sides & east == east {
            if sides & north == north || borders[y - 1][x] & east != east {
                count += 1;
            }
        }
        if sides & south == south {
            if sides & east == east || borders[y][x + 1] & south != south {
                count += 1;
            }
        }
        if sides & west == west {
            if sides & south == south || borders[y + 1][x] & west != west {
                count += 1;
            }
        }
        count
    };

    let mut areas = HashMap::<usize, u32>::new();
    let mut fences = HashMap::new();
    let mut sides = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let id = uf.get_id((x, y)).unwrap();
            *fences.entry(id).or_default() += borders[y][x].count_ones();
            *sides.entry(id).or_default() += count_sides(x, y);
            *areas.entry(id).or_default() += 1;
        }
    }
    let part1: u32 = areas
        .iter()
        .map(|(id, area)| area * fences.get(id).unwrap())
        .sum();
    let part2: u32 = areas
        .iter()
        .map(|(id, area)| area * sides.get(id).unwrap())
        .sum();

    (part1.to_string(), part2.to_string())
}
