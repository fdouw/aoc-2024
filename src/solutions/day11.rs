use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let init: Vec<_> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut stones = CachedStones::new();
    let part1 = init.iter().map(|s| stones.count(*s, 25)).sum::<usize>();
    let part2 = init.iter().map(|s| stones.count(*s, 75)).sum::<usize>();

    (part1.to_string(), part2.to_string())
}

struct CachedStones {
    cache: HashMap<(usize, usize), usize>,
}

impl CachedStones {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn cache(&mut self, key: (usize, usize), val: usize) -> usize {
        self.cache.insert(key, val);
        return val;
    }
    fn count(&mut self, number: usize, depth: usize) -> usize {
        if self.cache.contains_key(&(number, depth)) {
            return self.cache[&(number, depth)];
        } else {
            if depth == 0 {
                return 1;
            } else if number == 0 {
                let result = self.count(1, depth - 1);
                return self.cache((number, depth), result);
            } else {
                let txt = number.to_string();
                let (div, rem) = (txt.len() / 2, txt.len() % 2);
                if rem == 0 {
                    let (a, b) = txt.split_at(div);
                    let result = self.count(a.parse().unwrap(), depth - 1)
                        + self.count(b.parse().unwrap(), depth - 1);
                    return self.cache((number, depth), result);
                } else {
                    let result = self.count(number * 2024, depth - 1);
                    return self.cache((number, depth), result);
                }
            }
        }
    }
}
