pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let targets: Vec<u64> = input
        .lines()
        .map(|l| l.split_once(":").unwrap().0.parse().unwrap())
        .collect();
    let parts: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (ans, elem) in targets.iter().zip(parts) {
        let mut partials = vec![elem[0]];
        let mut partials2 = vec![elem[0]];

        for e in elem.iter().skip(1) {
            partials = apply_ops(&partials, *e, *ans, false);
            partials2 = apply_ops(&partials2, *e, *ans, true);
        }
        if partials.iter().any(|n| n == ans) {
            part1 += ans;
        }
        if partials2.iter().any(|n| n == ans) {
            part2 += ans;
        }
    }

    (part1.to_string(), part2.to_string())
}

fn apply_ops(v: &Vec<u64>, b: u64, target: u64, concat: bool) -> Vec<u64> {
    let mut result = Vec::with_capacity(v.len() * 2);
    for a in v {
        let x = a + b;
        if x <= target {
            result.push(x);
        }
        let x = a * b;
        if x <= target {
            result.push(x);
        }
        if concat {
            let x = concatenate(*a, b);
            if x <= target {
                result.push(x);
            }
        }
    }
    return result;
}

fn concatenate(a: u64, b: u64) -> u64 {
    let mut c = b;
    let mut n = 0;
    while c > 0 {
        n += 1;
        c /= 10;
    }
    return a * 10u64.pow(n) + b;
}
