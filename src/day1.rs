pub fn execute() {
    let input = include_str!("../input/day1.txt");
    //let mut total = 0;

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

        //let diff = (left - right).abs();
        //total += diff;
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

    //println!("{}", total);
    println!("correct: {}", total2);
}
