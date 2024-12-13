use regex::Regex;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let pattern =
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)";
    let re = Regex::new(&pattern).unwrap();
    let mut scores = 0;
    let mut scores2 = 0;
    for (_, [ax, ay, bx, by, x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
        let ax: f64 = ax.parse().unwrap();
        let ay: f64 = ay.parse().unwrap();
        let bx: f64 = bx.parse().unwrap();
        let by: f64 = by.parse().unwrap();
        let x: f64 = x.parse().unwrap();
        let y: f64 = y.parse().unwrap();

        // Part 1
        // Cramer's rule
        let a = (x * by - y * bx) / (ax * by - ay * bx);
        let b = (ax * y - x * ay) / (ax * by - ay * bx);

        if a.floor() == a && b.floor() == b {
            scores += 3 * a as u64 + b as u64;
        }

        // Part 2
        let x = 10000000000000. + x;
        let y = 10000000000000. + y;
        // Cramer's rule
        let a = (x * by - y * bx) / (ax * by - ay * bx);
        let b = (ax * y - x * ay) / (ax * by - ay * bx);

        if a.floor() == a && b.floor() == b {
            scores2 += 3 * a as u64 + b as u64;
        }
    }

    (scores.to_string(), scores2.to_string())
}
