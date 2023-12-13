/// Solution to an Advent of Code problem, day 13, 2023
/// https://adventofcode.com/2023/day/13

use std::env;
use std::fs;
use nom::character::complete::{newline, not_line_ending};
use nom::combinator::{map, verify};
use nom::IResult;
use nom::multi::{separated_list1};
use nom::sequence::tuple;

fn parse_patterns(i: &str) -> IResult<&str, Vec<Vec<Vec<u8>>>> {
    separated_list1(tuple((newline, newline)), separated_list1(newline,
                                                               map(/* without this "verify", we just blast through the empty lines */ verify(not_line_ending, |l: &str| l.len() > 0),
                                                                   |f: &str| f.as_bytes().to_vec())))(i)
}

fn is_mirrored_x(pattern: &Vec<Vec<u8>>, axis: usize, required_smudges: usize) -> bool {
    //println!("is_mirrored_x({})", axis);
    let w = pattern[0].len();
    let mut smudge_count = 0usize;

    for line in pattern {
        for dx in 0..w {
            if axis + dx < w && axis >= dx + 1 {
                let a = axis + dx;
                let b = axis - dx - 1;
                if line[a] != line[b] {
                    //println!("is_mirrored_x({}): {} != {} (line[{}] != line[{}]", axis, line[a], line[b], a, b);
                    smudge_count += 1;
                }
            }
        }
    }

    smudge_count == required_smudges
}

fn is_mirrored_y(pattern: &Vec<Vec<u8>>, axis: usize, required_smudges: usize) -> bool {
    let w = pattern[0].len();
    let h = pattern.len();
    let mut smudge_count = 0usize;

    for x in 0..w {
        for dy in 0..h {
            if axis + dy < h && axis >= dy + 1 {
                let a = axis + dy;
                let b = axis - dy - 1;
                if pattern[a][x] != pattern[b][x] {
                    smudge_count += 1;
                }
            }
        }
    }

    smudge_count == required_smudges
}

fn find_axis(pattern: &Vec<Vec<u8>>, required_smudges: usize) -> u64 {
    let w = pattern[0].len();
    let h = pattern.len();

    // x
    for x in 1..w {
        if is_mirrored_x(pattern, x, required_smudges) {
            return x as u64;
        }
    }

    // y
    for y in 1..h {
        if is_mirrored_y(pattern, y, required_smudges) {
            return 100u64 * y as u64;
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let patterns = parse_patterns(&contents).unwrap().1;

    let sum_part_1 = patterns.iter().map(|p| find_axis(p, 0)).sum::<u64>();
    println!("Sum [part 1]: {}", sum_part_1);
    let sum_part_2 = patterns.iter().map(|p| find_axis(p, 1)).sum::<u64>();
    println!("Sum [part 2]: {}", sum_part_2);
}
