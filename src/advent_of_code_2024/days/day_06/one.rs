#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &str) -> u32 {
    let mut map = _input.as_bytes().to_vec();

    let orientations = [(-1,0), (0,1), (1,0), (0,-1)];

    let mut pos = _input.find('^').unwrap() as i32;
    let width = map.iter().position(|c| c == &b'\n').unwrap_or(map.len()) as i32 + 1;

    let mut orientation_index = 0_usize;
    let (mut dy, mut dx) = (-1, 0);

    map[pos as usize] = b'X';
    let mut unique_visited = 1_usize;

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
        
        if map[pos as usize] != b'X' {
            map[pos as usize] = b'X';
            unique_visited += 1;
        }
        
        pos += dx + dy * width;
        if pos < 0 { break; }
    }

    unique_visited as u32
}