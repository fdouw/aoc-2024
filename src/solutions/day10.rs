use std::collections::HashSet;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut distances: Vec<Vec<_>> = (0..height)
        .map(|y| (0..width).map(|x| (grid[y][x] == 9) as u64).collect())
        .collect();
    let mut destinations: Vec<Vec<_>> = (0..height)
        .map(|_| (0..width).map(|_| HashSet::new()).collect())
        .collect();
    let mut current = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 9 {
                current.insert((x, y));
                destinations[y][x].insert((x, y));
            }
        }
    }
    let mut next = HashSet::new();
    let dirs: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for h in (0..=8).rev() {
        for (x, y) in current {
            let dest = destinations[y][x].clone();
            for (dx, dy) in dirs {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if 0 <= new_x && new_x < width as isize && 0 <= new_y && new_y < height as isize {
                    if grid[new_y as usize][new_x as usize] == h {
                        distances[new_y as usize][new_x as usize] += distances[y][x];
                        destinations[new_y as usize][new_x as usize].extend(&dest);
                        next.insert((new_x as usize, new_y as usize));
                    }
                }
            }
        }
        current = next;
        next = HashSet::new();
    }

    // current contains positions of zeros, which is where the distances and endpoints are stored
    let part1: usize = current
        .iter()
        .map(|(x, y)| destinations[*y][*x].len())
        .sum();
    let part2: u64 = current.iter().map(|(x, y)| distances[*y][*x]).sum();

    (part1.to_string(), part2.to_string())
}
