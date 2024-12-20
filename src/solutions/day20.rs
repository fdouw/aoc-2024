use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let maze: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut distances = HashMap::new();

    let mut start = (0, 0);
    'outer: for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if maze[y][x] == 'S' {
                start = (x, y);
                distances.insert((x, y), 0);
                break 'outer;
            }
        }
    }

    // Compute the distance along the path
    let mut stack = vec![(start.0, start.1, 0)];
    while let Some((x, y, d)) = stack.pop() {
        for (nx, ny) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            if maze[ny][nx] != '#' && !distances.contains_key(&(nx, ny)) {
                distances.insert((nx, ny), d + 1);
                stack.push((nx, ny, d + 1));
            }
        }
    }

    // Find cheats (skip borders: we don't want to go out of bounds)
    let mut cheats: Vec<usize> = Vec::new();
    for y in 1..maze.len() - 1 {
        for x in 1..maze[y].len() - 1 {
            if maze[y][x] == '#' {
                if let Some(d1) = distances.get(&(x, y - 1)) {
                    if let Some(d2) = distances.get(&(x, y + 1)) {
                        cheats.push((*d1 as i32).abs_diff(*d2) as usize - 2); // Sub 2, because you step twice through the wall
                    }
                }
                if let Some(d1) = distances.get(&(x - 1, y)) {
                    if let Some(d2) = distances.get(&(x + 1, y)) {
                        cheats.push((*d1 as i32).abs_diff(*d2) as usize - 2); // idem
                    }
                }
            }
        }
    }
    let part1 = cheats.iter().filter(|c| **c >= 100).count();
    // cheats
    //     .iter()
    //     .counts()
    //     .iter()
    //     .sorted()
    //     .for_each(|cheat| println!("{cheat:?}"));

    cheats.clear();
    for ((x, y), d) in distances.iter() {
        for l in 2..=20 {
            for dy in 0..=l {
                let dx = l - dy;
                // Search half the circle, to avoid double counting
                // South East
                if let Some(d2) = distances.get(&(x + dx, y + dy)) {
                    let savings = d.abs_diff(*d2) as usize - l;
                    cheats.push(savings);
                }
                // North East
                if *y > dy && dy > 0 && dx > 0 {
                    if let Some(d2) = distances.get(&(x + dx, y - dy)) {
                        let savings = d.abs_diff(*d2) as usize - l;
                        cheats.push(savings);
                    }
                }
            }
        }
    }
    // cheats
    //     .values()
    //     .filter(|savings| **savings >= 50)
    //     .counts()
    //     .iter()
    //     .sorted()
    //     .for_each(|cheat| println!("{cheat:?}"));
    let part2 = cheats.iter().filter(|savings| **savings >= 100).count();

    (part1.to_string(), part2.to_string())
}

#[test]
fn test_part1() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    solve(input.to_string(), false);
    panic!("haha")
}
