pub fn execute(is_part_two: bool) -> usize {
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
