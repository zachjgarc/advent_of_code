use chrono::prelude::*;
use once_cell::sync::Lazy;
use std::time::Duration;

use crate::advent_of_code_2024::days as days_2024;

pub static CURRENT_YEAR: Lazy<u16> = Lazy::new(|| Utc::now().year() as u16);

pub fn get_day_str(day: u8) -> String {
    if day < 10 {
        return format!("0{}", day);
    } else {
        return day.to_string();
    }
}

pub fn get_year_str(year: u16) -> String {
    return year.to_string();
}

pub fn read_file_to_str(dir: &str) -> String {
    match std::fs::read_to_string(dir) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file: {}", dir);
            std::process::exit(1);
        }
    }
}

pub fn run_solutions(year: u16, day: u8, input: &String) -> Option<(Option<(u32, Duration)>, Option<(u32, Duration)>)> {
    match year {
        2024 => match day {
            1 => Some((
                time_solution(days_2024::day_01::one::run, input),
                time_solution(days_2024::day_01::two::run, input)
            )),
            2 => Some((
                time_solution(days_2024::day_02::one::run, input),
                time_solution(days_2024::day_02::two::run, input)
            )),
            3 => Some((
                time_solution(days_2024::day_03::one::run, input),
                time_solution(days_2024::day_03::two::run, input)
            )),
            4 => Some((
                time_solution(days_2024::day_04::one::run, input),
                time_solution(days_2024::day_04::two::run, input)
            )),
            5 => Some((
                time_solution(days_2024::day_05::one::run, input),
                time_solution(days_2024::day_05::two::run, input)
            )),
            6 => Some((
                time_solution(days_2024::day_06::one::run, input),
                time_solution(days_2024::day_06::two::run, input)
            )),
            _ => None
        }
        _ => None
    }
}

pub fn time_solution(solution_fn: fn(&String) -> u32, input: &String) -> Option<(u32, std::time::Duration)> {
    let start_time = std::time::Instant::now();
    let result = std::panic::catch_unwind(|| solution_fn(input));
    match result {
        Ok(solution) => Some((solution, start_time.elapsed())),
        Err(_) => None
    }
}