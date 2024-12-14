use std::collections::HashSet;

use regex::Regex;

const ROOM_WIDTH: isize = 101;
const ROOM_HEIGHT: isize = 103;

const MID_X: isize = 50;
const MID_Y: isize = 51;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let pattern = Regex::new(r"(?ms)p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Part 1
    let mut quadrants = [0, 0, 0, 0];
    pattern
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| {
            (
                (x.parse::<isize>().unwrap() + 100 * vx.parse::<isize>().unwrap())
                    .rem_euclid(ROOM_WIDTH),
                (y.parse::<isize>().unwrap() + 100 * vy.parse::<isize>().unwrap())
                    .rem_euclid(ROOM_HEIGHT),
            )
        })
        .filter_map(quadrant)
        .for_each(|q| quadrants[q] += 1);
    let part1: i64 = quadrants.iter().product();

    // Part 2
    let mut positions = HashSet::new();
    let mut bots: Vec<(isize, isize, isize, isize)> = pattern
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| {
            (
                x.parse().unwrap(),
                y.parse().unwrap(),
                vx.parse().unwrap(),
                vy.parse().unwrap(),
            )
        })
        .collect();
    // let mut max_connected = 0;
    let mut part2 = 0;
    for i in 0..10_000 {
        positions.clear();
        // Move bots
        for bot in bots.iter_mut() {
            *bot = (
                (bot.0 + bot.2).rem_euclid(ROOM_WIDTH),
                (bot.1 + bot.3).rem_euclid(ROOM_HEIGHT),
                bot.2,
                bot.3,
            );
            positions.insert((bot.0, bot.1));
        }
        // Naive check if they are forming a connected pattern
        let connected: usize = bots
            .iter()
            .map(|(x, y, _, _)| {
                DIRS.iter()
                    .any(|(dx, dy)| positions.contains(&(x + dx, y + dy))) as usize
            })
            .sum();
        // max_connected = max_connected.max(connected);
        if connected > 300 {
            // show(&positions);
            // println!("Found at {}", i + 1);
            part2 = i + 1; // Plus 1, because we move the bots before checking
            break;
        }
    }
    // println!("Max connected: {max_connected}");

    (part1.to_string(), part2.to_string())
}

#[allow(dead_code)]
fn show(positions: &HashSet<(isize, isize)>) {
    for y in 0..ROOM_HEIGHT {
        println!();
        for x in 0..ROOM_WIDTH {
            if positions.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
    }
    println!();
}

fn quadrant(p: (isize, isize)) -> Option<usize> {
    let (x, y) = p;
    if x < MID_X && y < MID_Y {
        Some(0)
    } else if x > MID_X && y < MID_Y {
        Some(1)
    } else if x < MID_X && y > MID_Y {
        Some(2)
    } else if x > MID_X && y > MID_Y {
        Some(3)
    } else {
        // Midline
        None
    }
}

#[test]
fn test_part1() {
    // NB: update the consts above for these tests to work!
    let test_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let pattern = Regex::new(r"(?ms)p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut quadrants = [0, 0, 0, 0];
    pattern
        .captures_iter(&test_input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| {
            (
                (x.parse::<isize>().unwrap() + 100 * vx.parse::<isize>().unwrap())
                    .rem_euclid(ROOM_WIDTH),
                (y.parse::<isize>().unwrap() + 100 * vy.parse::<isize>().unwrap())
                    .rem_euclid(ROOM_HEIGHT),
            )
        })
        .filter_map(quadrant)
        .for_each(|q| quadrants[q] += 1);

    assert_eq!(quadrants[0], 1, "top left");
    assert_eq!(quadrants[1], 3, "top right");
    assert_eq!(quadrants[2], 4, "bottom left");
    assert_eq!(quadrants[3], 1, "bottom right");

    let part1: i64 = quadrants.iter().product();
    assert_eq!(part1, 12);
}
