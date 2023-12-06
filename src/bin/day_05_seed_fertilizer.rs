/// Solution to an Advent of Code problem, day 05, 2023
/// https://adventofcode.com/2023/day/05

use std::env;
use std::env::current_exe;
use std::fs;
use std::str::FromStr;
use nom::{Finish, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{space1};
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

fn flatten_mapping(a:&Vec<IntervalMap>, b: &Vec<IntervalMap>) -> Vec<IntervalMap> {
    // interesting edge points = all starts/ends from A + reverse mapped (through A) starts/ends from B.

    // TODO

    let mut result = vec![];
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines();

    let (_, seeds) = parse_seed_list(lines.next().unwrap()).unwrap();

    let mut mappings: Vec<Vec<IntervalMap>> = vec![];
    for i in 0..7 {
        let mut mapping: Vec<IntervalMap> = vec![];
        lines.next(); // skip empty line
        lines.next(); // skip title
        while let Some(Ok(im)) = lines.next().map(|l| l.parse::<IntervalMap>()) {
            mapping.push(im);
        }
        mappings.push(mapping);
    }

    let mut min_location_part_1: Option<u64> = None;
    for seed in seeds {
        let mut current = seed;
        print!("{}", current);

        for mapping in &mappings {
            for im in mapping {
                if current >= im.source_start && current < im.source_start + im.length {
                    current = current - im.source_start + im.destination_start;
                    print!(" -> {}", current);
                    break;
                }
            }
        }
        println!("");

        if min_location_part_1.is_none() || min_location_part_1.unwrap() > current {
            min_location_part_1 = Some(current);
        }
    }

    // println!("{:?}", mappings);
    println!("Min location [part 1]: {}", min_location_part_1.unwrap());


    /**
    For part 2, we have maaaany seeds (intervals!)
    Count of seeds > 10e7 --> iterating over all seeds = impractical
    Binary search of min location? ---> no go. Reverse-mapping does not give us ordering (=information whether we're above or below the target)

    Optimizations:
     - it's possible to reduce all 7 mappings to one mapping (=flatten).
     - for each interval map in this mapping, we need to find lowest seed number. mapping this lowest seed number returns the lowest possible location, that this interval map would produce
     - find min location among all interval maps.
    **/
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interval_map_from_str() {
        assert_eq!(Ok(IntervalMap {
            source_start: 4,
            destination_start: 5,
            length: 6,
        }), "4 5 6".parse());
    }
}

