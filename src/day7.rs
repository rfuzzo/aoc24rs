#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct SInput {
    first: usize,
    numbers: Vec<usize>,
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
    let mut inputs = Vec::new();
    for line in input.lines() {
        //println!("> line: {}", line);
        let splits = line.split(": ").collect::<Vec<&str>>();
        let first: usize = splits[0].parse().unwrap();
        let numbers: Vec<usize> = splits[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        inputs.push(SInput { first, numbers });
    }

    let mut total = 0;

    for input in inputs {
        let first = input.first;
        let numbers = input.numbers;

        let num_len = numbers.len() - 1;
        let num_perms = 2_u32.pow(num_len as u32) as usize;

        let mut found = false;
        for i in 0..num_perms {
            let mut result = numbers[0];
            for (k, o) in to_base2_op(i, num_len).iter().enumerate() {
                match o {
                    Operator::Add => {
                        result += numbers[k + 1];
                    }
                    Operator::Multiply => {
                        result *= numbers[k + 1];
                    }
                    Operator::Concatenate => todo!(),
                }

                if result >= first {
                    break;
                }
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

// fn to_base3(v: usize, len: usize) -> String {
//     let mut b = String::new();
//     let mut r = v;
//     for i in (0..len).rev() {
//         let base = 3_usize.pow(i as u32);
//         if r >= 2 * base {
//             r -= 2 * base;
//             b.push('2');
//         } else if r >= base {
//             r -= base;
//             b.push('1');
//         } else {
//             b.push('0');
//         }
//     }
//     b
// }

fn to_base2_op(v: usize, len: usize) -> Vec<Operator> {
    let mut b = vec![];
    let mut r = v;
    for i in (0..len).rev() {
        let base = 2_usize.pow(i as u32);
        if r >= base {
            r -= base;
            b.push(Operator::Multiply);
        } else {
            b.push(Operator::Add);
        }
    }
    b
}

fn to_base3_op(v: usize, len: usize) -> Vec<Operator> {
    let mut b = vec![];
    let mut r = v;
    for i in (0..len).rev() {
        let base = 3_usize.pow(i as u32);
        if r >= 2 * base {
            r -= 2 * base;
            b.push(Operator::Concatenate);
        } else if r >= base {
            r -= base;
            b.push(Operator::Multiply);
        } else {
            b.push(Operator::Add);
        }
    }
    b
}

pub fn part_two() -> usize {
    let input = include_str!("../input/day7.txt");

    // split
    let mut inputs = Vec::new();
    for line in input.lines() {
        //println!("> line: {}", line);
        let splits = line.split(": ").collect::<Vec<&str>>();
        let first: usize = splits[0].parse().unwrap();
        let numbers: Vec<usize> = splits[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        inputs.push(SInput { first, numbers });
    }

    let mut total = 0;

    for input in inputs {
        let first = input.first;
        let numbers = input.numbers;

        let num_len = numbers.len() - 1;
        let num_perms = 3_usize.pow(num_len as u32);

        let mut found = false;
        for i in 0..num_perms {
            let mut result = numbers[0];
            for (k, o) in to_base3_op(i, num_len).iter().enumerate() {
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

                if result >= first {
                    break;
                }
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

    total
}
