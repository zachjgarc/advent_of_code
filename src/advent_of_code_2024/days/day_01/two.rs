#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &String) -> u32 {
    let mut counts = HashMap::<u32,(u32,u32)>::new();

    for line in _input.lines() {
        let mut line_data = line.split_whitespace().map(|num| num.parse::<u32>().unwrap());
        counts.entry(line_data.next().unwrap()).or_insert((0,0)).0 += 1;
        counts.entry(line_data.next().unwrap()).or_insert((0,0)).1 += 1;
    }

    counts.iter().fold(0_u32, |total, (num, (left, right))| total + num * left * right)
}