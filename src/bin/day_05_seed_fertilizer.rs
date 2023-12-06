#![feature(iter_array_chunks)]

/// Solution to an Advent of Code problem, day 05, 2023
/// https://adventofcode.com/2023/day/05

use std::cmp::min;
use std::env;
use std::fs;
use std::str::FromStr;

use itertools::Itertools;
use nom::{Finish, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug, PartialEq)]
struct IntervalMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

fn parse_interval_map(input: &str) -> IResult<&str, IntervalMap> {
    let (rest, (
        destination_start, _, source_start, _, length
    )) = tuple((nom::character::complete::u64,
                space1,
                nom::character::complete::u64,
                space1,
                nom::character::complete::u64))(input)?;

    Ok((rest, IntervalMap {
        source_start,
        destination_start,
        length,
    }))
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (rest, (_, lst)) = tuple((tag("seeds: "), separated_list1(
        space1,
        nom::character::complete::u64,
    )))(input)?;

    Ok((rest, lst))
}

impl FromStr for IntervalMap {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_interval_map(s).finish() {
            Ok((_remaining, interval_map)) => Ok(interval_map),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
struct Interval {
    from: u64,
    length: u64,
}

/**
mapping must be sorted by source_start!
 */
fn project_interval(interval: Interval, mapping: &Vec<IntervalMap>) -> Vec<Interval> {
    let mut output: Vec<Interval> = vec![];

    let mut interval_from = interval.from;
    let mut interval_to = interval_from + interval.length;


    let mut map_index: usize = 0;

    while interval_from < interval_to && mapping.len() > map_index {
        // Left of the interval map
        if mapping[map_index].source_start > interval_from {
            let from = interval_from;
            let to = min(interval_to, mapping[map_index].source_start);
            // println!("[case1] map_index: {}, from: {}, to: {}", map_index, from, to);
            output.push(Interval { from, length: to - from });
            interval_from = to;
        }

        if interval_from < interval_to && mapping[map_index].source_start <= interval_from && mapping[map_index].source_start + mapping[map_index].length > interval_from {
            let from = interval_from;
            let to = min(interval_to, mapping[map_index].source_start + mapping[map_index].length);
            // println!("[case2] map_index: {}, from: {}, to: {}", map_index, from, to);
            output.push(Interval {
                from: from - mapping[map_index].source_start + mapping[map_index].destination_start,
                length: to - from,
            });
            interval_from = to;
        }

        map_index = map_index + 1;
    }

    // Right of all interval maps
    if interval_from < interval_to {
        output.push(Interval { from: interval_from, length: interval_to - interval_from });
    }

    output
}

// part 1
fn interpret_numbers_as_individual_seeds(numbers: &Vec<u64>) -> Vec<Interval> {
    numbers.iter().map(|&n| { Interval { from: n, length: 1 } }).collect_vec()
}

// part 2
fn interpret_numbers_as_intervals_of_seeds(numbers: &Vec<u64>) -> Vec<Interval> {
    numbers.iter().array_chunks().map(|[&from, &length]| Interval { from, length }).collect_vec()
}

fn map_intervals(mut intervals: Vec<Interval>, mappings: &Vec<Vec<IntervalMap>>) -> Vec<Interval> {
    for mapping in mappings {
        intervals = intervals.into_iter().flat_map(|i| project_interval(i, mapping)).collect_vec();
    }
    intervals
}

fn min_number(intervals: Vec<Interval>) -> u64 {
    intervals.iter().min_by_key(|i| i.from).unwrap().from
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines();

    let (_, numbers) = parse_seed_list(lines.next().unwrap()).unwrap();

    lines.next(); // skip empty line
    let mut mappings: Vec<Vec<IntervalMap>> = vec![];
    for _ in 0..7 {
        let mut mapping: Vec<IntervalMap> = vec![];
        lines.next(); // skip title
        while let Some(Ok(im)) = lines.next().map(|l| l.parse::<IntervalMap>()) {
            mapping.push(im);
        }
        mapping.sort_by_key(|m| m.source_start); // Because map_intervals assumes mappings are sorted!!!
        mappings.push(mapping);
    }

    let intervals = map_intervals(interpret_numbers_as_individual_seeds(&numbers), &mappings);
    let min_location_part_1 = min_number(intervals);
    println!("Min location [part 1]: {}", min_location_part_1);

    let intervals = map_intervals(interpret_numbers_as_intervals_of_seeds(&numbers), &mappings);
    let min_location_part_2 = min_number(intervals);
    println!("Min location [part 2]: {}", min_location_part_2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interval_map_from_str() {
        assert_eq!(Ok(IntervalMap {
            source_start: 5,
            destination_start: 4,
            length: 6,
        }), "4 5 6".parse());

        assert_eq!(Ok(IntervalMap {
            source_start: 11,
            destination_start: 0,
            length: 42,
        }), "0 11 42".parse());
    }

    #[test]
    fn test_project_interval() {
        assert_eq!(vec![Interval { from: 100, length: 300 }], project_interval(Interval { from: 100, length: 300 }, &vec![IntervalMap { source_start: 0, destination_start: 1, length: 10 }, IntervalMap { source_start: 401, destination_start: 403, length: 20 }]));
        assert_eq!(vec![Interval { from: 1, length: 3 }], project_interval(Interval { from: 0, length: 3 }, &vec![IntervalMap { source_start: 0, destination_start: 1, length: 10 }, IntervalMap { source_start: 401, destination_start: 403, length: 20 }]));
        assert_eq!(vec![Interval { from: 500, length: 3 }], project_interval(Interval { from: 500, length: 3 }, &vec![IntervalMap { source_start: 0, destination_start: 1, length: 10 }, IntervalMap { source_start: 401, destination_start: 403, length: 20 }]));

        println!("LAST");
        assert_eq!(vec![
            Interval { from: 1, length: 10 },
            Interval { from: 10, length: 401 - 10 },
            Interval { from: 403, length: 20 },
            Interval { from: 421, length: 500 - 421 }], project_interval(Interval { from: 0, length: 500 }, &vec![IntervalMap { source_start: 0, destination_start: 1, length: 10 }, IntervalMap { source_start: 401, destination_start: 403, length: 20 }]));


        assert_eq!(vec![Interval { from: 53, length: 4 }, Interval { from: 61, length: 9 }], project_interval(Interval { from: 57, length: 13 }, &vec![
            IntervalMap { source_start: 0, destination_start: 42, length: 7 },
            IntervalMap { source_start: 7, destination_start: 57, length: 4 },
            IntervalMap { source_start: 11, destination_start: 0, length: 42 },
            IntervalMap { source_start: 53, destination_start: 49, length: 8 },
        ]))
    }
}

