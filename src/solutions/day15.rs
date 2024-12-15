use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let (grid_data, move_data) = input.splitn(2, "\n\n").collect_tuple().unwrap();
    let mut grid: Vec<Vec<_>> = grid_data.lines().map(|l| l.chars().collect()).collect();
    let part1 = get_gps(&mut grid, move_data);
    grid = grid_data
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    e => panic!("Invalid character in input stream: {e}"),
                })
                .collect()
        })
        .collect();
    let part2 = get_gps(&mut grid, move_data);

    (part1.to_string(), part2.to_string())
}

fn get_gps(grid: &mut Vec<Vec<char>>, moves: &str) -> usize {
    let mut robot = (0, 0);
    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                grid[y][x] = '.';
                robot = (x, y);
                break 'outer;
            }
        }
    }

    for c in moves.replace('\n', "").chars() {
        let (x, y) = match c {
            '^' => (robot.0, robot.1 - 1),
            '>' => (robot.0 + 1, robot.1),
            'v' => (robot.0, robot.1 + 1),
            '<' => (robot.0 - 1, robot.1),
            e => panic!("Unknown direction: {e}"),
        };
        match grid[y][x] {
            '.' => {
                robot = (x, y);
            }
            '#' => {
                // Wall, skip
            }
            'O' => {
                // Robot, see if we can push
                match find_space(
                    &grid,
                    x,
                    y,
                    x as isize - robot.0 as isize,
                    y as isize - robot.1 as isize,
                ) {
                    Some((row, col)) => {
                        grid[col][row] = 'O';
                        grid[y][x] = '.';
                        robot = (x, y);
                    }
                    None => {
                        // No room to push, skip
                    }
                }
            }
            '[' | ']' => {
                let mut boxes = HashSet::new();
                let xx = if grid[y][x] == '[' { x + 1 } else { x - 1 };
                if can_move(grid, x, y, c, &mut boxes) && can_move(grid, xx, y, c, &mut boxes) {
                    match c {
                        '^' => {
                            for b in boxes.iter().sorted_by_key(|b| b.1) {
                                grid[b.1 - 1][b.0] = grid[b.1][b.0];
                                grid[b.1][b.0] = '.';
                            }
                        }
                        '>' => {
                            for b in boxes.iter().sorted_by_key(|b| -(b.0 as isize)) {
                                assert_eq!(grid[b.1][b.0 + 1], '.');
                                grid[b.1][b.0 + 1] = grid[b.1][b.0];
                                grid[b.1][b.0] = '.';
                            }
                        }
                        'v' => {
                            for b in boxes.iter().sorted_by_key(|b| -(b.1 as isize)) {
                                assert_eq!(grid[b.1 + 1][b.0], '.');
                                grid[b.1 + 1][b.0] = grid[b.1][b.0];
                                grid[b.1][b.0] = '.';
                            }
                        }
                        '<' => {
                            for b in boxes.iter().sorted_by_key(|b| b.0) {
                                assert_eq!(grid[b.1][b.0 - 1], '.');
                                grid[b.1][b.0 - 1] = grid[b.1][b.0];
                                grid[b.1][b.0] = '.';
                            }
                        }
                        e => panic!("Unexpected tile '{e}' at ({x}, {y})"),
                    }
                    robot = (x, y);
                }
            }
            e => {
                panic!("Unknown tile: {e}")
            }
        }
    }

    let mut gps = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' || grid[y][x] == '[' {
                gps += x + 100 * y;
            }
        }
    }
    gps
}

fn find_space(
    grid: &Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    dx: isize,
    dy: isize,
) -> Option<(usize, usize)> {
    loop {
        match grid[y][x] {
            '.' => {
                return Some((x, y));
            }
            '#' => {
                return None;
            }
            'O' => {
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
            }
            e => {
                panic!("Could not parse grid item '{e}' at ({}, {})", x, y)
            }
        }
    }
}

fn can_move(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir: char,
    mut boxes: &mut HashSet<(usize, usize)>,
) -> bool {
    if boxes.contains(&(x, y)) {
        // This box has been checked already
        return true;
    }
    match grid[y][x] {
        '.' => true,
        '#' => false,
        '[' => {
            boxes.insert((x, y));
            if !can_move(grid, x + 1, y, dir, boxes) {
                return false;
            }
            match dir {
                '^' => can_move(grid, x, y - 1, dir, &mut boxes),
                '>' => can_move(grid, x + 1, y, dir, &mut boxes),
                'v' => can_move(grid, x, y + 1, dir, &mut boxes),
                '<' => can_move(grid, x - 1, y, dir, &mut boxes),
                e => panic!("Unexpected tile '{e}' at ({x}, {y})"),
            }
        }
        ']' => {
            boxes.insert((x, y));
            if !can_move(grid, x - 1, y, dir, boxes) {
                return false;
            }
            match dir {
                '^' => can_move(grid, x, y - 1, dir, &mut boxes),
                '>' => can_move(grid, x + 1, y, dir, &mut boxes),
                'v' => can_move(grid, x, y + 1, dir, &mut boxes),
                '<' => can_move(grid, x - 1, y, dir, &mut boxes),
                e => panic!("Unexpected tile '{e}' at ({x}, {y})"),
            }
        }
        e => panic!("Unexpected tile '{e}' at ({x}, {y})"),
    }
}

#[allow(dead_code)]
fn show_grid(grid: &Vec<Vec<char>>, robot: (usize, usize)) {
    for y in 0..grid.len() {
        let mut line = String::new();
        grid[y].iter().enumerate().for_each(|(x, c)| {
            if (x, y) == robot {
                line.push('@')
            } else {
                line.push(*c)
            }
        });
        println!("{line}");
    }
}

#[allow(dead_code)]
fn test_grid(grid: &Vec<Vec<char>>, robot: (usize, usize)) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if (x, y) == robot && grid[y][x] != '.' {
                return false;
            }
            match grid[y][x] {
                '.' | '#' | 'O' => {}
                '[' => {
                    if grid[y][x + 1] != ']' {
                        return false;
                    }
                }
                ']' => {
                    if grid[y][x - 1] != '[' {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
    }
    return true;
}

#[allow(dead_code)]
const SMALL_TEST: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
#[allow(dead_code)]
const LARGE_TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
#[allow(dead_code)]
const SMALL_TEST2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

#[test]
fn test_part1_small() {
    let (grid_data, move_data) = SMALL_TEST.splitn(2, "\n\n").collect_tuple().unwrap();
    let mut grid: Vec<Vec<_>> = grid_data.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(get_gps(&mut grid, move_data), 2028)
}

#[test]
fn test_part1_large() {
    let (grid_data, move_data) = LARGE_TEST.splitn(2, "\n\n").collect_tuple().unwrap();
    let mut grid: Vec<Vec<_>> = grid_data.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(get_gps(&mut grid, move_data), 10092)
}

#[test]
fn test_part2_small() {
    assert_eq!(solve(SMALL_TEST.to_string(), false).1, "1751");
}

#[test]
fn test_part2_small2() {
    assert_eq!(solve(SMALL_TEST2.to_string(), false).1, "618");
}

#[test]
fn test_part2_large() {
    assert_eq!(solve(LARGE_TEST.to_string(), false).1, "9021");
}
