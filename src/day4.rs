const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";
const N: usize = 140;

pub fn execute(part_two: bool) -> i32 {
    let input = include_bytes!("../input/day4.txt");

    // transform into 140x140 grid
    let mut cnt = 0;
    let mut grid: [[u8; N]; N] = [[0; N]; N];
    for c in input.iter() {
        // skip newline
        if c.is_ascii_control() {
            continue;
        }

        // assemble grid
        grid[cnt / N][cnt % N] = *c;
        cnt += 1;
    }

    let mut found_horizontal = 0;
    let mut found_vertical = 0;

    // check grid horizontally and vertically

    for i in 0..N {
        let mut hbuffer = String::new();
        let mut vbuffer = String::new();
        for j in 0..N {
            hbuffer.push(grid[i][j] as char);
            vbuffer.push(grid[j][i] as char);

            // check buffer horizontally
            if hbuffer.ends_with(XMAS) || hbuffer.ends_with(SAMX) {
                found_horizontal += 1;
            }
            // check buffer vertically
            if vbuffer.ends_with(XMAS) || vbuffer.ends_with(SAMX) {
                found_vertical += 1;
            }
        }
    }

    let mut found_diagonal = 0;
    let mut found_diagonal2 = 0;

    // now check it diagonally 789
    for i in 0..N {
        // upper triangle
        {
            let mut buffer = String::new();
            (0..=i).for_each(|k| {
                let c = grid[k][i - k];
                buffer.push(c as char);

                if buffer.ends_with(XMAS) || buffer.ends_with(SAMX) {
                    found_diagonal += 1;
                }
            });
        }

        // lower triangle
        if i > 0 {
            let mut buffer = String::new();
            (i..N).for_each(|k| {
                let c = grid[k][N - 1 + i - k];
                buffer.push(c as char);

                if buffer.ends_with(XMAS) || buffer.ends_with(SAMX) {
                    found_diagonal += 1;
                }
            });
        }

        // OTHER DIRECTION

        // upper triangle
        {
            let mut buffer = String::new();
            (0..=i).for_each(|k| {
                let c = grid[k][N - 1 - i + k];
                buffer.push(c as char);

                if buffer.ends_with(XMAS) || buffer.ends_with(SAMX) {
                    found_diagonal2 += 1;
                }
            });
        }

        // lower triangle
        if i > 0 {
            let mut buffer = String::new();
            (i..N).for_each(|k| {
                let c = grid[k][k - i];
                buffer.push(c as char);

                if buffer.ends_with(XMAS) || buffer.ends_with(SAMX) {
                    found_diagonal2 += 1;
                }
            });
        }
    }

    // println!("found_horizontal: {}", found_horizontal);
    // println!("found_vertical: {}", found_vertical);
    // println!("found_diagonal 1: {}", found_diagonal);
    // println!("found_diagonal 2: {}", found_diagonal2);
    // println!("===");

    found_horizontal + found_vertical + found_diagonal + found_diagonal2
}
