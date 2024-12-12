#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &String) -> u32 {
    let mut multiply = true;

    let re = Regex::new(r"mul\(\d+,\d+\)|(do|don't)\(\)").unwrap();

    re.find_iter(_input)
        .map(|x| x.as_str())
        .fold(0_u32, |total, item| {
            if item.chars().nth(0).unwrap() == 'm' {
                if multiply {
                    let nums = item.split(',')
                        .map(|x| 
                            x.chars()
                                .filter(|c| c.is_digit(10))
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap()
                        ).collect::<Vec<_>>();
                    return total + nums.get(0).unwrap() * nums.get(1).unwrap();
                }
            } else {
                multiply = item.len() < 5;
            }
            total
        })
}