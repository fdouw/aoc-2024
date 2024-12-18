use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const SIZE: u32 = 71; // 7
const BYTES_FALLING: usize = 1024; // 12

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let bytes_all: Vec<_> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let bytes = bytes_all.iter().take(BYTES_FALLING).collect();
    let part1 = find_path(&bytes).unwrap();

    let mut lo = BYTES_FALLING + 1;
    let mut hi = bytes_all.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let bytes = bytes_all.iter().take(mid).collect();
        if has_path(&bytes) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let part2 = format!("{},{}", bytes_all[lo - 1].0, bytes_all[lo - 1].1);

    (part1.to_string(), part2)
}

fn find_path(bytes: &HashSet<&(u32, u32)>) -> Option<u32> {
    let mut history: HashSet<(u32, u32)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while !queue.is_empty() {
        let (x, y, dist) = queue.pop_front().unwrap();
        if x == SIZE - 1 && y == SIZE - 1 {
            // Finish
            // println!("Found the finish at ({x}, {y}) after distance {dist}");
            return Some(dist);
        }
        if !bytes.contains(&(x, y)) {
            if 0 < x && !history.contains(&(x - 1, y)) {
                history.insert((x - 1, y));
                queue.push_back((x - 1, y, dist + 1));
            }
            if 0 < y && !history.contains(&(x, y - 1)) {
                history.insert((x, y - 1));
                queue.push_back((x, y - 1, dist + 1));
            }
            if x + 1 < SIZE && !history.contains(&(x + 1, y)) {
                history.insert((x + 1, y));
                queue.push_back((x + 1, y, dist + 1));
            }
            if y + 1 < SIZE && !history.contains(&(x, y + 1)) {
                history.insert((x, y + 1));
                queue.push_back((x, y + 1, dist + 1));
            }
        }
    }
    return None;
}

fn has_path(bytes: &HashSet<&(u32, u32)>) -> bool {
    let mut history: HashSet<(u32, u32)> = HashSet::new();
    let mut queue = Vec::new();
    queue.push((0, 0));
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if x == SIZE - 1 && y == SIZE - 1 {
            // Finish
            return true;
        }
        if !bytes.contains(&(x, y)) {
            if 0 < x && !history.contains(&(x - 1, y)) {
                history.insert((x - 1, y));
                queue.push((x - 1, y));
            }
            if 0 < y && !history.contains(&(x, y - 1)) {
                history.insert((x, y - 1));
                queue.push((x, y - 1));
            }
            if x + 1 < SIZE && !history.contains(&(x + 1, y)) {
                history.insert((x + 1, y));
                queue.push((x + 1, y));
            }
            if y + 1 < SIZE && !history.contains(&(x, y + 1)) {
                history.insert((x, y + 1));
                queue.push((x, y + 1));
            }
        }
    }
    return false;
}

#[allow(dead_code)]
fn show_bytes(bytes: &HashSet<(u32, u32)>) {
    // Doesn't actually print the path, but the visited tiles instead!
    for y in 0..SIZE {
        println!();
        for x in 0..SIZE {
            if bytes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    println!("\n");
}

#[test]
fn test_part1() {
    // NB: need to change SIZE and BYTES_FALLING for the test to work
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(
        solve(input.to_string(), false),
        ("22".to_string(), "6,1".to_string())
    )
}
