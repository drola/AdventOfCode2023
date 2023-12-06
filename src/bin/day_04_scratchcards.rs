/// Solution to an Advent of Code problem, day 04, 2023
/// https://adventofcode.com/2023/day/04

use std::env;
use std::fs;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u64>,
    my_numbers: Vec<u64>,
}

impl Card {
    fn score(&self) -> u64 {
        let matching_count = self.matching_count();

        if matching_count > 0 {
            u64::pow(2, matching_count - 1)
        } else {
            0
        }
    }

    fn matching_count(&self) -> u32 {
        let mut matching_count: u32 = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(n) {
                matching_count = matching_count + 1;
            }
        }
        matching_count
    }
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (rest, (_, _, _, _, _, winning_numbers, _, _, my_numbers)) = tuple((
        tag("Card"),
        space1,
        nom::character::complete::u64,
        tag(":"),
        space1,
        separated_list1(space1, nom::character::complete::u64),
        tag(" |"),
        space1,
        separated_list1(space1, nom::character::complete::u64),
    )
    )(s)?;

    Ok((rest, Card { winning_numbers, my_numbers }))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
    let cards = lines.map(|l| parse_card(l).unwrap().1).collect_vec();
    //println!("{:?}", cards);
    let scores_part_1: u64 = cards.iter().map(|c| c.score()).sum();
    println!("Total points [part 1]: {}", scores_part_1);

    let mut instance_counts: Vec<u64> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matching_count = card.matching_count();
        for j in i+1..i+1+matching_count as usize{
            instance_counts[j] = instance_counts[j] + instance_counts[i];
        }
    }
    let cards_count_part_2: u64 = instance_counts.iter().sum();
    println!("Cards count [part 2]: {}", cards_count_part_2);
}
