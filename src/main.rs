use clap::{Arg, ArgAction, Command};
use std::path::Path;

mod advent_of_code_2024;

mod utils;

fn main() {
    let matches = Command::new("Advent of Code Runner")
        .version("1.0")
        .author("Zachary Garcia, zachary_garcia@icloud.com")
        .about("Runs Advent of Code Solutions")
        .arg(
            Arg::new("inputs")
                .help("Give day # (defaults to current year) or year & day (e.g. cargo run 2023 1)")
                .num_args(1..=2)
        )
        .arg(
            Arg::new("test")
                .short('t')
                .long("test")
                .help("Use test input (src/advent_of_code_{year}/test inputs/day_{day_num}")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("best")
                .short('b')
                .short_alias('f')
                .long("best")
                .alias("fast")
                .help("Runs solution ten times, returns fastest and  times")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("once")
                .short('o')
                .long("once")
                .help("Runs solution once (if running 50 times would take too long)")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let inputs = matches.get_many::<String>("inputs").unwrap().collect::<Vec<_>>();
    let use_test = matches.get_flag("test");
    let use_best = matches.get_flag("best");
    let run_once = matches.get_flag("once");

    let run_case: u8 = if !(use_best || run_once) { 0 } else if use_best { 1 } else { 2 };

    match inputs.len() {
        1 => { 
            let day: u8 = inputs.get(0).unwrap()
                .parse()
                .expect("Day must be a valid number (1-25)");

            run_day(*utils::base::CURRENT_YEAR, day, use_test, run_case);
        }
        2 => {
            let year: u16 = inputs.get(0).unwrap()
                .parse()
                .expect(format!("Year must be a valid number (2015-{})", *utils::base::CURRENT_YEAR).as_str());

            let day: u8 = inputs.get(1).unwrap()
                .parse()
                .expect("Day must be a valid number (1-25)");

            run_day(year, day, use_test, run_case);
        }
        _ => unreachable!()
    }
}

fn run_day(year: u16, day: u8, use_test: bool, run_case: u8) {
    let year_str: String = utils::base::get_year_str(year);
    let day_str: String = utils::base::get_day_str(day);
    
    let year_dir: String = format!("src/advent_of_code_{}", year_str);
    let day_dir: String = format!("{}/days/day_{}", year_dir, day_str);

    // check for validity
    let mut invalid: bool = false;

    if year < 2015_u16 || year > *utils::base::CURRENT_YEAR {
        eprintln!("Please enter a valid year (2015-{})", year);
        invalid = true;
    } else {
        let implemented_year: bool = Path::new(&year_dir).exists();

        if !implemented_year {
            eprintln!("Solutions for Advent of Code {} are not yet implemented", year);
            invalid = true;
        } else {
            if day > 25 {
                eprintln!("Please enter a valid day (1-25)");
                invalid = true;
            }

            let implemented_day = Path::new(&day_dir).exists();
            if !implemented_day {
                eprintln!("Solution for Advent of Code {} day {} is not yet implemented", year, day);
                invalid = true;
            }
        }
    }

    if invalid { std::process::exit(1); }

    let input_dir: String = format!(
        "{}/{}/day_{}.txt",
        year_dir,
        if !use_test { "inputs" } else { "test inputs" },
        day_str
    );

    let input: &str = &utils::base::read_file_to_str(&input_dir);

    println!("Running Advent of Code {} day {}...", year, day);

    if let Some((run_data_one, run_data_two)) = utils::base::run_solutions(year, day, input, run_case) {
        if let Some((sol_one, dur_one)) = run_data_one {
            println!("Part one: {}, ran in {:?}", sol_one, dur_one);
        }

        if let Some((sol_two, dur_two)) = run_data_two {
            println!("Part two: {}, ran in {:?}", sol_two, dur_two);
        }
        
        std::process::exit(0);
    } else {
        eprintln!("Runner file likely damaged");
        std::process::exit(1);
    }
}