use std::collections::VecDeque;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut files: VecDeque<_> = input
        .trim_end()
        .char_indices()
        .step_by(2)
        .map(|(i, c)| (i / 2, c.to_digit(10).unwrap() as usize))
        .collect();
    let mut frees = input
        .trim_end()
        .char_indices()
        .skip(1)
        .step_by(2)
        .map(|(_, c)| (usize::max_value(), c.to_digit(10).unwrap() as usize));

    // let bla: String = files
    //     .iter()
    //     .map(|(_, c)| char::from_digit(*c as u32, 10).unwrap())
    //     .collect();
    // println!("Files: {bla}");

    // let bla: String = frees
    //     .by_ref()
    //     .map(|(_, c)| char::from_digit(c as u32, 10).unwrap())
    //     .collect();
    // println!("Frees: {bla}");

    // let mut dbg_layout = "".to_string();

    let mut part1 = 0;
    let mut pos = 0;
    while files.len() > 0 {
        // First add (the next) file data
        let (id, block_count) = files.pop_front().unwrap();
        // (pos..pos + block_count).for_each(|_| dbg_layout += &id.to_string());
        part1 += (pos..pos + block_count).sum::<usize>() * id;
        pos += block_count;

        if files.len() > 0 {
            // Fill the next empty space with file data
            let (_, mut free_count) = frees.next().unwrap();
            while free_count > 0 && files.len() > 0 {
                let (id, mut block_count) = files.pop_back().unwrap();

                if block_count > free_count {
                    // Don't overfill the free space
                    files.push_back((id, block_count - free_count));
                    block_count = free_count;
                }

                // (pos..pos + block_count).for_each(|_| dbg_layout += &id.to_string());
                part1 += (pos..pos + block_count).sum::<usize>() * id;
                pos += block_count;
                free_count -= block_count;
            }
        }
    }
    // println!("Final layout: {dbg_layout}");

    (part1.to_string(), String::from("<not yet implemented>"))
}
