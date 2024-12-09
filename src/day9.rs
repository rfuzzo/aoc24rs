use std::collections::HashMap;

pub fn execute(is_part_two: bool) -> usize {
    if is_part_two {
        return part_two();
    }

    part_one()
}

fn part_two() -> usize {
    let input = include_str!("../input/day9.txt");

    //let mut files_id: usize = 0;
    let mut spaces = Vec::new();
    let mut files = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_control() {
            continue;
        }

        let b = c.to_digit(10).unwrap() as u8;

        // even
        if i % 2 == 0 {
            // for _j in 0..b {
            //     files.push(Some(files_id));
            // }
            files.push(b);
            //files_id += 1;
        } else {
            // for _j in 0..b {
            //     files.push(None);
            // }
            spaces.push(b);
        }
    }

    // println!("files: {:?}", files);
    // println!("space: {:?}", spaces);

    let mut new_idices: HashMap<usize, Vec<usize>> = HashMap::new();
    let files_len = files.len();
    for (i, file) in files.iter().rev().enumerate() {
        //println!("file: {}", file);
        let idx = files_len - 1 - i;
        if idx == 0 {
            break;
        }
        //println!("idx: {}", idx);

        for j in 0..idx {
            let space = &mut spaces[j];

            if space.to_owned() >= *file {
                // we can move it
                *space -= file;

                // update the new indices
                //println!("idx: {}, j: {}", idx, j);
                assert!(idx > j);
                if let Some(x) = new_idices.get_mut(&j) {
                    x.push(idx);
                } else {
                    new_idices.insert(j, vec![idx]);
                }

                break;
            }
        }
    }

    println!("new_idices: {:?}", new_idices);
    //println!("space: {:?}", spaces);

    // collect
    let mut was_moved = Vec::new();
    let mut output: String = String::new();

    for i in 0..files.len() {
        if !was_moved.contains(&i) {
            let file: u8 = files[i];
            for _j in 0..file {
                output += &i.to_string();
            }
        } else {
            let file: u8 = files[i];
            for _j in 0..file {
                output += ".";
            }
        }

        if let Some(to_move) = new_idices.get(&i) {
            for j in to_move {
                let file: u8 = files[*j];
                for _k in 0..file {
                    output += &j.to_string();
                }
                was_moved.push(*j);
            }
        }

        if i < spaces.len() {
            let space = spaces[i];
            for _i in 0..space {
                output += ".";
            }
        }
    }

    println!("output: {:?}", output);

    let mut total = 0;
    for (i, c) in output.chars().enumerate() {
        if c == '.' {
            continue;
        } else {
            let c = c.to_digit(10).unwrap() as usize;
            total += i * c;
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
