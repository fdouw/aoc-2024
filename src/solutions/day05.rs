use std::{cmp::Ordering, collections::HashSet, hash::RandomState};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let data: Vec<_> = input.split("\n\n").collect();

    // Read ruleset
    let ruleset: HashSet<(u8, u8), RandomState> = HashSet::from_iter(data[0].lines().map(|l| {
        l.split("|")
            .map(|s| s.parse::<u8>().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    let mut updates: Vec<Vec<u8>> = data[1]
        .lines()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut part1 = 0u32;
    let mut part2 = 0u32;
    for update in updates.iter_mut() {
        // This code assumes that the ruleset covers all pairs (a,b) where needed
        // Topaz is that nice :)
        if update
            .iter()
            .tuple_windows()
            .any(|(a, b)| ruleset.contains(&(*b, *a)))
        {
            update.sort_by(|a, b| {
                if ruleset.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            part2 += update[update.len() / 2] as u32;
        } else {
            part1 += update[update.len() / 2] as u32;
        }
    }

    (part1.to_string(), part2.to_string())
}
