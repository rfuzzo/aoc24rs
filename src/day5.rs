use std::collections::HashMap;

pub fn execute(is_part_two: bool) -> i32 {
    //     if is_part_two {
    //         part_two()
    //     } else {
    part_one()
    // }
}

fn part_one() -> i32 {
    // parse input
    let input = include_str!("../input/day5.txt");
    let mut p2 = false;
    let mut order_after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut order_before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            p2 = true;
            continue;
        }

        if !p2 {
            // ordering rules
            let n1 = line[0..2].parse::<i32>().unwrap();
            let n2 = line[3..5].parse::<i32>().unwrap();
            // insert or update
            if let Some(v) = order_after.get_mut(&n1) {
                v.push(n2);
            } else {
                order_after.insert(n1, vec![n2]);
            }

            if let Some(v) = order_before.get_mut(&n2) {
                v.push(n1);
            } else {
                order_before.insert(n2, vec![n1]);
            }
        } else {
            let numbers: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(numbers);
        }
    }

    // order
    // for values in order_after.values_mut() {
    //     values.sort();
    // }
    // for values in order_before.values_mut() {
    //     values.sort();
    // }

    // task
    let mut correct_updates = Vec::new();
    for (k, v) in updates.iter().enumerate() {
        println!("{}#{:?}", k, v);
        let mut correct = true;

        for (i, n) in v.iter().enumerate() {
            // check numbers before: fails if any number is before
            if let Some(nums) = order_after.get(n) {
                // checks
                //println!("\tAfter {}: {:?}", n, nums);

                for j in v.iter().take(i) {
                    if nums.contains(j) {
                        correct = false;
                        //println!("\tError: {} before {}", j, n);
                        break;
                    } else {
                        correct = true;
                    }
                }
            }

            // check numbers after: fails if
            if let Some(nums) = order_before.get(n) {
                // checks
                //println!("\tBefore {}: {:?}", n, nums);

                for j in v.iter().rev().take(v.len() - i) {
                    if nums.contains(j) {
                        correct = false;
                        //println!("\tError: {} after {}", j, n);
                        break;
                    } else {
                        correct = true;
                    }
                }
            }

            if !correct {
                break;
            }
        }

        if correct {
            let middle_number = v[(v.len() - 1) / 2];
            correct_updates.push(middle_number);
        }
    }

    // debug stuff
    // println!("{:?}", order);
    // println!("{:?}", updates);

    correct_updates.iter().sum()
}
