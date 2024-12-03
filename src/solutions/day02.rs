pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let part1 = reports.iter().filter(|report| test_seq(report)).count();

    let part2 = reports
        .iter()
        .filter(|report| test_seq(report) || (0..report.len()).any(|k| test_seq_skip(report, k)))
        .count();

    (part1.to_string(), part2.to_string())
}

fn test_seq(report: &Vec<u32>) -> bool {
    return report.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] < 4)
        || report.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] < 4);
}

fn test_seq_skip(report: &Vec<u32>, k: usize) -> bool {
    let mut inc = true;
    let mut dec = true;
    for i in 1..report.len() {
        if i == k + 1 {
            if i > 1 {
                inc &= report[i - 2] < report[i] && report[i] - report[i - 2] < 4;
                dec &= report[i] < report[i - 2] && report[i - 2] - report[i] < 4;
            }
        } else if i != k {
            inc &= report[i - 1] < report[i] && report[i] - report[i - 1] < 4;
            dec &= report[i] < report[i - 1] && report[i - 1] - report[i] < 4;
        }
    }
    return inc || dec;
}
