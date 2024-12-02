pub fn day1() {
    let input = include_str!("../input/day1.txt");
    let mut total = 0;

    let mut left_v = Vec::new();
    let mut right_v = Vec::new();

    for line in input.lines() {
        // split line by whitespace
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // get absolute difference between max and min
        let left = numbers[0];
        let right = numbers[1];

        left_v.push(left);
        right_v.push(right);

        let diff = (left - right).abs();
        total += diff;
    }

    // sort the vectors
    left_v.sort();
    right_v.sort();

    let mut total2 = 0;

    // iterate over the vectors and get the difference
    for (i, l) in left_v.iter().enumerate() {
        let r = right_v[i];
        let d = (l - r).abs();

        total2 += d;
    }

    println!("{}", total);
    println!("correct: {}", total2);
}

pub fn day2() {
    use std::collections::VecDeque;

    let input = include_str!("../input/day2.txt");

    let mut safe = 0;

    for (i, line) in input.lines().enumerate() {
        // split line by whitespace
        let mut numbers: VecDeque<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // debug stuff
        let mut reason = "".to_owned();
        let mut sequence = 0;

        // check if safe
        let mut safe_line = true;
        let mut last_number = numbers.pop_front().unwrap();
        let mut increasing = None;

        for (j, n) in numbers.into_iter().enumerate() {
            sequence = j + 2;

            // check difference to last number
            let diff = n - last_number;
            if diff == 0 {
                safe_line = false;
                reason = "zero difference".to_owned();
                break;
            }

            // check if always either increasing or decreasing
            if increasing.is_none() {
                increasing = Some(diff > 0);
            } else if increasing.unwrap() != (diff > 0) {
                safe_line = false;
                reason = "not always increasing or decreasing".to_owned();
                break;
            }

            // check if difference is at least one and at most three.
            if diff.abs() > 3 || diff.abs() < 1 {
                safe_line = false;
                reason = "difference not between 1 and 3".to_owned();
                break;
            }

            last_number = n;
        }

        if safe_line {
            safe += 1;
        } else {
            println!("#{};{} ({}) unsafe: {}", i, sequence, line, reason);
        }
    }

    println!("{}", safe);
}
