use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut network = HashMap::new();
    // Read network connectome
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .for_each(|(a, b)| {
            network.entry(a).or_insert(HashSet::new()).insert(b);
            network.entry(b).or_insert(HashSet::new()).insert(a);
        });
    // Find starting with t
    let mut triplets = HashSet::new();
    for (computer, neighbours) in network.iter().filter(|(k, _)| k.starts_with('t')) {
        for nb in neighbours {
            for nbnb in network.get(nb).unwrap() {
                if neighbours.contains(nbnb) {
                    let mut v = [computer, nb, nbnb];
                    v.sort();
                    triplets.insert(v);
                }
            }
        }
    }
    let part1 = triplets.len();

    let mut best = Vec::new();
    for (computer, neighbours) in network.iter() {
        let connected = search(&network, computer, neighbours);
        if connected.len() > best.len() {
            best = connected;
            println!("partial result: {:?}", best.iter().sorted().join(","));
        }
    }
    let part2 = best.iter().sorted().join(",");

    (part1.to_string(), part2.to_string())
}

fn search<'a>(
    network: &HashMap<&str, HashSet<&'a str>>,
    computer: &'a str,
    adjacent: &HashSet<&'a str>,
) -> Vec<&'a str> {
    if adjacent.len() == 0 {
        // nothing left to search
        return vec![computer];
    }
    let mut best = Vec::new();
    for &nb in adjacent {
        let set = network.get(nb).unwrap() & adjacent;
        let part_results = search(network, &nb, &set);
        if part_results.len() > best.len() {
            best = part_results;
        }
    }
    best.push(computer);
    return best;
}

#[test]
fn test_part1() {
    let input = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn\n";
    assert_eq!(solve(input.to_string(), false).0, "7")
}

#[test]
fn test_part2() {
    let input = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn\n";
    assert_eq!(solve(input.to_string(), false).1, "co,de,ka,ta")
}
