use regex::Regex;

const ROOM_WIDTH: i64 = 101;
const ROOM_HEIGHT: i64 = 103;

const MID_X: i64 = 50;
const MID_Y: i64 = 51;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let pattern = Regex::new(r"(?ms)p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut quadrants = [0, 0, 0, 0];
    pattern
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| {
            (
                (x.parse::<i64>().unwrap() + 100 * vx.parse::<i64>().unwrap())
                    .rem_euclid(ROOM_WIDTH),
                (y.parse::<i64>().unwrap() + 100 * vy.parse::<i64>().unwrap())
                    .rem_euclid(ROOM_HEIGHT),
            )
        })
        .filter_map(quadrant)
        .for_each(|q| quadrants[q] += 1);
    let part1: i64 = quadrants.iter().product();

    (part1.to_string(), String::from("<not yet implemented>"))
}

fn quadrant(p: (i64, i64)) -> Option<usize> {
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
                (x.parse::<i64>().unwrap() + 100 * vx.parse::<i64>().unwrap())
                    .rem_euclid(ROOM_WIDTH),
                (y.parse::<i64>().unwrap() + 100 * vy.parse::<i64>().unwrap())
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
