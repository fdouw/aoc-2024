use regex::{Captures, Regex};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let re_mut = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_skip = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)").unwrap();

    let part1: u32 = re_mut
        .captures_iter(input.as_str())
        .map(|c| c.get_num(1) * c.get_num(2))
        .sum();

    let cleared_input = re_skip.replace_all(&input, ":");

    let part2: u32 = re_mut
        .captures_iter(&cleared_input)
        .map(|c| c.get_num(1) * c.get_num(2))
        .sum();

    (part1.to_string(), part2.to_string())
}

trait ExtractNum {
    fn get_num(&self, index: usize) -> u32;
}
impl ExtractNum for Captures<'_> {
    fn get_num(&self, index: usize) -> u32 {
        self.get(index).map_or(0, |m| m.as_str().parse().unwrap())
    }
}
