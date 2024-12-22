use std::collections::{HashMap, HashSet};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let initial_numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut results = Vec::new();
    let mut total_bananas = HashMap::new();
    for num in initial_numbers {
        let mut n = num;
        let mut prices = Vec::new();
        prices.push(n % 10);
        for _ in 0..2000 {
            n = (n ^ (n << 6)) & 0x00FFFFFF;
            n = (n ^ (n >> 5)) & 0x00FFFFFF;
            n = (n ^ (n << 11)) & 0x00FFFFFF;
            prices.push(n % 10);
        }
        results.push(n);

        let diffs = Vec::from_iter((1..prices.len()).map(|i| prices[i] - prices[i - 1]));
        let mut seen = HashSet::new();
        for i in 3..diffs.len() {
            let seq = (diffs[i - 3], diffs[i - 2], diffs[i - 1], diffs[i]);
            if !seen.contains(&seq) {
                let current_price = prices[i + 1]; // diffs is offset by one
                seen.insert(seq);
                total_bananas
                    .entry(seq)
                    .and_modify(|bananas| *bananas += current_price)
                    .or_insert(current_price);
            }
        }
    }
    let part1: i64 = results.iter().sum();
    let part2 = total_bananas.values().max().unwrap();
    // 1995 is too low

    (part1.to_string(), part2.to_string())
}

#[test]
fn test_part1() {
    let input = "1\n10\n100\n2024";
    // let input = "123";
    assert_eq!(solve(input.to_string(), false).0, "37327623");
}

#[test]
fn test_part2() {
    let input = "1\n2\n3\n2024";
    let input2 = "1\n10\n100\n2024";
    assert_eq!(solve(input.to_string(), false).1, "23");
    assert_eq!(solve(input2.to_string(), false).1, "24");
}
