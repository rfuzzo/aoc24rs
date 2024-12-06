use std::vec;

const N: usize = 130;
const OBSTACLE: u8 = b'#';
const START: u8 = b'^';

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct SCursor {
    pos: (usize, usize),
    dir: Direction,
}

impl SCursor {
    fn is_inside(&self) -> bool {
        self.pos.0 < N - 1 && self.pos.1 < N - 1 && self.pos.0 > 0 && self.pos.1 > 0
    }

    fn can_move(&self, grid: &[[u8; 130]; 130]) -> bool {
        let check_pos = match self.dir {
            Direction::Up => {
                // check if we can move up
                (self.pos.0, self.pos.1 - 1)
            }
            Direction::Down => {
                // check if we can move down
                (self.pos.0, self.pos.1 + 1)
            }
            Direction::Left => {
                // check if we can move left
                (self.pos.0 - 1, self.pos.1)
            }
            Direction::Right => {
                // check if we can move right
                (self.pos.0 + 1, self.pos.1)
            }
        };

        let resolved = grid[check_pos.0][check_pos.1];
        resolved != OBSTACLE
    }

    fn do_move(&mut self) {
        match self.dir {
            Direction::Up => {
                self.pos.1 -= 1;
            }
            Direction::Down => {
                self.pos.1 += 1;
            }
            Direction::Left => {
                self.pos.0 -= 1;
            }
            Direction::Right => {
                self.pos.0 += 1;
            }
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    // to string
    // fn debug(&self) -> String {
    //     format!("({:?},{:?}) {:?}", self.pos.1 + 1, self.pos.0 + 1, self.dir)
    // }
}

pub fn execute(is_part_two: bool) -> usize {
    // parse input
    // transform into grid
    let input = include_bytes!("../input/day6.txt");
    let mut cnt = 0;
    let mut grid: [[u8; N]; N] = [[0; N]; N];
    let mut start_pos = (0, 0);
    for c in input.iter() {
        // skip newline
        if c.is_ascii_control() {
            continue;
        }

        // assemble grid
        let y = cnt / N;
        let x = cnt % N;
        grid[x][y] = *c;

        if *c == START {
            start_pos = (x, y);
        }

        cnt += 1;
    }
    //println!("Start: {:?}", cursor.debug());
    let start = SCursor {
        pos: start_pos,
        dir: Direction::Up,
    };
    let visited = calculate_path(&grid, &start).unwrap(); // hacky but we can unwrap here since we know there is a solution
    let mut visited_positions = visited
        .iter()
        .map(|f| f.pos)
        .collect::<Vec<(usize, usize)>>();
    visited_positions.sort();
    visited_positions.dedup();

    let total = visited_positions.len();

    if !is_part_two {
        // only count unique positions
        total
    } else {
        let mut wrapping_configurations_count = 0;
        println!("Visited positions: {:?}", total);

        for (i, p) in visited_positions.iter().enumerate() {
            if *p == start.pos {
                continue;
            }

            // replace p with obstacle
            let mut test_grid = grid;
            test_grid[p.0][p.1] = OBSTACLE;

            let wrapping = calculate_path(&test_grid, &start).is_none();
            if wrapping {
                println!("{}/{}: {:?} is wrapping", i, total, p);
                wrapping_configurations_count += 1;
            } else {
                println!("{}/{}: {:?} not wrapping", i, total, p);
            }
        }

        wrapping_configurations_count
    }
}

fn calculate_path(grid: &[[u8; 130]; 130], start: &SCursor) -> Option<Vec<SCursor>> {
    let mut cursor = start.clone();
    let mut visited = vec![cursor.clone()];

    while cursor.is_inside() {
        // check if we can move
        if cursor.can_move(grid) {
            // move in direction
            cursor.do_move();
        } else {
            // turn right
            cursor.turn_right();
        }

        if !visited.contains(&cursor) {
            visited.push(cursor.clone());
        } else {
            return None;
        }

        //println!("{}: {:?}", visited.len(), cursor.debug());
    }

    Some(visited)
}
