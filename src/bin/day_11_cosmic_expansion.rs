/// Solution to an Advent of Code problem, day 11, 2023
/// https://adventofcode.com/2023/day/11

use std::env;
use std::fs;
use itertools::Itertools;

fn expand_cosmos(c: &mut Vec<Vec<u8>>) {
    // expand lines
    let mut y = 0usize;
    while y < c.len() {
        if c[y].iter().all(|&v| v == b'.') {
            c.insert(y, c[y].clone());
            y += 2;
        } else {
            y += 1;
        }
    }

    // expand columns
    let mut x = 0usize;
    while x < c[0].len() {
        if c.iter().all(|l| l[x] == b'.') {
            c.iter_mut().for_each(|f| f.insert(x, b'.'));
            x += 2;
        } else {
            x += 1;
        }
    }
}

fn expand_universe_v2(c: &Vec<Vec<u8>>, expansion_factor: usize) -> (Vec<usize>, Vec<usize>) {
    let mut x_mapping = vec![0; c[0].len()];
    let mut y_mapping = vec![0; c.len()];

    // expand lines
    let mut prev_mapped_y = 0usize;
    for y in 1..c.len() {
        if c[y].iter().all(|&v| v == b'.') {
            prev_mapped_y += expansion_factor;
        } else {
            prev_mapped_y += 1;
        }
        y_mapping[y] = prev_mapped_y;
    }

    // expand columns
    let mut prev_mapped_x = 0usize;
    for x in 1..c[0].len() {
        if c.iter().all(|l| l[x] == b'.') {
            prev_mapped_x += expansion_factor;
        } else {
            prev_mapped_x += 1;
        }
        x_mapping[x] = prev_mapped_x;
    }

    (x_mapping, y_mapping)
}

fn find_galaxies(c: &Vec<Vec<u8>>, position_mapping: (Vec<usize>, Vec<usize>)) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for y in 0..c.len() {
        for x in 0..c[y].len() {
            if c[y][x] == b'#' {
                galaxies.push((position_mapping.0[x], position_mapping.1[y]));
            }
        }
    }
    galaxies
}

fn sum_distances(galaxies: Vec<(usize, usize)>) -> usize {
    let mut distance_matrix = vec![vec![0; galaxies.len()]; galaxies.len()];
    for i in 0..galaxies.len() {
        for j in 0..galaxies.len() {
            if i != j {
                let distance = galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
                distance_matrix[i][j] = distance;
                distance_matrix[j][i] = distance;
            }
        }
    }

    let mut paths_sum = 0usize;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            paths_sum += distance_matrix[i][j];
        }
    }
    paths_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let cosmos = contents.lines().map(|f| f.bytes().into_iter().collect_vec()).collect_vec();

    let position_mapping_1 = expand_universe_v2(&cosmos, 2);
    let galaxies_1 = find_galaxies(&cosmos, position_mapping_1);
    let paths_sum_part_1 = sum_distances(galaxies_1);
    println!("Sum of path lengths [part 1]: {}", paths_sum_part_1);

    let position_mapping_2 = expand_universe_v2(&cosmos, 1_000_000);
    let galaxies_2 = find_galaxies(&cosmos, position_mapping_2);
    let paths_sum_part_2 = sum_distances(galaxies_2);
    println!("Sum of path lengths [part 2]: {}", paths_sum_part_2);
}
