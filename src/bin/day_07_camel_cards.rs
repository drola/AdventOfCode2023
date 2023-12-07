use std::cmp::Ordering;
/// Solution to an Advent of Code problem, day 07, 2023
/// https://adventofcode.com/2023/day/07

use std::env;
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

use nom::branch::alt;
use nom::character::complete::{char, space1};
use nom::combinator::value;
use nom::IResult;
use nom::multi::fill;
use nom::sequence::tuple;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Card {
    Null,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum CardWithJoker {
    Null,
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

#[derive(PartialOrd, PartialEq, Debug, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
}

#[derive(PartialEq, Eq, Debug)]
struct HandWithJoker {
    cards: [CardWithJoker; 5],
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Self> for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ht_self = self.hand_type();
        let ht_other = other.hand_type();

        match ht_self.cmp(&ht_other) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards)
        }
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        let ht_self = self.hand_type();
        let ht_other = other.hand_type();

        match ht_self.cmp(&ht_other) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards)
        }
    }
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    alt((
        value(Card::A, char('A')),
        value(Card::K, char('K')),
        value(Card::Q, char('Q')),
        value(Card::J, char('J')),
        value(Card::T, char('T')),
        value(Card::N9, char('9')),
        value(Card::N8, char('8')),
        value(Card::N7, char('7')),
        value(Card::N6, char('6')),
        value(Card::N5, char('5')),
        value(Card::N4, char('4')),
        value(Card::N3, char('3')),
        value(Card::N2, char('2')),
    ))(s)
}

fn parse_card_with_joker(s: &str) -> IResult<&str, CardWithJoker> {
    alt((
        value(CardWithJoker::A, char('A')),
        value(CardWithJoker::K, char('K')),
        value(CardWithJoker::Q, char('Q')),
        value(CardWithJoker::J, char('J')),
        value(CardWithJoker::T, char('T')),
        value(CardWithJoker::N9, char('9')),
        value(CardWithJoker::N8, char('8')),
        value(CardWithJoker::N7, char('7')),
        value(CardWithJoker::N6, char('6')),
        value(CardWithJoker::N5, char('5')),
        value(CardWithJoker::N4, char('4')),
        value(CardWithJoker::N3, char('3')),
        value(CardWithJoker::N2, char('2')),
    ))(s)
}

fn parse_hand(s: &str) -> IResult<&str, Hand> {
    let mut cards: [Card; 5] = [Card::Null; 5];
    let (rest, ()) = fill(parse_card, &mut cards)(s)?;
    Ok((&rest, Hand { cards }))
}

fn parse_hand_with_joker(s: &str) -> IResult<&str, HandWithJoker> {
    let mut cards: [CardWithJoker; 5] = [CardWithJoker::Null; 5];
    let (rest, ()) = fill(parse_card_with_joker, &mut cards)(s)?;
    Ok((&rest, HandWithJoker { cards }))
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hand(s) {
            Ok((_, hand)) => Ok(hand),
            _ => Err(())
        }
    }
}

impl FromStr for HandWithJoker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hand_with_joker(s) {
            Ok((_, hand)) => Ok(hand),
            _ => Err(())
        }
    }
}


impl Hand {
    fn hand_type(&self) -> HandType {
        let mut frequencies: [Option<(u64, Card)>; 5] = [None; 5]; // pairs of (count, card)
        for card in self.cards {
            for i in 0..5 {
                if frequencies[i].is_none() {
                    frequencies[i] = Some((1, card));
                    break;
                } else if frequencies[i].is_some() && frequencies[i].unwrap().1 == card {
                    frequencies[i] = Some((frequencies[i].unwrap().0 + 1, card));
                    break;
                }
            }
        }

        frequencies.sort();
        frequencies.reverse();

        match (frequencies[0].clone(), frequencies[1].clone()) {
            (Some((5, _)), _) => HandType::FiveOfAKind,
            (Some((4, _)), _) => HandType::FourOfAKind,
            (Some((3, _)), Some((2, _))) => HandType::FullHouse,
            (Some((3, _)), _) => HandType::ThreeOfAKind,
            (Some((2, _)), Some((2, _))) => HandType::TwoPair,
            (Some((2, _)), _) => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
}

impl HandWithJoker {
    fn hand_type(&self) -> HandType {
        let mut frequencies: [Option<(u64, CardWithJoker)>; 5] = [None; 5]; // pairs of (count, card)
        let mut joker_count = 0u64;

        for card in self.cards {
            if card == CardWithJoker::J {
                joker_count = joker_count + 1;
                continue;
            }
            for i in 0..5 {
                if frequencies[i].is_none() {
                    frequencies[i] = Some((1, card));
                    break;
                } else if frequencies[i].is_some() && frequencies[i].unwrap().1 == card {
                    frequencies[i] = Some((frequencies[i].unwrap().0 + 1, card));
                    break;
                }
            }
        }

        frequencies.sort();
        frequencies.reverse();

        match (frequencies[0].clone(), frequencies[1].clone(), joker_count) {
            (Some((5, _)), _, 0) => HandType::FiveOfAKind,
            (Some((4, _)), _, 1) => HandType::FiveOfAKind,
            (Some((3, _)), _, 2) => HandType::FiveOfAKind,
            (Some((2, _)), _, 3) => HandType::FiveOfAKind,
            (Some((1, _)), _, 4) => HandType::FiveOfAKind,
            (_, _, 5) => HandType::FiveOfAKind,

            (Some((4, _)), _, 0) => HandType::FourOfAKind,
            (Some((3, _)), _, 1) => HandType::FourOfAKind,
            (Some((2, _)), _, 2) => HandType::FourOfAKind,
            (Some((1, _)), _, 3) => HandType::FourOfAKind,

            (Some((3, _)), Some((2, _)), 0) => HandType::FullHouse,
            (Some((2, _)), Some((2, _)), 1) => HandType::FullHouse,
            //(Some((3, _)), Some((1, _)), 1) => HandType::FullHouse,
            //(Some((2, _)), Some((1, _)), 2) => HandType::FullHouse,

            (Some((3, _)), _, 0) => HandType::ThreeOfAKind,
            (Some((2, _)), _, 1) => HandType::ThreeOfAKind,
            (Some((1, _)), _, 2) => HandType::ThreeOfAKind,

            (Some((2, _)), Some((2, _)), 0) => HandType::TwoPair,
            (Some((2, _)), _, 0) => HandType::OnePair,
            (Some((1, _)), _, 1) => HandType::OnePair,
            (_, _, 0) => HandType::HighCard,
            (a, b, c) => panic!("Unknown combination: {:?}, {:?}, {:?}", a, b, c)
        }
    }
}

fn parse_line(s: &str) -> (Hand, u64) {
    let (_, (hand, _, bet)) = tuple((parse_hand, space1, nom::character::complete::u64))(s).unwrap();
    (hand, bet)
}

fn parse_line_with_jokers(s: &str) -> (HandWithJoker, u64) {
    let (_, (hand, _, bet)) = tuple((parse_hand_with_joker, space1, nom::character::complete::u64))(s).unwrap();
    (hand, bet)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let mut players = contents.clone().lines().map(parse_line).collect_vec();
    players.sort_by(|a, b| a.0.cmp(&b.0));
    let total_winnings_part_1: u64 = players.iter().enumerate().map(|(rank, player)| (rank as u64 + 1) * player.1).sum();
    println!("Total winnings [part 1]: {}", total_winnings_part_1);

    let mut players = contents.clone().lines().map(parse_line_with_jokers).collect_vec();
    players.sort_by(|a, b| a.0.cmp(&b.0));
    let total_winnings_part_2: u64 = players.iter().enumerate().map(|(rank, player)| (rank as u64 + 1) * player.1).sum();
    println!("Total winnings [part 2]: {}", total_winnings_part_2);
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cmp_hands() {
        assert_eq!(Hand::from_str("KK677").unwrap().cmp(&Hand::from_str("KTJJT").unwrap()), Ordering::Greater);
        assert_eq!(Hand::from_str("T55J5").unwrap().cmp(&Hand::from_str("QQQJA").unwrap()), Ordering::Less);
        assert_eq!(Hand::from_str("KTJJT").unwrap().cmp(&Hand::from_str("T55J5").unwrap()), Ordering::Less);
        assert_eq!(Hand::from_str("KTJJT").unwrap().cmp(&Hand::from_str("KK677").unwrap()), Ordering::Less);
        assert_eq!(Hand::from_str("KTJJT").unwrap().cmp(&Hand::from_str("QQQJA").unwrap()), Ordering::Less);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand::from_str("AAAAA").unwrap().hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::from_str("AA8AA").unwrap().hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::from_str("23332").unwrap().hand_type(), HandType::FullHouse);
        assert_eq!(Hand::from_str("TTT98").unwrap().hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::from_str("23432").unwrap().hand_type(), HandType::TwoPair);
        assert_eq!(Hand::from_str("23456").unwrap().hand_type(), HandType::HighCard);
        assert_eq!(Hand::from_str("T55J5").unwrap().hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::from_str("QQQJA").unwrap().hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::from_str("KK677").unwrap().hand_type(), HandType::TwoPair);
        assert_eq!(Hand::from_str("KTJJT").unwrap().hand_type(), HandType::TwoPair);
    }

    #[test]
    fn test_hand_type_with_joker() {
        assert_eq!(HandWithJoker::from_str("32T3K").unwrap().hand_type(), HandType::OnePair);
        assert_eq!(HandWithJoker::from_str("KK677").unwrap().hand_type(), HandType::TwoPair);
        assert_eq!(HandWithJoker::from_str("T55J5").unwrap().hand_type(), HandType::FourOfAKind);
        assert_eq!(HandWithJoker::from_str("KTJJT").unwrap().hand_type(), HandType::FourOfAKind);
        assert_eq!(HandWithJoker::from_str("QQQJA").unwrap().hand_type(), HandType::FourOfAKind);
    }
}