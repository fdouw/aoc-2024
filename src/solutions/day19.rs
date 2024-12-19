use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let (pattern_data, design_data) = input.split_once("\n\n").unwrap();
    let pattern = Regex::new(format!("^({})+$", pattern_data.replace(", ", "|")).as_str()).unwrap();

    let possible: Vec<_> = design_data
        .lines()
        .filter(|l| pattern.is_match(l))
        .collect();
    let part1 = possible.len();

    let possible_all = possible.join(",");
    let pattern_data = pattern_data
        .split(", ")
        .filter(|p| possible_all.contains(p))
        .collect();
    let mut counter = Counter::new(pattern_data);
    let part2: u64 = possible.iter().map(|design| counter.count(design)).sum();

    (part1.to_string(), part2.to_string())
}

struct Counter<'a> {
    cache: HashMap<&'a str, u64>,
    patterns: Vec<&'a str>,
}

impl<'a> Counter<'a> {
    fn new(patterns: Vec<&'a str>) -> Self {
        Self {
            cache: HashMap::new(),
            patterns,
        }
    }
    fn count(&mut self, design: &'a str) -> u64 {
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(design);
        while !stack.is_empty() {
            let d = stack.pop().unwrap();
            if d.len() == 0 {
                count += 1;
            } else if self.cache.contains_key(d) {
                count += self.cache[d];
            } else {
                let stripped: Vec<_> = self
                    .patterns
                    .iter()
                    .filter_map(|p| d.strip_prefix(p))
                    .collect();
                let partial_count = stripped.iter().map(|s| self.count(s)).sum::<u64>();
                self.cache.insert(d, partial_count);
                count += partial_count;
            }
        }
        return count;
    }
}

#[test]
fn test_day19() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    assert_eq!(solve(input.to_string(), false), ("6".into(), "16".into()))
}
