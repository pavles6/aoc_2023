use std::collections::HashMap;
use std::{error::Error, fs};

const MINIMUM_LENGTH_OF_WORD_THAT_CAN_REPRESENT_A_DIGIT: usize = 3;

// Module for day 1
pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day1/input.txt")?;

    let mut calibration_values_sum: u32 = 0;

    let words_to_digits: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|&(word, digit)| (word, digit))
    .collect();

    let words_to_digits_reversed: HashMap<&str, u32> = [
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ]
    .iter()
    .map(|&(word, digit)| (word, digit))
    .collect();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let mut digits: (u32, u32) = (0, 0);

        search_line_for_digit(&chars, &words_to_digits).map(|digit| {
            digits.0 = digit;
        });

        search_line_for_digit(
            &chars.iter().rev().collect::<String>().chars().collect(),
            &words_to_digits_reversed,
        )
        .map(|digit| {
            digits.1 = digit;
        });

        let calibration_value = format!("{}{}", digits.0, digits.1).parse::<u32>().unwrap();

        calibration_values_sum += calibration_value;
    }

    println!("Calibration values sum: {}", calibration_values_sum);

    Ok(())
}

fn search_line_for_digit(line: &Vec<char>, words_to_digits: &HashMap<&str, u32>) -> Option<u32> {
    let mut word_buf = String::new();

    for char in line.into_iter() {
        if char.is_numeric() {
            return Some(char.to_digit(10).unwrap());
        }

        word_buf.push(*char);

        if word_buf.len() > MINIMUM_LENGTH_OF_WORD_THAT_CAN_REPRESENT_A_DIGIT - 1 {
            if let Some(digit) = find_digit_by_word_buffer(&word_buf, &words_to_digits) {
                return Some(digit);
            }
        }
    }

    return None;
}

fn find_digit_by_word_buffer(word_buf: &str, words_to_digits: &HashMap<&str, u32>) -> Option<u32> {
    if let Some(word) = words_to_digits.keys().find(|&key| word_buf.contains(*key)) {
        if let Some(val) = words_to_digits.get(word) {
            return Some(*val);
        }
    }
    return None;
}
