/// Solution to an Advent of Code problem, day 12, 2023
/// https://adventofcode.com/2023/day/12

use std::env;
use std::fs;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::value;
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<u64>,
}

impl Record {
    fn unfold(&self) -> Record {
        let new_springs = vec![self.springs.clone(); 5].join(&Spring::Unknown);
        Record {
            springs: new_springs,
            groups: self.groups.repeat(5),
        }
    }

    // Up to the first ?, removes known springs and groups
    fn trim(mut self) -> Option<Record> {
        let mut new_start_springs = 0usize;
        let mut new_start_groups = 0usize;

        let mut i = 0;
        while i < self.springs.len() {
            if self.springs[i] == Spring::Unknown {
                break;
            } else if self.springs[i] == Spring::Operational {
                i += 1;
            } else if self.springs[i] == Spring::Damaged { // group starting here
                // find group end
                let mut group_end_maybe = None;
                for j in i..self.springs.len() {
                    if self.springs[j] == Spring::Operational {
                        group_end_maybe = Some(j);
                        break;
                    } else if self.springs[j] == Spring::Unknown {
                        break;
                    }
                }

                if let Some(group_end) = group_end_maybe {
                    let group_length = (group_end - i) as u64;
                    if self.groups.len() > new_start_groups && group_length == self.groups[new_start_groups] {
                        new_start_groups += 1;
                        new_start_springs = group_end + 1;
                        i = new_start_springs;
                    } else {
                        return None;
                    }
                } else {
                    break;
                }
            }
        }

        self.springs.drain(0..new_start_springs);
        self.groups.drain(0..new_start_groups);
        Some(self)
    }

    fn is_resolved(&self) -> bool {
        self.springs.iter().all(|&s| s != Spring::Unknown)
    }

    fn with_first_resolved(&self, resolved_as: Spring) -> Record {
        let mut springs = self.springs.clone();
        for i in 0..springs.len() {
            if springs[i] == Spring::Unknown {
                springs[i] = resolved_as;
                break;
            }
        }

        Record {
            springs,
            groups: self.groups.clone(),
        }
    }
}

fn parse_spring(i: &str) -> IResult<&str, Spring> {
    alt((
        value(Spring::Damaged, tag("#")),
        value(Spring::Operational, tag(".")),
        value(Spring::Unknown, tag("?"))
    ))(i)
}

fn parse_numbers(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), nom::character::complete::u64)(i)
}

fn parse_record(i: &str) -> Record {
    let (_, (springs, _, groups)) = tuple((many1(parse_spring), space1, parse_numbers))(i).unwrap();
    Record { springs, groups }
}

fn is_valid(springs: &[Spring], groups: &Vec<u64>) -> bool {
    let mut groups_in_s = Vec::with_capacity(groups.len());
    let mut current_group_length = 0;
    for s in springs {
        if *s == Spring::Damaged {
            current_group_length += 1;
        } else if *s == Spring::Operational && current_group_length > 0 {
            groups_in_s.push(current_group_length);
            current_group_length = 0;
        }
    }
    if current_group_length > 0 {
        groups_in_s.push(current_group_length);
    }

    *groups == groups_in_s
}


fn is_valid_till_first_unknown(springs: &[Spring], groups: &Vec<u64>) -> bool {
    let mut damaged_count = 0u64;
    for s in springs {
        if *s == Spring::Damaged {
            damaged_count += 1;
        }
    }
    let total_count_across_groups = groups.iter().sum::<u64>();
    if damaged_count > total_count_across_groups {
        return false;
    }

    // Forward
    let mut current_group_length = 0u64;
    let mut current_group_index = 0usize;
    for s in springs {
        if *s == Spring::Damaged {
            current_group_length += 1;
        } else if *s == Spring::Operational && current_group_length > 0 {
            if current_group_index < groups.len() && groups[current_group_index] == current_group_length {
                current_group_length = 0;
                current_group_index += 1;
            } else {
                return false;
            }
        } else if *s == Spring::Unknown {
            current_group_length = 0;
            break;
        }
    }
    if current_group_length > 0 {
        if !(current_group_index < groups.len() && groups[current_group_index] == current_group_length) {
            return false;
        }
    }

    // Backward
    let mut current_group_length = 0;
    let mut current_group_index = 0;
    for s in springs.iter().rev() {
        if *s == Spring::Damaged {
            current_group_length += 1;
        } else if *s == Spring::Operational && current_group_length > 0 {
            if groups.len() > current_group_index && groups[groups.len() - 1 - current_group_index] == current_group_length {
                current_group_length = 0;
                current_group_index += 1;
            } else {
                return false;
            }
        } else if *s == Spring::Unknown {
            current_group_length = 0;
            break;
        }
    }
    if current_group_length > 0 {
        if !(groups.len() > current_group_index && groups[groups.len() - 1 - current_group_index] == current_group_length) {
            return false;
        }
    }

    true
}

fn count_combinations_brute_force(r: &Record) -> u64 {
    let mut valid_combinations_count = 0;

    // generating combinations:
    // we can count "Unknown" springs
    // there are 2^unknown_count combinations
    let unknown_springs_count = r.springs.iter().filter(|&&s| s == Spring::Unknown).count() as u32;
    let combination_count = 2u64.pow(unknown_springs_count);
    for c in 0..combination_count {
        let mut d = c;

        let mut springs = r.springs.clone();
        for s in springs.iter_mut() {
            if *s == Spring::Unknown {
                *s = match d % 2 {
                    0 => Spring::Damaged,
                    1 => Spring::Operational,
                    _ => panic!("Should not happen!")
                };
                d /= 2;
            }
        }

        if is_valid(&springs, &r.groups) {
            valid_combinations_count += 1;
        }
    }

    valid_combinations_count
}

fn count_combinations_recursive(r: &Record) -> u64 {
    if r.is_resolved() {
        return match is_valid(&r.springs, &r.groups) {
            true => 1,
            false => 0
        };
    }

    if !is_valid_till_first_unknown(&r.springs, &r.groups) {
        return 0;
    }

    let option1 = r.with_first_resolved(Spring::Operational).trim();
    let option2 = r.with_first_resolved(Spring::Damaged).trim();

    let mut count = 0;
    if let Some(option) = option1 {
        count += count_combinations_recursive(&option);
    }
    if let Some(option) = option2 {
        count += count_combinations_recursive(&option);
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let records = contents.lines().map(parse_record).collect_vec();

    let combinations_sum = records.iter().map(count_combinations_brute_force).sum::<u64>();
    println!("Combinations sum [part 1]: {}", combinations_sum);

    let combinations_sum = records.iter().map(count_combinations_recursive).sum::<u64>();
    println!("Combinations sum [part 1, recursive]: {}", combinations_sum);

    // For part 2, brute force will not be fast enough.
    // After analysing the input, there can be up to 75 damaged springs to allocate between

    let unfolded_records = records.iter().map(|r| r.unfold()).collect_vec();

    // Statistics in preparation for part 2.
    let mut max_unknown_damaged_count = 0;
    let mut max_unknown_count = 0;
    for r in unfolded_records {
        let mut unknown_count = 0u64;
        let mut damaged_count = 0u64;
        let mut operational_count = 0u64;
        for s in r.springs {
            match s {
                Spring::Operational => { operational_count += 1; }
                Spring::Damaged => { damaged_count += 1; }
                Spring::Unknown => { unknown_count += 1; }
            }
        }
        let total_damaged_count = r.groups.iter().sum::<u64>();
        let unknown_damaged_count = total_damaged_count - damaged_count;
        if unknown_damaged_count > max_unknown_damaged_count {
            max_unknown_damaged_count = unknown_damaged_count;
        }
        if unknown_count > max_unknown_count {
            max_unknown_count = unknown_count;
            println!("max_unknown_count={} with unknown_damaged={}", unknown_count, unknown_damaged_count);
        }
    }

    println!("Max unknown damaged count: {}", max_unknown_damaged_count);
    println!("Max unknown count: {}", max_unknown_count);

    // In worst examples, we'd be brute-forcing 10e25 combinations.

    // Potential recursion? - leads to smaller sub-problems
    // ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
    //  -> pick & trim
    // #??.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1)
    // .??.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.2)

    // From 1.1
    // #.?.###????.###????.###????.###????.### _,1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1.1)
    // ##?.###????.###????.###????.###????.### _,1,3,1,1,3,1,1,3,1,1,3,1,1,3 IMPOSSIBLE
    // Trimmed:
    // ?.###????.###????.###????.###????.### 1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1.1)
    // From 1.1.1
    // ..###????.###????.###????.###????.### 1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1.1.1) - IMPOSSIBLE
    // #.###????.###????.###????.###????.### 1,3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1.1.2)
    // Trimmed:
    // ###????.###????.###????.###????.### 3,1,1,3,1,1,3,1,1,3,1,1,3 (1.1.1.2)
    // ... here we'll also quickly loose one of the branches.
    let combinations_sum = records.par_iter().map(|r| {
        println!("...");
        count_combinations_recursive(&r.unfold())
    }).sum::<u64>();
    println!("Combinations sum [part 2, recursive]: {}", combinations_sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_trim() {
        assert_eq!(parse_record(".??..??...?##. 1,1,3").trim(), Some(parse_record(".??..??...?##. 1,1,3")));
        assert_eq!(parse_record("..?..??...?##. 1,1,3").trim(), Some(parse_record("..?..??...?##. 1,1,3")));
        assert_eq!(parse_record(".#?..??...?##. 1,1,3").trim(), Some(parse_record(".#?..??...?##. 1,1,3")));
        assert_eq!(parse_record(".#...??...?##. 1,1,3").trim(), Some(parse_record("..??...?##. 1,3")));
    }
}
