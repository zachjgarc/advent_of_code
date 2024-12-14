use std::u8;

#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &str) -> u32 {
    let map = _input.as_bytes().to_vec();

    let orientations = [(-1,0), (0,1), (1,0), (0,-1)];

    let initial_pos = _input.find('^').unwrap() as i32;
    let width = map.iter().position(|c| c == &b'\n').unwrap_or(map.len()) as i32 + 1;

    let mut pos;

    let mut orientation_index: usize;
    let (mut dy, mut dx): (i32, i32);

    let mut result = 0_usize;

    for i in 0..map.len() {
        let mut map = map.clone();

        if map[i] != b'.' { continue; }

        map[i] = b'#';

        pos = initial_pos;

        orientation_index = 0;
        (dy, dx) = (-1, 0);

        map[pos as usize] = (orientation_index % 4) as u8;
        pos += dx + dy * width;
        
        while let Some(c) = map.get(pos as usize) {
            if c == &b'\n' { break; }

            if c == &b'#' {
                pos -= dx + dy * width;

                orientation_index += 1;
                (dy, dx) = orientations[orientation_index % 4];
                pos += dx + dy * width;

                continue;
            }
            
            if map[pos as usize] == (orientation_index % 4) as u8 {
                result += 1;
                break;
            }

            map[pos as usize] = (orientation_index % 4) as u8;
            
            pos += dx + dy * width;
            if pos < 0 { break; }
        }

        map[i] = b'.';
    }

    result as u32
}