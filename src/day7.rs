#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

pub fn execute(is_part_two: bool) -> usize {
    if is_part_two {
        part_two()
    } else {
        part_one()
    }
}

pub fn part_one() -> usize {
    let input = include_str!("../input/day7.txt");

    // split
    let mut total = 0;
    for line in input.lines() {
        // println!("> line: {}", line);
        let splits = line.split(": ").collect::<Vec<&str>>();
        let first: usize = splits[0].parse().unwrap();
        let numbers: Vec<usize> = splits[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let num_len = numbers.len() - 1;
        let num_perms = 2_u32.pow(num_len as u32) as usize;

        let mut tries = Vec::new();
        (0..num_perms).for_each(|i| {
            // get binary representation of i
            let binary = format!("{i:0width$b}", width = num_len);

            let bin = binary
                .chars()
                .map(|c| {
                    if c == '0' {
                        Operator::Add
                    } else {
                        Operator::Multiply
                    }
                })
                .collect::<Vec<Operator>>();

            tries.push(bin.clone());
        });

        let mut found = false;
        for seq in tries {
            let mut result = numbers[0];
            for (k, o) in seq.iter().enumerate() {
                match o {
                    Operator::Add => {
                        result += numbers[k + 1];
                    }
                    Operator::Multiply => {
                        result *= numbers[k + 1];
                    }
                    Operator::Concatenate => todo!(),
                }
            }

            //println!("\t{i}: {i:0width$b} result: {}", result, width = num_len);
            if result > first {
                continue;
            }

            if result == first {
                found = true;
                break;
            }
        }

        if found {
            total += first;
        }
    }

    //println!("Correct lines: {:?}", correct_lines);

    total
}

fn to_base3(v: usize, len: usize) -> String {
    let mut b = String::new();
    let mut r = v;
    for i in (0..len).rev() {
        let base = 3_usize.pow(i as u32);
        if r >= 2 * base {
            r -= 2 * base;
            b.push('2');
        } else if r >= base {
            r -= base;
            b.push('1');
        } else {
            b.push('0');
        }
    }
    b
}

pub fn part_two() -> usize {
    let input = include_str!("../input/day7.txt");

    // split
    let mut total = 0;
    for line in input.lines() {
        //println!("> line: {}", line);
        let splits = line.split(": ").collect::<Vec<&str>>();
        let first: usize = splits[0].parse().unwrap();
        let numbers: Vec<usize> = splits[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let num_len = numbers.len() - 1;
        let num_perms = 3_usize.pow(num_len as u32);

        let mut tries = Vec::new();
        (0..num_perms).for_each(|i| {
            // get binary representation of i
            let t = to_base3(i, num_len);
            //println!("t: {}", t);

            let bin = t
                .chars()
                .map(|c| {
                    if c == '2' {
                        Operator::Add
                    } else if c == '1' {
                        Operator::Multiply
                    } else {
                        Operator::Concatenate
                    }
                })
                .collect::<Vec<Operator>>();

            //println!("bin: {:?}", bin);
            tries.push(bin.clone());
        });

        let mut found = false;
        for bin in tries {
            let mut result = numbers[0];
            for (k, o) in bin.iter().enumerate() {
                match o {
                    Operator::Add => {
                        result += numbers[k + 1];
                    }
                    Operator::Multiply => {
                        result *= numbers[k + 1];
                    }
                    Operator::Concatenate => {
                        result = format!("{}{}", result, numbers[k + 1]).parse().unwrap()
                    }
                }
            }

            //println!("\t{i}: {i:0width$b} result: {}", result, width = num_len);
            if result > first {
                continue;
            }

            if result == first {
                found = true;
                break;
            }
        }

        if found {
            total += first;
        }
    }

    //println!("Correct lines: {:?}", correct_lines);

    total
}
