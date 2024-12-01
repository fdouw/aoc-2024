pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let size = input.lines().count();

    // Split the input in 2 lists
    let mut a: Vec<u32> = Vec::with_capacity(size);
    let mut b: Vec<u32> = Vec::with_capacity(size);
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        a.push(parts.next().unwrap().parse::<u32>().unwrap());
        b.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

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
