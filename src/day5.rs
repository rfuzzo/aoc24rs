use std::collections::HashMap;

pub fn execute(is_part_two: bool) -> usize {
    // parse input
    let input = include_str!("../input/day5.txt");
    let mut p2 = false;
    let mut order_after: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut order_before: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut edges = Vec::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            p2 = true;
            continue;
        }

        if !p2 {
            // ordering rules
            let n1 = line[0..2].parse::<usize>().unwrap();
            let n2 = line[3..5].parse::<usize>().unwrap();

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
            let numbers: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
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

    for (k, v) in order_after.iter() {
        // add edges
        for n in v {
            edges.push((*k, *n));
        }
    }
    for (k, v) in order_before.iter() {
        // add edges
        for n in v {
            edges.push((*n, *k));
        }
    }

    //println!("{:?}", edges);

    // task
    let mut middle_numbers = 0;
    //let mut correct_updates_idx = Vec::new();
    let mut incorrect_updates_idx = Vec::new();
    for (k, v) in updates.iter().enumerate() {
        //println!("{}#{:?}", k, v);

        if check_range(v, &order_after, &order_before) {
            middle_numbers += v[(v.len() - 1) / 2];
            //correct_updates_idx.push(k);
        } else {
            incorrect_updates_idx.push(k);
        }
    }

    if !is_part_two {
        return middle_numbers;

        // let sum = correct_updates_idx.iter().fold(0, |acc, idx| {
        //     let x = &updates[*idx];
        //     let middle_number = x[(x.len() - 1) / 2];
        //     acc + middle_number
        // });
        // return sum;
    }

    // part 2
    //let mut max_iterations = 0;
    let mut sum = 0;
    for idx in incorrect_updates_idx.iter() {
        let v = &updates[*idx];
        //println!("Incorrect Update {}: {:?}", i, v);

        // stable sort according to rules
        // we simply assume that no cycle is present and that the stable sort is possible
        let mut sorted = v.clone();
        let mut index = 0;
        //let mut fixed = false;
        for _j in 1..3 {
            //max_iterations = max_iterations.max(_j);

            //let any_change = stable_topo_sort_inner(&edges, &mut sorted, &mut index);
            let any_change = stable_topo_sort_opt2(&edges, &mut sorted, &mut index);

            // sort again
            if !any_change {
                //println!("\tFixed Update after {} iterations: {:?}", j, &sorted);
                //fixed = true;
                sum += sorted[(sorted.len() - 1) / 2];
                break;
            }
        }

        // //checks
        // if !fixed {
        //     println!("Error: Could not fix the incorrect update");
        // } else {
        //     // check if the fixed update is correct
        //     if !check_range(&sorted, &order_after, &order_before) {
        //         println!("Error: Fixed Update is incorrect");
        //     }
        // }
    }

    //println!("Max iterations: {}", max_iterations);

    //assert!(fixed_updates.len() == incorrect_updates_idx.len());

    // calculate the middle numbers
    // let sum = fixed_updates.iter().fold(0, |acc, x| {
    //     let middle_number = x[(x.len() - 1) / 2];
    //     acc + middle_number
    // });
    sum
}

fn check_range(
    v: &[usize],
    order_after: &HashMap<usize, Vec<usize>>,
    order_before: &HashMap<usize, Vec<usize>>,
) -> bool {
    let mut correct = true;

    for (i, n) in v.iter().enumerate() {
        // check numbers before: fails if any number is before
        if let Some(should_be_after) = order_after.get(n) {
            // checks
            //println!("\tAfter {}: {:?}", n, nums);

            for j in v.iter().take(i) {
                if should_be_after.contains(j) {
                    correct = false;
                    //println!("\tError: {} before {}", j, n);
                    break;
                } else {
                    correct = true;
                }
            }
        }

        // check numbers after: fails if
        if let Some(should_be_before) = order_before.get(n) {
            // checks
            //println!("\tBefore {}: {:?}", n, nums);

            for j in v.iter().rev().take(v.len() - i) {
                if should_be_before.contains(j) {
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
    correct
}

pub fn stable_topo_sort_inner(
    edges: &[(usize, usize)],
    result: &mut Vec<usize>,
    last_index: &mut usize,
) -> bool {
    for i in 0..result.len() {
        for j in 0..i {
            let x = result[i];
            let y: usize = result[j];
            if edges.contains(&(x, y)) {
                let t = result[i].to_owned();
                result.remove(i);
                result.insert(j, t);

                *last_index = j;

                return true;
            }
        }
    }
    false
}

pub fn stable_topo_sort_opt2(
    edges: &[(usize, usize)],
    result: &mut Vec<usize>,
    last_index: &mut usize,
) -> bool {
    // optimize B: only check edges
    let mut b = false;
    for (idx, edge) in edges.iter().enumerate() {
        let i = edge.0;
        let j = edge.1;

        // let x = result[i];
        // let y = result[j];

        if let Some(idx_of_x) = result.iter().position(|f| *f == i) {
            if let Some(idx_of_y) = result.iter().position(|f| *f == j) {
                // if i not before j x should be before y
                if idx_of_x > idx_of_y {
                    let t = result[idx_of_x].to_owned();
                    result.remove(idx_of_x);
                    result.insert(idx_of_y, t);

                    *last_index = idx;

                    b = true;
                }
            }
        }
    }

    b
}
