#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &str) -> u32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in _input.lines() {
        let mut line_data = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        left_list.push(line_data.next().unwrap());
        right_list.push(line_data.next().unwrap());
    }

    left_list.sort();
    right_list.sort();

    left_list.iter().zip(right_list.iter())
        .fold(0_i32, |total, (left, right)| total + (left - right).abs()) as u32
}