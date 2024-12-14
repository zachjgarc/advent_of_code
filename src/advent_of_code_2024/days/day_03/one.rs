#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(_input)
        .map(|instance|
            instance.as_str().split(",")
                .fold(1_u32, |a,b|
                    a * b.chars().filter(|c| c.is_digit(10))
                        .collect::<String>().parse::<u32>().unwrap()
                )
        )
        .sum()
}