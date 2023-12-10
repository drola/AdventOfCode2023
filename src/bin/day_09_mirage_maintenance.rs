/// Solution to an Advent of Code problem, day 09, 2023
/// https://adventofcode.com/2023/day/09

use std::env;
use std::fs;
use itertools::Itertools;
use nom::character::complete::space1;
use nom::IResult;
use nom::multi::separated_list0;

fn extrapolate(history: &[i64]) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![history.to_vec()];

    // Compute diffs
    while histories.last().unwrap().iter().any(|&v| v != 0) {
        let last_history = histories.last().unwrap();
        let mut diffs = vec![0; last_history.len() - 1];
        for (i, diff) in diffs.iter_mut().enumerate() {
            *diff = last_history[i + 1] - last_history[i];
        }
        histories.push(diffs);
    }

    // Extrapolate up
    let mut next = 0;
    for history in histories.iter().rev().skip(1) {
        next = history.last().unwrap() + next;
    }

    next
}

fn extrapolate_back(history: &[i64]) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![history.to_vec()];

    // Compute diffs
    while histories.last().unwrap().iter().any(|&v| v != 0) {
        let last_history = histories.last().unwrap();
        let mut diffs = vec![0; last_history.len() - 1];
        for (i, diff) in diffs.iter_mut().enumerate() {
            *diff = last_history[i + 1] - last_history[i];
        }
        histories.push(diffs);
    }

    // Extrapolate up
    let mut previous = 0;
    for history in histories.iter().rev().skip(1) {
        previous = history[0] - previous;
    }

    previous
}

fn parse_list(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(space1, nom::character::complete::i64)(i)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines().map(|l| parse_list(l).unwrap().1).collect_vec();

    let sum_of_extrapolations_part_1: i64 = lines.iter().map(|v| extrapolate(v)).sum();
    println!("Sum of extrapolations [part 1]: {}", sum_of_extrapolations_part_1);

    let sum_of_back_extrapolations_part_2: i64 = lines.iter().map(|v| extrapolate_back(v)).sum();
    println!("Sum of back extrapolations [part 2]: {}", sum_of_back_extrapolations_part_2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn test_extrapolate_back() {
        assert_eq!(extrapolate_back(&vec![10, 13, 16, 21, 30, 45]), 5);
    }
}