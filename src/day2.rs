pub fn execute() {
    let input = include_str!("../input/day2.txt");

    let mut safe = 0;

    for (i, line) in input.lines().enumerate() {
        // split line by whitespace
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // debug stuff
        let mut reason = "".to_owned();

        let error_index = check_line(numbers.clone(), None, &mut reason);

        if let Some(ind) = error_index {
            println!("#{};{} ({}): {}", i, ind, line, reason);

            // for until numbers size
            let mut found = false;
            for j in 0..numbers.len() {
                if check_line(numbers.clone(), Some(j), &mut reason).is_none() {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("\tno safe line found");
            } else {
                safe += 1;
            }
        } else {
            safe += 1;
        }
    }

    println!("{}", safe);
}

fn check_line(
    mut numbers: Vec<i32>,
    ignore_index: Option<usize>,
    reason: &mut String,
) -> Option<usize> {
    if let Some(ignored) = ignore_index {
        println!("ignoring index {}", ignored);

        // remove from numbers
        numbers.remove(ignored);
    }

    // check if safe
    let mut last_number = numbers.first().unwrap();
    let mut increasing = None;

    for (j, n) in numbers.iter().enumerate() {
        // ignore first
        if j == 0 {
            continue;
        }

        // check difference to last number
        let diff = n - last_number;
        if diff == 0 {
            *reason = "zero difference".to_owned();
            return Some(j);
        }

        // check if always either increasing or decreasing
        if increasing.is_none() {
            increasing = Some(diff > 0);
        } else if increasing.unwrap() != (diff > 0) {
            *reason = "not always increasing or decreasing".to_owned();
            return Some(j);
        }

        // check if difference is at least one and at most three.
        if diff.abs() > 3 || diff.abs() < 1 {
            *reason = "difference not between 1 and 3".to_owned();
            return Some(j);
        }

        last_number = n;
    }

    None
}
