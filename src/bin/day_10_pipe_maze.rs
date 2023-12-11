/// Solution to an Advent of Code problem, day 10, 2023
/// https://adventofcode.com/2023/day/10

use std::env;
use std::fs;
use itertools::Itertools;

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

    let mut current_position = start_coords.unwrap();
    let mut path = vec![start_coords.unwrap()];
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
        if next_step == start_coords {
            break;
        }
    }

    println!("Path: {:?}", path);
    println!("Path length: {:?}", path.len());
    println!("Longest distance: {}", path.len() / 2);

    // Counting inside area of the loop
    let mut colors = vec![vec![0i64; w]; h];

    // Mark path itself
    for (x, y) in path {
        colors[y][x] = -1;
    }

    // Left-right pass
    for y in 0..h {
        let mut v = 0;
        for x in 0..w {
            if colors[y][x] == -1 {
                if (map[y][x] != b'-') {
                    v = v + 1;
                }
            } else {
                //colors[y][x] = v;
            }
        }
    }

    // Top-down pass
    // for x in 0..w {
    //     let mut v = 0;
    //     for y in 0..h {
    //         if colors_v[y][x] == -1 {
    //             if map[y][x] != b'|' {
    //                 v = v + 1;
    //             }
    //         } else {
    //             colors_v[y][x] = v;
    //         }
    //     }
    // }

    // Visualize MAP
    for y in 0..h {
        for x in 0..w {
            print!("{:4}", colors[y][x]);
        }
        println!("");
    }

    // Inside area = count of ODD POSITIVE numbers
    let mut enclosed_area: u64 = 0;
    for y in 0..h {
        for x in 0..w {
            let color = colors[y][x];
            if color > 0 && color % 2 == 1 {
                enclosed_area += 1;
            }
        }
    }
    println!("Enclosed area [part 2]: {}", enclosed_area);
}
