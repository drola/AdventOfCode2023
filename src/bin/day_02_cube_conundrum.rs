/// Solution to an Advent of Code problem, day 02, 2023
/// https://adventofcode.com/2023/day/02

use std::env;
use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::IResult;
use nom::multi::{count, separated_list0};
use nom::sequence::tuple;


#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

fn max(a: &CubeSet, b: &CubeSet) -> CubeSet {
    CubeSet {
        red: std::cmp::max(a.red, b.red),
        green: std::cmp::max(a.green, b.green),
        blue: std::cmp::max(a.blue, b.blue),
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u64,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.cube_sets.iter().all(|cube_set| cube_set.red <= 12 && cube_set.green <= 13 && cube_set.blue <= 14)
    }

    fn min_cube_set(&self) -> CubeSet {
        self.cube_sets.iter().fold(CubeSet::new(), |acc, cube_set| max(&acc, cube_set))
    }
}

fn cube_set(input: &str) -> IResult<&str, CubeSet> {
    let count_and_color = tuple((nom::character::complete::u64, space0,
                                 alt((tag("red"), tag("green"), tag("blue")))
    ));
    let (input, counts_and_colors) = separated_list0(tag(", "), count_and_color)(input)?;

    let mut r = CubeSet { red: 0, green: 0, blue: 0 };
    for (count, _, color) in counts_and_colors {
        match color {
            "red" => {
                r.red = count;
            }
            "green" => {
                r.green = count;
            }
            "blue" => {
                r.blue = count;
            }
            _ => panic!("Unknown color??")
        }
    }
    Ok((input, r))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::u64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = separated_list0(tag("; "), cube_set)(input)?;

    Ok((input, Game {
        id,
        cube_sets,
    }))
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut games = vec![];
    for line in lines {
        if let Ok((_, game_)) = game(line) {
            games.push(game_);
        }
    }

    // println!("{:?}", games);

    let sum_of_ids_of_possible_games = games.iter().filter(|g| g.is_possible()).fold(0, |acc, g| acc + g.id);
    println!("Sum of ids of possible games [part 1]: {}", sum_of_ids_of_possible_games);

    let sum_of_powers_of_min_cube_sets: u64 = games.iter().map(|g| g.min_cube_set().power()).sum();
    println!("Sum of powers of min cube sets [part 2]: {}", sum_of_powers_of_min_cube_sets);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cube_set() {
        assert_eq!(cube_set("1 red, 2 green, 6 blue"), Ok(("", CubeSet { red: 1, green: 2, blue: 6 })));
    }

    #[test]
    fn test_game() {
        assert_eq!(game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
                   Ok(("", Game {
                       id: 5,
                       cube_sets: vec![
                           CubeSet { red: 6, green: 3, blue: 1 },
                           CubeSet { red: 1, green: 2, blue: 2 },
                       ],
                   })));
    }
}