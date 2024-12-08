use std::collections::HashMap;

const N: usize = 50; //50;

pub fn execute(is_part_two: bool) -> usize {
    let input = include_bytes!("../input/day8.txt");

    let mut cnt = 0;
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for b in input.iter() {
        // skip newline
        if b.is_ascii_control() {
            continue;
        }

        let y = cnt / N;
        let x = cnt % N;

        if b != &b'.' {
            let c = *b as char;
            if let Some(v) = map.get_mut(&c) {
                v.push((x, y));
            } else {
                map.insert(c, vec![(x, y)]);
            }
        }

        cnt += 1;
    }

    //println!("{:?}", map);

    // iterate through in pairs
    let mut nodes = Vec::new();
    for key in map.keys() {
        let v = map.get(key).unwrap();
        //println!("{:?} {:?}", key, v);

        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (x1, y1) = v[i];
                let (x2, y2) = v[j];

                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                if is_part_two {
                    nodes.push((x1, y1));
                    nodes.push((x2, y2));
                }

                {
                    let mut cnt = 0;
                    let mut is_out_of_bounds = false;
                    let mut ax = x1 as i32 - dx;
                    let mut ay = y1 as i32 - dy;
                    while !is_out_of_bounds {
                        // hacky break
                        if !is_part_two && cnt > 0 {
                            break;
                        }

                        // check if out of bounds
                        if ax < 0 || ax >= N as i32 || ay < 0 || ay >= N as i32 {
                            is_out_of_bounds = true;
                        } else {
                            nodes.push((ax as usize, ay as usize));

                            ax -= dx;
                            ay -= dy;
                        }

                        cnt += 1;
                    }
                }

                {
                    let mut cnt = 0;
                    let mut is_out_of_bounds = false;
                    let mut bx = x2 as i32 + dx;
                    let mut by = y2 as i32 + dy;
                    while !is_out_of_bounds {
                        // hacky break
                        if !is_part_two && cnt > 0 {
                            break;
                        }

                        // check if out of bounds
                        if bx < 0 || bx >= N as i32 || by < 0 || by >= N as i32 {
                            is_out_of_bounds = true;
                        } else {
                            nodes.push((bx as usize, by as usize));

                            bx += dx;
                            by += dy;
                        }

                        cnt += 1;
                    }
                }
            }
        }
    }

    nodes.sort();
    nodes.dedup();

    //println!("{:?}", nodes);

    nodes.len()
}
