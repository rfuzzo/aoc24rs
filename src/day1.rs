pub fn execute() -> i32 {
    let input = include_str!("../input/day1.txt");
    //let mut total = 0;

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

        //let diff = (left - right).abs();
        //total += diff;
    }

    // sort the vectors
    left_v.sort_unstable();
    right_v.sort_unstable();

    let mut total2 = 0;

    // iterate over the vectors and get the difference
    for (i, l) in left_v.iter().enumerate() {
        let r = right_v[i];
        let d = (l - r).abs();

        total2 += d;
    }

    //println!("{}", total);
    //println!("correct: {}", total2);
    total2
}
