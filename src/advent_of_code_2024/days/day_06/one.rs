#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &String) -> u32 {
    let mut map: Vec<Vec<_>> = _input.lines().map(|line| line.chars().collect()).collect();

    let orientations: Vec<(i32,i32)> = vec![(-1,0), (0,1), (1,0), (0,-1)];

    let raw_pos = _input.find('^').unwrap();
    let width = map[0].len() + 1;
    let mut pos: (i32,i32) = ((raw_pos / width) as i32, (raw_pos % width) as i32);

    let mut rotations = 0_usize;
    
    while let Some(c) = map.get(pos.0 as usize).unwrap_or(&Vec::<char>::new()).get(pos.1 as usize) {
        if c == &'#' {
            pos.0 -= orientations[rotations % 4].0;
            pos.1 -= orientations[rotations % 4].1;

            rotations += 1;
            pos.0 += orientations[rotations % 4].0;
            pos.1 += orientations[rotations % 4].1;

            continue;
        }
        
        map[pos.0 as usize][pos.1 as usize] = 'X';
        pos.0 += orientations[rotations % 4].0;
        pos.1 += orientations[rotations % 4].1;
    }

    map.iter().flatten().filter(|c| **c == 'X').count() as u32
}