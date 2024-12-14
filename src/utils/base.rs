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

pub fn run_solutions(year: u16, day: u8, input: &str, run_case: u8) -> Option<(Option<(u32, Duration)>, Option<(u32, Duration)>)> {
    match year {
        2024 => match day {
            1 => Some((
                get_solution(days_2024::day_01::one::run, input, run_case),
                get_solution(days_2024::day_01::two::run, input, run_case)
            )),
            2 => Some((
                get_solution(days_2024::day_02::one::run, input, run_case),
                get_solution(days_2024::day_02::two::run, input, run_case)
            )),
            3 => Some((
                get_solution(days_2024::day_03::one::run, input, run_case),
                get_solution(days_2024::day_03::two::run, input, run_case)
            )),
            4 => Some((
                get_solution(days_2024::day_04::one::run, input, run_case),
                get_solution(days_2024::day_04::two::run, input, run_case)
            )),
            5 => Some((
                get_solution(days_2024::day_05::one::run, input, run_case),
                get_solution(days_2024::day_05::two::run, input, run_case)
            )),
            6 => Some((
                get_solution(days_2024::day_06::one::run, input, run_case),
                get_solution(days_2024::day_06::two::run, input, run_case)
            )),
            _ => None
        }
        _ => None
    }
}

pub fn get_solution(solution_fn: fn(&str) -> u32, input: &str, run_case: u8) -> Option<(u32, std::time::Duration)> {
    let mut start_time = std::time::Instant::now();
    let result = std::panic::catch_unwind(|| solution_fn(input));
    let mut dur = start_time.elapsed();
    match result {
        Ok(solution) => {
            if run_case == 2 { return Some((solution, dur)); }

            let mut durations = Vec::<Duration>::new();

            while durations.len() < 50 {
                start_time = std::time::Instant::now();
                solution_fn(input); // local var
                dur = start_time.elapsed();
                durations.push(dur);
            }

            if run_case == 0 {
                return Some((solution, durations.iter().fold(Duration::ZERO, |acc, &dur| acc + dur) / durations.len() as u32));
            } else if run_case == 1 {
                return Some((solution, *durations.iter().min().unwrap()));
            } else {
                return None;
            }
        }
        Err(_) => None
    }
}