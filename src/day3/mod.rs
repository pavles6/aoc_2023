use std::{collections::HashMap, error::Error};

// Module for day 3

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut lines = include_str!("input.txt").lines().enumerate().peekable();

    let mut prev_line: Option<&str> = None;

    let mut gear_numbers: HashMap<(usize, usize), Vec<i32>> = HashMap::new(); // (x, y): gear ("*") coords => [x, y]

    let mut part_numbers_sum = 0;

    while let Some((curr_line_idx, curr_line)) = lines.next() {
        let next_line = lines.peek();

        let mut line_start_idx = 0;

        while let Some((start, end)) = get_number_idx_range(curr_line, line_start_idx) {
            let is_part = is_part_number(
                start,
                end,
                prev_line,
                curr_line,
                next_line,
                curr_line_idx,
                &mut gear_numbers,
            );

            if is_part {
                let part_number = curr_line[start..end].parse::<i32>()?;
                part_numbers_sum += part_number;
            }

            line_start_idx = end + 1;
        }

        prev_line = Some(curr_line);
    }

    println!("Part numbers sum: {:?}", part_numbers_sum);

    println!(
        "Gear numbers sum: {:?}",
        gear_numbers
            .values()
            .filter(|x| x.len() > 1)
            .map(|x| x[0] * x[1])
            .reduce(|acc, curr| acc + curr)
            .unwrap()
    );

    Ok(())
}

// number is a part number if any of its adjancencies characters are not numeric
fn is_part_number(
    start: usize,
    end: usize,
    prev_line: Option<&str>,
    curr_line: &str,
    next_line: Option<&(usize, &str)>,
    curr_line_idx: usize,
    gear_numbers: &mut HashMap<(usize, usize), Vec<i32>>,
) -> bool {
    let mut symbol_in_prev = false;
    let mut symbol_in_next = false;

    let number = &curr_line[start..end];

    let line_start = if start == 0 { 0 } else { start - 1 };
    let line_end = if end == curr_line.len() {
        curr_line.len()
    } else {
        end + 1
    };

    if let Some(prev_line) = prev_line {
        symbol_in_prev = scan_for_symbol_and_check_for_gear_numbers(
            line_start,
            line_end,
            prev_line,
            curr_line_idx - 1,
            number,
            gear_numbers,
        );
    }

    if let Some((nex_line_idx, next_line)) = next_line {
        symbol_in_next = scan_for_symbol_and_check_for_gear_numbers(
            line_start,
            line_end,
            *next_line,
            *nex_line_idx,
            number,
            gear_numbers,
        );
    }

    return symbol_in_prev
        || symbol_in_next
        || scan_for_symbol_and_check_for_gear_numbers(
            line_start,
            line_end,
            curr_line,
            curr_line_idx,
            number,
            gear_numbers,
        );
}

fn scan_for_symbol_and_check_for_gear_numbers(
    start: usize,
    end: usize,
    line: &str,
    line_idx: usize,
    number: &str,
    gear_numbers: &mut HashMap<(usize, usize), Vec<i32>>,
) -> bool {
    let mut is_symbol = false;

    line[start..end].char_indices().for_each(|(idx, ch)| {
        // skip check if we already know it's a symbol
        if !is_symbol {
            is_symbol = !ch.is_alphanumeric() && ch != '.';
        }

        // check if curr number can also be gear number
        if ch == '*' {
            let gear_nums = gear_numbers.entry((line_idx, start + idx)).or_default();

            gear_nums.push(number.parse::<i32>().unwrap());
        }
    });

    return is_symbol;
}

fn get_number_idx_range(line: &str, start_from: usize) -> Option<(usize, usize)> {
    let mut start_idx = None;
    let mut end_idx = None;

    if start_from >= line.len() {
        return None;
    }

    for (idx, ch) in line.char_indices().skip(start_from) {
        if ch.is_numeric() {
            if start_idx == None {
                start_idx = Some(idx);
            }

            if idx == line.len() - 1 {
                if start_idx == None {
                    start_idx = Some(idx);
                }

                end_idx = Some(line.len()); // end_idx should be inclusive if it's end of line since we use it as a range
                break;
            }
        } else {
            if start_idx != None {
                end_idx = Some(idx); // end_idx should be inclusive if it's end of line since we use it as a range
                break;
            }
        }
    }

    if start_idx == None || end_idx == None {
        return None;
    }

    return Some((start_idx.unwrap(), end_idx.unwrap()));
}
