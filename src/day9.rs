use std::collections::HashMap;

pub fn execute(is_part_two: bool) -> usize {
    if is_part_two {
        return part_two();
    }

    part_one()
}

fn part_two() -> usize {
    let input = include_str!("../input/day9.txt");

    let mut files_id: usize = 0;
    let mut files_count = HashMap::new();
    let mut files = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_control() {
            continue;
        }

        let b = c.to_digit(10).unwrap() as u8;

        // even
        if i % 2 == 0 {
            for _j in 0..b {
                files.push(Some(files_id));
            }
            files_count.insert(files_id, b);

            files_id += 1;
        } else {
            for _j in 0..b {
                files.push(None);
            }
        }
    }

    // println!("files_id: {:?}", files_id);
    // println!("files_count: {:?}", files_count);

    // for f in files.iter() {
    //     if let Some(d) = f {
    //         print!("{}", d);
    //     } else {
    //         print!(".");
    //     }
    // }
    // println!();

    let n = files.len();
    // println!("n: {}", n);
    for i in (1..files_id).rev() {
        // println!("i: {}", i);

        let mut idx_end = 0;
        for j in (0..n).rev() {
            if let Some(d) = files[j] {
                if d == i {
                    idx_end = j;
                    break;
                }
            }
        }
        let len = *files_count.get(&i).unwrap() as usize;
        let idx_start = idx_end - len + 1;
        // println!(" idx_start: {}, idx_end: {}", idx_start, idx_end);

        for (j, window) in files
            .iter()
            .take(idx_start)
            .collect::<Vec<_>>()
            .windows(len)
            .enumerate()
        {
            if window.iter().all(|x| x.is_none()) {
                // println!(" window: {}-{}", j, j + len);

                // insert
                for item in files.iter_mut().skip(j).take(len) {
                    *item = Some(i);
                }

                // remove
                for item in files.iter_mut().take(idx_end + 1).skip(idx_start) {
                    *item = None;
                }

                // for f in files.iter() {
                //     if let Some(d) = f {
                //         print!("{}", d);
                //     } else {
                //         print!(".");
                //     }
                // }
                // println!();

                break;
            }
        }
    }

    let mut total = 0;
    for (i, f) in files.iter().enumerate() {
        if let Some(d) = f {
            total += i * d;
        }
    }

    total
}

fn part_one() -> usize {
    let input = include_str!("../input/day9.txt");

    let mut files_id: usize = 0;
    let mut files = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_control() {
            continue;
        }

        let b = c.to_digit(10).unwrap() as u8;

        // even
        if i % 2 == 0 {
            for _j in 0..b {
                files.push(Some(files_id));
            }
            files_id += 1;
        } else {
            for _j in 0..b {
                files.push(None);
            }
        }
    }

    //println!("files_id: {:?}", files_id);
    //println!("files: {:?}", files);

    let mut output = Vec::new();
    let n = files.len();
    for i in 0..n {
        //println!("i: {}, len: {}", i, files.len());
        if files.len() <= i {
            break;
        }
        let c = &files[i];

        if let Some(d) = c {
            output.push(*d);
        } else {
            loop {
                if files.len() <= i {
                    break;
                }
                if let Some(last) = files.remove(files.len() - 1) {
                    output.push(last);
                    break;
                }
            }
        }
    }

    //println!("output: {:?}", output);

    let mut total = 0;
    for (i, c) in output.into_iter().enumerate() {
        total += i * c;
    }

    total
}
