/// Solution to an Advent of Code problem, day 10, 2023
/// https://adventofcode.com/2023/day/10

use std::env;
use std::fs;
use itertools::Itertools;
use nom::AsChar;

#[derive(Debug, Clone)]
struct Pipe {
    connections: Vec<(usize, usize)>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let map = contents.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let w = map[0].len();
    let h = map.len();

    // Pass 1: find "S"
    let mut start_coords = None;
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == b'S' {
                start_coords = Some((x, y));
            }
        }
    }
    let start_coords = start_coords.unwrap();

    let mut connections = vec![vec![Pipe { connections: vec![] }; w]; h];

    // Pass 2: extract lateral connections (west-east)
    for y in 0..h {
        for x in 0..(w - 1) {
            let current = map[y][x];
            let next = map[y][x + 1];
            if current == b'-' || current == b'L' || current == b'F' || current == b'S' {
                if next == b'-' || next == b'J' || next == b'7' || next == b'S' {
                    connections[y][x].connections.push((x + 1, y));
                    connections[y][x + 1].connections.push((x, y));
                }
            }
        }
    }

    // Pass 3: extract longitudinal connections (north-south)
    for x in 0..w {
        for y in 0..(h - 1) {
            let current = map[y][x];
            let next = map[y + 1][x];
            if current == b'|' || current == b'7' || current == b'F' || current == b'S' {
                if next == b'|' || next == b'L' || next == b'J' || next == b'S' {
                    connections[y][x].connections.push((x, y + 1));
                    connections[y + 1][x].connections.push((x, y));
                }
            }
        }
    }

    let mut current_position = start_coords;
    let mut path = vec![start_coords];
    loop {
        let mut next_step = None;

        for maybe_next_step in &connections[current_position.1][current_position.0].connections {
            if path.len() < 2 || path[path.len() - 2] != *maybe_next_step {
                next_step = Some(*maybe_next_step);
                break;
            }
        }


        current_position = next_step.unwrap();
        path.push(next_step.unwrap());
        if next_step.unwrap() == start_coords {
            break;
        }
    }

    // Fill in "S" at start coords with actual symbol:
    let start_out = path[1]; // First after start
    let delta_out = (start_out.0 as i64 - start_coords.0 as i64, start_out.1 as i64 - start_coords.1 as i64);
    let start_in = path[path.len() - 2]; // Last before start
    let delta_in = (start_coords.0 as i64 - start_in.0 as i64, start_coords.1 as i64 - start_in.1 as i64);


    //println!("Path: {:?}", path);
    //println!("Path length: {:?}", path.len());
    println!("Longest distance [part 1]: {}", path.len() / 2);

    let start_symbol = match (delta_in, delta_out) {
        ((1, 0), (1, 0)) => b'-', /*-S-*/
        ((-1, 0), (-1, 0)) => b'-', /*-S-*/
        ((0, 1), (0, 1)) => b'|',
        ((0, -1), (0, -1)) => b'|',

        ((1, 0), (0, 1)) => b'7',
        ((0, -1), (-1, 0)) => b'7',
        ((0, 1), (1, 0)) => b'L',
        ((-1, 0), (0, -1)) => b'L',
        ((1, 0), (0, -1)) => b'J',
        ((0, 1), (-1, 0)) => b'J',
        ((0, -1), (1, 0)) => b'F',
        ((-1, 0), (0, 1)) => b'F',

        (a, b) => panic!("Unrecognized case: ({:?}, {:?})!", a, b)
    };
    println!("Start symbol: {}", start_symbol.as_char());

    // Counting inside area of the loop
    let mut colors = vec![vec![b'.'; w]; h];

    // Mark path itself
    for (x, y) in path {
        colors[y][x] = match map[y][x] {
            b'S' => start_symbol,
            s => s
        };
    }

    // ray-trace, row by row
    // The idea behind the algorithm is from https://elixirforum.com/t/advent-of-code-2023-day-10/60279/4
    for y in 0..h {
        let mut inside = false;
        let mut corners = vec![];
        for x in 0..w {
            if colors[y][x] == b'.' {
                colors[y][x] = match inside {
                    true => b'I',
                    false => b'.'
                };
            } else if colors[y][x] == b'-' {
                // This case is horrendously uninteresting in horizontal ray trace.
            } else if colors[y][x] == b'|' {
                inside = !inside;
            } else {
                corners.push(colors[y][x]);
                if corners.len() >= 2 {
                    let flip_inside_outside = match (corners[corners.len() - 2], corners[corners.len() - 1]) {
                        (b'L', b'7') => true,
                        (b'L', b'J') => false,
                        (b'F', b'7') => false,
                        (b'F', b'J') => true,
                        _ => false
                    };
                    if flip_inside_outside {
                        inside = !inside;
                    }
                }
            }
        }
    }


    // print out map
    for y in &colors {
        println!("{}", String::from_utf8(y.clone()).unwrap());
    }

    // Count INSIDE cells
    let inside_cells_count: u64 = colors.iter().map(|l| l.iter().map(|v| match v {
        b'I' => 1,
        _ => 0
    }).sum::<u64>()).sum();
    println!("Inside cells count [part 2]: {}", inside_cells_count);
}
