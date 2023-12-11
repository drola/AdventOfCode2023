/// Solution to an Advent of Code problem, day 08, 2023
/// https://adventofcode.com/2023/day/08

use std::env;
use std::fs;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::value;
use nom::IResult;
use nom::multi::many1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct NextNodes {
    left: [u8; 3],
    right: [u8; 3],
}

fn name_to_index(name: &[u8; 3]) -> usize {
    ((name[0] as usize * 255usize) + name[1] as usize) * 255usize + name[2] as usize
}

fn ends_with_a(name: &[u8; 3]) -> bool {
    name[2] == 'A' as u8
}

fn ends_with_z(name: &[u8; 3]) -> bool {
    name[2] == 'Z' as u8
}

fn index_to_name(i: usize) -> [u8; 3] {
    let mut out = [0u8; 3];
    let mut remainder = i;

    out[2] = (remainder % 255) as u8;
    remainder = remainder / 255;
    out[1] = (remainder % 255) as u8;
    remainder = remainder / 255;
    out[0] = remainder as u8;
    out
}

fn parse_direction(i: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R"))
    ))(i)
}

fn parse_directions(i: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(i)
}

fn parse_node_name(i: &str) -> IResult<&str, [u8; 3]> {
    let (rest, c) = take(3usize)(i)?;
    Ok((rest, c.as_bytes().try_into().unwrap()))
}

fn parse_node(i: &str) -> IResult<&str, ([u8; 3], NextNodes)> {
    let (rest, from) = parse_node_name(i)?;
    let (rest, _) = tag(" = (")(rest)?;
    let (rest, left) = parse_node_name(rest)?;
    let (rest, _) = tag(", ")(rest)?;
    let (rest, right) = parse_node_name(rest)?;

    Ok((rest, (from, NextNodes { left, right })))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines();

    let mut all_node_names = vec![];
    let (_, directions) = parse_directions(lines.next().unwrap()).unwrap();
    lines.next(); // Empty line
    let mut nodes: Vec<Option<NextNodes>> = vec![None; 255 * 255 * 255];
    for line in lines {
        let (_, (from, next_nodes)) = parse_node(line).unwrap();
        nodes[name_to_index(&from)] = Some(next_nodes);
        all_node_names.push(from);
    }

    let mut step_count_part_1 = 0u64;
    let aaa: [u8; 3] = "AAA".as_bytes().try_into().unwrap();
    let zzz: [u8; 3] = "ZZZ".as_bytes().try_into().unwrap();
    let mut position: [u8; 3] = aaa.clone();
    let mut direction_index = 0usize;
    while position != zzz {
        step_count_part_1 = step_count_part_1 + 1;
        position = match directions[direction_index] {
            Direction::Left => nodes[name_to_index(&position)].unwrap().left,
            Direction::Right => nodes[name_to_index(&position)].unwrap().right,
        };

        direction_index = (direction_index + 1) % directions.len();
    }

    println!("Step count [part 1]: {}", step_count_part_1);

    let starts_for_part_2 = all_node_names.iter().filter_map(|f| match ends_with_a(f) {
        true => Some(*f),
        _ => None
    }).collect_vec();

    let mut cycle_lengths = vec![];

    for start_node in starts_for_part_2 {
        let mut step_index = 0usize;
        let mut position = start_node;
        let mut states = vec![];
        let mut direction_index = 0usize;


        // Observation on my input data:
        // AAA has cycle of length 17873 (=293*61)
        // QRA has cycle of length 19631 (=293*67)
        // KQA has cycle of length 17287 (=293+59)
        // DFA has cycle of length 12599 (=293*43)
        // DBA has cycle of length 21389 (=293*73)
        // HJA has cycle of length 20803 (=293*71)

        // Analyze cycles
        loop {
            position = match directions[direction_index] {
                Direction::Left => nodes[name_to_index(&position)].unwrap().left,
                Direction::Right => nodes[name_to_index(&position)].unwrap().right,
            };
            step_index += 1;
            direction_index = step_index % directions.len();

            if ends_with_z(&position) {
                if states.len() > 0 && (direction_index, position) == states[0] { // made a full cycle
                    break;
                } else {
                    states.push((direction_index, position));
                }
            }
        }

        let cycle_length = step_index / (states.len() + 1);

        // It's slightly surprising to me, that cycle lengths are so perfect as we see in the following output.
        // The input in advent of caledar 2023, is apparently well crafted.
        // In general case, the path taken could be much messier.
        println!("Cycle length for {} is {} = {} * {}", String::from_utf8_lossy(&start_node), cycle_length,
                 directions.len(), cycle_length / directions.len() + cycle_length % directions.len());
        cycle_lengths.push(cycle_length);
    }
    // Also, the GCD of all cycle lengths is 293, which is the length of the directions input.
    // This simplifies calculation below:
    println!("We need to make:");
    println!("{}", directions.len());
    let mut prod = directions.len();
    for cl in cycle_lengths {
        println!("* {}", cl / directions.len());
        prod *= cl / directions.len();
    }
    println!("= {} steps.", prod);


    // TODO: The brute-force implementation below is way too slow.
    //
    // let mut step_count_part_2 = 0u64;
    // let mut positions: Vec<[u8; 3]> = all_node_names.iter().filter_map(|f| match ends_with_a(f) {
    //     true => Some(*f),
    //     _ => None
    // }).collect_vec();
    // println!("These nodes ({}) end with A: {:?}", positions.len(), positions);
    // let mut direction_index = 0usize;
    // while !positions.iter().all(ends_with_z) {
    //     step_count_part_2 = step_count_part_2 + 1;
    //
    //     for position in positions.iter_mut() {
    //         *position = match directions[direction_index] {
    //             Direction::Left => nodes[name_to_index(&position)].unwrap().left,
    //             Direction::Right => nodes[name_to_index(&position)].unwrap().right,
    //         };
    //     }
    //
    //     direction_index = (direction_index + 1) % directions.len();
    // }

    // How to decouple "ghosts" from each other?
    // pattern of positions starts repeating after directions count * nodes count steps = 293 * 754 = 240.000
    // we could compute how many steps to next Z-ending node for each possible state.

    // For each ghost we can calculate complete cycle -> when it gets back to initial node with direction_index==0. Cycle length = 240k max.
    // Then we can compute position of arbitrary ghost in arbitrary step in O(1).

    //println!("Step count [part 2]: {}", step_count_part_2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_from_name() {
        let abc: [u8; 3] = "ABC".as_bytes().try_into().unwrap();
        let def: [u8; 3] = "DEF".as_bytes().try_into().unwrap();
        assert_eq!(index_to_name(name_to_index(&abc)), abc);
        assert_eq!(index_to_name(name_to_index(&def)), def);
        assert_ne!(name_to_index(&abc), name_to_index(&def));
    }

    #[test]
    fn test_ends_with_a() {
        let cba: [u8; 3] = "CBA".as_bytes().try_into().unwrap();
        let def: [u8; 3] = "DEF".as_bytes().try_into().unwrap();
        assert_eq!(ends_with_a(&cba), true);
        assert_eq!(ends_with_a(&def), false);
    }

    #[test]
    fn test_ends_with_z() {
        let cba: [u8; 3] = "CBA".as_bytes().try_into().unwrap();
        let xyz: [u8; 3] = "XZY".as_bytes().try_into().unwrap();
        assert_eq!(ends_with_a(&cba), false);
        assert_eq!(ends_with_a(&xyz), true);
    }
}