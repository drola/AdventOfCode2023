/// Solution to an Advent of Code problem, day 03, 2023
/// https://adventofcode.com/2023/day/03

use std::env;
use std::fs;
use std::ops::Range;
use itertools::Itertools;
use nom::Slice;

#[derive(Debug)]
struct Number {
    y: usize,
    x_range: Range<usize>,
    value: u64,
}

impl Number {
    fn adjacent_cells(&self, schematic: &Vec<&[u8]>) -> Vec<(usize, usize)> {
        let w = schematic[0].len();
        let h = schematic.len();

        let mut cells = vec![];

        // Up-left
        if self.x_range.start > 0 && self.y > 0 {
            cells.push((self.x_range.start - 1, self.y - 1));
        }

        // Up
        if self.y > 0 {
            for x in self.x_range.clone() {
                cells.push((x, self.y - 1));
            }
        }

        // Up-Right
        if self.x_range.end + 1 < w && self.y > 0 {
            cells.push((self.x_range.end, self.y - 1));
        }

        // Right
        if self.x_range.end + 1 < w {
            cells.push((self.x_range.end, self.y));
        }

        // Down-Right
        if self.x_range.end + 1 < w && self.y + 1 < h {
            cells.push((self.x_range.end, self.y + 1));
        }

        // Down
        if self.y + 1 < h {
            for x in self.x_range.clone() {
                cells.push((x, self.y + 1));
            }
        }

        // Down-Left
        if self.x_range.start > 0 && self.y + 1 < h {
            cells.push((self.x_range.start - 1, self.y + 1));
        }

        // Left
        if self.x_range.start > 0 {
            cells.push((self.x_range.start - 1, self.y));
        }

        cells
    }

    fn is_part(&self, schematic: &Vec<&[u8]>) -> bool {
        for (x, y) in self.adjacent_cells(schematic) {
            let b = schematic[y][x];
            if !b.is_ascii_digit() && b != b'.' {
                return true;
            }
        }
        false
    }

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        return (x >= std::cmp::max(self.x_range.start,1)-1 && x <= self.x_range.end &&
            (self.y + 1 == y || self.y == y + 1))
        || (self.x_range.start > 0 && x == self.x_range.start-1 && self.y == y) // Left
        || (x == self.x_range.end && self.y == y); // Right
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let schematic = contents.lines().map(|l| l.as_bytes()).collect_vec();
    let w = schematic[0].len();
    let h = schematic.len();
    let mut numbers: Vec<Number> = vec![];

    for y in 0..h {
        let mut x = 0;
        while x < w {
            if schematic[y][x].is_ascii_digit() {
                let x_start = x;
                let mut x_end = x + 1;
                while x_end < w && schematic[y][x_end].is_ascii_digit() {
                    x_end = x_end + 1;
                }

                let value = std::str::from_utf8(schematic[y].slice(x_start..x_end)).unwrap().parse::<u64>().unwrap();
                x = x_end;
                numbers.push(Number {
                    y,
                    x_range: x_start..x_end,
                    value,
                });
            } else {
                x = x + 1;
            }
        }
    }

    // Filter which numbers are parts
    let parts = numbers.iter().filter(|n| n.is_part(&schematic)).collect_vec();

    let parts_sum: u64 = parts.iter().map(|p| p.value).sum();
    println!("Parts sum [part 1]: {}", parts_sum);

    let mut gear_ratios = 0;
    for y in 0..h {
        for x in 0..w {
            if schematic[y][x] == b'*' {
                let adjacent_parts = parts.iter().filter(|p| p.is_adjacent(x, y)).collect_vec();
                println!("{:?}", adjacent_parts);
                if adjacent_parts.len() == 2 {
                    gear_ratios = gear_ratios + adjacent_parts[0].value * adjacent_parts[1].value;
                }
            }
        }
    }
    println!("Gear ratios sum [part 2]: {}", gear_ratios);
}
