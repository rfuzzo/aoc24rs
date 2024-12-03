pub fn execute() -> i32 {
    let input = include_str!("../input/day2.txt");

    let mut safe = 0;

    for line in input.lines() {
        // split line by whitespace
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // debug stuff
        //let mut reason = "".to_owned();

        // first check
        if let Some(ind) = check_line(numbers.clone(), None) {
            //println!("#{};{} ({}): {}", i, ind, line, reason);

            // try -2,-1 and the faulty index and check again
            let mut found = false;
            for j in 0..=2.min(ind) {
                let idx = (ind - j).clamp(0, numbers.len() - 1);
                if check_line(numbers.clone(), Some(idx)).is_none() {
                    found = true;
                    break;
                }
            }

            if !found {
                //println!("\tno safe line found");
            } else {
                safe += 1;
            }
        } else {
            safe += 1;
        }
    }

    //println!("{}", safe);
    safe
}

fn check_line(
    mut numbers: Vec<i32>,
    ignore_index: Option<usize>,
    //reason: &mut String,
) -> Option<usize> {
    if let Some(ignored) = ignore_index {
        //println!("ignoring index {}", ignored);

        // remove from numbers
        numbers.remove(ignored);
    }

    // check if safe
    let mut last_number = numbers[0];
    let increasing = numbers[1] > last_number;

    for (j, n) in numbers.into_iter().enumerate() {
        // ignore first
        if j == 0 {
            continue;
        }

        // check difference to last number
        let diff = n - last_number;
        // if diff == 0 {
        //     //*reason = "zero difference".to_owned();
        //     return Some(j);
        // }

        // check if always either increasing or decreasing
        if increasing != (diff > 0) {
            //*reason = "not always increasing or decreasing".to_owned();
            return Some(j);
        }

        // check if difference is at least one and at most three.
        let abs = diff.abs();
        if !(1..=3).contains(&abs) {
            //*reason = "difference not between 1 and 3".to_owned();
            return Some(j);
        }

        last_number = n;
    }

    None
}
