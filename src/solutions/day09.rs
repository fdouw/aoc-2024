use std::collections::VecDeque;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Part 1
    let mut files: VecDeque<_> = input
        .trim_end()
        .char_indices()
        .step_by(2)
        .map(|(i, c)| (i / 2, c.to_digit(10).unwrap() as usize))
        .collect();
    let mut frees = input
        .trim_end()
        .chars()
        .skip(1)
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap() as usize);

    let mut part1 = 0;
    let mut pos = 0;
    while files.len() > 0 {
        // First add (the next) file data
        let (id, block_count) = files.pop_front().unwrap();
        part1 += (pos..pos + block_count).sum::<usize>() * id;
        pos += block_count;

        if files.len() > 0 {
            // Fill the next empty space with file data
            let mut free_count = frees.next().unwrap();
            while free_count > 0 && files.len() > 0 {
                let (id, mut block_count) = files.pop_back().unwrap();

                if block_count > free_count {
                    // Don't overfill the free space
                    files.push_back((id, block_count - free_count));
                    block_count = free_count;
                }

                part1 += (pos..pos + block_count).sum::<usize>() * id;
                pos += block_count;
                free_count -= block_count;
            }
        }
    }

    // Part 2
    fn parse_id(i: usize) -> isize {
        if i % 2 == 1 {
            -1
        } else {
            (i / 2) as isize
        }
    }

    let mut layout: Vec<_> = input
        .trim_end()
        .char_indices()
        .map(|(i, c)| (parse_id(i), c.to_digit(10).unwrap() as usize))
        .collect();

    // Move files to the left
    let mut i = layout.len() - 1;
    loop {
        let (id, len) = layout[i];

        if id >= 0 {
            // Not free space

            for j in 0..i {
                let (left_id, left_len) = layout[j];

                if left_id == -1 && left_len >= len {
                    // Free space that's big enough
                    layout[j] = layout[i];
                    layout[i] = (-1, len);
                    let remaining = left_len - len;
                    if remaining > 0 {
                        // Only insert space if there is any left
                        layout.insert(j + 1, (-1, remaining));
                        i += 1;
                    }
                    // Stop here, because we've moved the file
                    break;
                }
            }
        }

        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }

    let mut part2 = 0;
    let mut pos = 0;
    for (id, len) in layout {
        if id == -1 {
            // Skip free space
            pos += len;
        } else {
            part2 += (pos..pos + len).sum::<usize>() * id as usize;
            pos += len;
        }
    }

    (part1.to_string(), part2.to_string())
}

#[allow(dead_code)]
fn dump_layout(layout: &Vec<(isize, usize)>) {
    for (id, len) in layout.iter() {
        if *id == -1 {
            (0..*len).for_each(|_| print!("."));
        } else {
            (0..*len).for_each(|_| print!("{id}"));
        }
    }
    println!();
}
