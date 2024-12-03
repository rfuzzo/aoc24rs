const INSTR_MUL: &str = "mul(";
const INSTR_MUL_END: char = ')';
const INSTR_SEP: char = ',';
const INSTR_START: &str = "do()";
const INSTR_END: &str = "don't()";

pub fn execute(part_two: bool) -> i32 {
    let input = include_str!("../input/day3.txt");

    let mut total = 0;
    let mut enabled = true;
    let mut reading = false;
    let mut buffer = String::new();

    for c in input.chars() {
        buffer += &c.to_string();

        if part_two {
            // Check for the start of a do
            if !enabled && buffer.ends_with(INSTR_START) {
                enabled = true;
                buffer.clear();
                continue;
            }

            // Check for the end of a don't
            if enabled && buffer.ends_with(INSTR_END) {
                enabled = false;
                buffer.clear();
                continue;
            }

            // Skip if not enabled
            if !enabled {
                continue;
            }
        }

        // Check for the start of a mul
        if buffer.ends_with(INSTR_MUL) {
            reading = true;

            // strip everything before the mul
            buffer = buffer[buffer.len() - INSTR_MUL.len()..].to_string();

            continue;
        }

        // Check for the end of a mul
        if c == INSTR_MUL_END && reading {
            if let Some(output) = process_buffer(&buffer) {
                total += output;
            }
            buffer.clear();
            reading = false;
        }
    }

    total
}

fn process_buffer(buffer: &str) -> Option<i32> {
    if !buffer.starts_with(INSTR_MUL) {
        return None;
    }
    if !buffer.ends_with(INSTR_MUL_END) {
        return None;
    }

    let inner_buffer = buffer[INSTR_MUL.len()..buffer.len() - 1].to_string();

    // ignore non-digit characters and , (44)
    for c in inner_buffer.chars() {
        if !c.is_ascii_digit() && c != INSTR_SEP {
            return None;
        }
    }

    // println!("Processing buffer: {}", buffer);

    let parts = inner_buffer.split(INSTR_SEP).collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }

    let a = parts[0].parse::<i32>().ok()?;
    let b = parts[1].parse::<i32>().ok()?;

    Some(a * b)
}
