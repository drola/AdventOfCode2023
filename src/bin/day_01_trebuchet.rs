/// Solution to an Advent of Code problem, day 01, 2023
/// https://adventofcode.com/2023/day/01

use std::env;
use std::fs;
use std::str::FromStr;

use nom::{InputLength, IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map, value};

fn parse_digit_v1(input: &str) -> IResult<&str, u64> {
    //map_parser(take(1), digit1)(input)
    map(
        take_while_m_n(1, 1, |s: char| s.is_digit(10)),
        |s| u64::from_str(s).unwrap(),
    )(input)
}


fn parse_digit_v2(input: &str) -> IResult<&str, u64> {
    alt((
        parse_digit_v1,
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn find_all_digits<'a, E, F>(input: &'a str, mut parser: F) -> Vec<u64> where F: Parser<&'a str, u64, E> {
    let mut res = Vec::new();

    let mut i = input;
    while i.input_len() > 0 {
        let pr = parser.parse(i);
        if let Ok((i2, r)) = pr {
            res.push(r);
        }
        i = &i[1..];
    }

    res
}

fn sum_up(digits: &[u64]) -> u64 {
    return if digits.len() > 0 {
        digits[0] * 10 + digits[digits.len() - 1]
    } else {
        0
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut result_v1 = 0;
    let mut result_v2 = 0;

    for line in lines {
        let digits_in_line_v1 = find_all_digits(line, parse_digit_v1);
        result_v1 += sum_up(&digits_in_line_v1);

        let digits_in_line_v2 = find_all_digits(line, parse_digit_v2);
        result_v2 += sum_up(&digits_in_line_v2);
    }

    println!("Result [part 1]: {}", result_v1);
    println!("Result [part 2]: {}", result_v2);
}
