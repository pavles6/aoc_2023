use std::{env, error::Error};

use aoc_2023::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21,
    day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "day1" => day1::run()?,
            "day2" => day2::run(),
            "day3" => day3::run(),
            "day4" => day4::run(),
            "day5" => day5::run(),
            "day6" => day6::run(),
            "day7" => day7::run(),
            "day8" => day8::run(),
            "day9" => day9::run(),
            "day10" => day10::run(),
            "day11" => day11::run(),
            "day12" => day12::run(),
            "day13" => day13::run(),
            "day14" => day14::run(),
            "day15" => day15::run(),
            "day16" => day16::run(),
            "day17" => day17::run(),
            "day18" => day18::run(),
            "day19" => day19::run(),
            "day20" => day20::run(),
            "day21" => day21::run(),
            "day22" => day22::run(),
            "day23" => day23::run(),
            "day24" => day24::run(),
            "day25" => day25::run(),
            _ => println!("Module not recognized"),
        };
    } else {
        println!("Please specify a module to run (day1, day2, ... day25)");
    }

    Ok(())
}
