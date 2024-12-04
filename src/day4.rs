const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";
const MAS: &str = "MAS";
const SAM: &str = "SAM";
const N: usize = 140;

pub fn execute(is_part_two: bool) -> i32 {
    if is_part_two {
        part_two()
    } else {
        part_one()
    }
}

fn part_two() -> i32 {
    // transform into 140x140 grid
    let input = include_bytes!("../input/day4.txt");
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

    // search for A's
    let mut indices = Vec::new();
    (1..N - 1).for_each(|i| {
        (1..N - 1).for_each(|j| {
            let c = grid[i][j] as char;
            if c == 'A' {
                // now check the surrounding
                let diag1 = format!(
                    "{}{}{}",
                    grid[i - 1][j - 1] as char,
                    c,
                    grid[i + 1][j + 1] as char
                );

                let diag2 = format!(
                    "{}{}{}",
                    grid[i - 1][j + 1] as char,
                    c,
                    grid[i + 1][j - 1] as char
                );

                if (diag1 == MAS || diag1 == SAM) && (diag2 == MAS || diag2 == SAM) {
                    indices.push((i, j));
                }
            }
        });
    });

    indices.len() as i32
}

fn part_one() -> i32 {
    // transform into 140x140 grid
    let input = include_bytes!("../input/day4.txt");
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

    (0..N).for_each(|i| {
        let mut hbuffer = String::new();
        let mut vbuffer = String::new();
        (0..N).for_each(|j| {
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
        });
    });

    let mut found_diagonal = 0;
    let mut found_diagonal2 = 0;

    // now check it diagonally 789
    (0..N).for_each(|i| {
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
    });

    found_horizontal + found_vertical + found_diagonal + found_diagonal2
}
