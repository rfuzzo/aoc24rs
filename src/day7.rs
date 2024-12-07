#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

pub fn execute(is_part_two: bool) -> usize {
    let input = include_str!("../input/day7.txt");

    // split
    let mut correct_lines = Vec::new();
    let mut total = 0;
    for (j, line) in input.lines().enumerate() {
        //println!("> line: {}", line);
        let first: usize = line.split(": ").next().unwrap().parse().unwrap();
        let numbers: Vec<usize> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let num_len = numbers.len() - 1;
        let num_perms = 2_u32.pow(num_len as u32) as usize;

        let mut results = Vec::new();
        (0..num_perms).for_each(|i| {
            // get binary representation of i
            let bin = format!("{i:0width$b}", width = num_len)
                .chars()
                .map(|c| {
                    if c == '0' {
                        Operator::Add
                    } else {
                        Operator::Multiply
                    }
                })
                .collect::<Vec<Operator>>();

            //println!("bin: {:?}", bin);

            let mut result = numbers[0];
            for (k, o) in bin.iter().enumerate() {
                match o {
                    Operator::Add => {
                        result += numbers[k + 1];
                    }
                    Operator::Multiply => {
                        result *= numbers[k + 1];
                    }
                }
            }

            //println!("\t{i}: {i:0width$b} result: {}", result, width = num_len);
            results.push(result);
        });

        if results.contains(&first) {
            correct_lines.push(j);
            //println!("Correct line: {}", j);
            total += first;
        }
    }

    //println!("Correct lines: {:?}", correct_lines);

    total
}
