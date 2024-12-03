use std::collections::HashMap;

pub fn execute(is_part_two: bool) -> i32 {
    if is_part_two {
        part_two()
    } else {
        part_one()
    }
}

fn part_two() -> i32 {
    let input = include_str!("../input/day1.txt");

    let mut right_map = HashMap::new();
    let mut left_v = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let left: i32 = line[0..5]
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse left: {}", i));
        let right: i32 = line[8..]
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse right: {}", i));

        left_v.push(left);

        if let Some(x) = right_map.get_mut(&right) {
            *x += 1;
        } else {
            right_map.insert(right, 1);
        }
    }

    let mut total = 0;

    // iterate over the vectors and get the difference
    for l in left_v.iter() {
        let mut count = 0;
        if let Some(x) = right_map.get(l) {
            count = *x;
        }

        total += l * count;
    }

    total
}

fn part_one() -> i32 {
    let input = include_str!("../input/day1.txt");

    let mut left_v = Vec::new();
    let mut right_v = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let left: i32 = line[0..5]
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse left: {}", i));
        let right: i32 = line[8..]
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse right: {}", i));

        left_v.push(left);
        right_v.push(right);
    }

    // sort the vectors
    left_v.sort_unstable();
    right_v.sort_unstable();

    let mut total = 0;

    // iterate over the vectors and get the difference
    for (i, l) in left_v.iter().enumerate() {
        let r = right_v[i];
        let d = (l - r).abs();

        total += d;
    }

    total
}
