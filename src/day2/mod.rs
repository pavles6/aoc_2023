use std::{error::Error, fs};

const MAX_RED_CUBES_PER_SET: u32 = 12;
const MAX_GREEN_CUBES_PER_SET: u32 = 13;
const MAX_BLUE_CUBES_PER_SET: u32 = 14;

// Module for day 2
pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day2/input.txt")?;

    let mut valid_game_id_sum = 0;

    let mut game_set_power_sum = 0;

    for game in input.lines() {
        let data_section_start_idx = game.find(|x| x == ':').unwrap();

        let game_id: String = game.chars().take(data_section_start_idx).collect();

        let game_id_parts: Vec<&str> = game_id.split(" ").collect();
        let game_id = game_id_parts[1];

        let data_section = game
            .chars()
            .skip(data_section_start_idx + 2) // Skip ": "
            .collect::<String>();

        let mut cube_sets: Vec<&str> = data_section.split(";").collect();

        let mut game_valid = true;

        let mut least_required_cubes: (u32, u32, u32) = (0, 0, 0); // red, green, blue

        for cube_set in cube_sets.iter_mut() {
            for cube_data in cube_set.split(",") {
                let cube_data = cube_data.trim().split(" ").collect::<Vec<&str>>();

                let cube_count = cube_data[0].parse::<u32>()?;
                let cube_color = cube_data[1];

                match cube_color {
                    "red" => {
                        if least_required_cubes.0 < cube_count {
                            least_required_cubes.0 = cube_count;
                        }
                        if cube_count > MAX_RED_CUBES_PER_SET {
                            game_valid = false
                        }
                    }
                    "green" => {
                        if least_required_cubes.1 < cube_count {
                            least_required_cubes.1 = cube_count;
                        }
                        if cube_count > MAX_GREEN_CUBES_PER_SET {
                            game_valid = false
                        }
                    }
                    "blue" => {
                        if least_required_cubes.2 < cube_count {
                            least_required_cubes.2 = cube_count;
                        }
                        if cube_count > MAX_BLUE_CUBES_PER_SET {
                            game_valid = false
                        }
                    }
                    _ => {
                        panic!("Invalid cube color: {}", cube_color);
                    }
                }
            }
        }

        let game_set_power =
            least_required_cubes.0 * least_required_cubes.1 * least_required_cubes.2;

        game_set_power_sum += game_set_power;

        if game_valid {
            valid_game_id_sum += game_id.parse::<u32>().unwrap();
        }
    }

    println!("Valid game ID sum: {}", valid_game_id_sum);

    println!("Game set power sum: {}", game_set_power_sum);
    Ok(())
}
