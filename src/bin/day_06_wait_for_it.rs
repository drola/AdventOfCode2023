/// Solution to an Advent of Code problem, day 06, 2023
/// https://adventofcode.com/2023/day/06

use std::env;
use std::fs;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug)]
struct Game {
    time: u64,
    distance: u64,
}


fn distance_travelled(holding_time: u64, time: u64) -> u64 {
    let speed = holding_time;
    let travelling_time = time - holding_time;
    travelling_time * speed
}

impl Game {
    fn ways_to_win(&self) -> u64 {
        let mut count = 0;
        for holding_time in 0..self.time {
            let distance = distance_travelled(holding_time, self.time);
            if distance > self.distance {
                count = count + 1;
            }
        }
        count
    }
}

fn parse_times(s: &str) -> IResult<&str, Vec<u64>> {
    let (rest, (_, _, times)) = tuple((tag("Time:"), space0, separated_list1(space1, nom::character::complete::u64)))(s)?;
    Ok((rest, times))
}

fn parse_distances(s: &str) -> IResult<&str, Vec<u64>> {
    let (rest, (_, _, distances)) = tuple((tag("Distance:"), space0, separated_list1(space1, nom::character::complete::u64)))(s)?;
    Ok((rest, distances))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let mut lines = contents.lines();
    let (_, times) = parse_times(lines.next().unwrap()).unwrap();
    let (_, distances) = parse_distances(lines.next().unwrap()).unwrap();
    let games = times.into_iter().zip(distances).map(|(time, distance)| Game { time, distance }).collect_vec();

    let ways_to_win_part_1: u64 = games.iter().map(|g| g.ways_to_win()).product();
    println!("Ways to win [part 1]: {}", ways_to_win_part_1);


    // Part 2: reread the file, ignoring spaces
    let mut lines = contents.lines();
    let (_, times) = parse_times(lines.next().unwrap().replace(" ", "").as_str()).unwrap();
    let (_, distances) = parse_distances(lines.next().unwrap().replace(" ", "").as_str()).unwrap();
    let games = times.into_iter().zip(distances).map(|(time, distance)| Game { time, distance }).collect_vec();
    let ways_to_win_part_2: u64 = games.iter().map(|g| g.ways_to_win()).product();
    println!("Ways to win [part 2]: {}", ways_to_win_part_2);
}
