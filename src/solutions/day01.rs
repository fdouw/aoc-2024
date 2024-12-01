use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    // Part 1
    a.sort_unstable();
    b.sort_unstable();

    let part1: u32 = a
        .iter()
        .zip(b.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    // Part 2
    let mut part2 = 0;
    let mut i = 0;
    let mut j = 0;
    'outer: while i < a.len() && j < b.len() {
        while a[i] > b[j] {
            j += 1;
            if j >= b.len() {
                break 'outer;
            }
        }
        while a[i] < b[j] {
            i += 1;
            if i >= a.len() {
                break 'outer;
            }
        }
        if a[i] == b[j] {
            let mut l = 1;
            let mut r = 1;
            while i + 1 < a.len() && a[i + 1] == a[i] {
                l += 1;
                i += 1;
            }
            while j + 1 < b.len() && b[j + 1] == b[j] {
                r += 1;
                j += 1;
            }
            part2 += a[i] * l * r;
            // Move pointers past this stretch
            i += 1;
            j += 1;
        }
    }
    (part1.to_string(), part2.to_string())
}
