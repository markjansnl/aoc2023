use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, space1, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day07;
impl Day for Day07 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<([Card; 5], Bid)>;
    type Output = u64;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut hands = parsed
            .iter()
            .copied()
            .map(|(cards, bid)| Hand::part1_from(cards, bid))
            .collect::<Vec<_>>();
        hands.sort_by(|hand1, hand2| hand1.cmp(hand2, Part1));
        Day07::total_winnings(hands)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut hands = parsed
            .iter()
            .copied()
            .map(|(cards, bid)| Hand::part2_from(cards, bid))
            .collect::<Vec<_>>();
        hands.sort_by(|hand1, hand2| hand1.cmp(hand2, Part2));
        Day07::total_winnings(hands)
    }
}

impl Day07 {
    fn total_winnings(hands: Vec<Hand>) -> Result<<Day07 as Day>::Output> {
        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    pub fn as_u64_part2(self) -> u64 {
        match self {
            Card::Jack => 1,
            _ => self as u64,
        }
    }

    pub fn cmp_part2(&self, other: &Card) -> Ordering {
        self.as_u64_part2().cmp(&other.as_u64_part2())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    cards: [Card; 5],
    bid: Bid,
    hand_type: HandType,
}

pub type Bid = u64;

impl Hand {
    pub fn part1_from(cards: [Card; 5], bid: Bid) -> Hand {
        let mut counts = [0u8; 15];
        for card in cards {
            counts[card as usize] += 1;
        }

        Hand {
            cards,
            bid,
            hand_type: HandType::from_counts(counts, 0),
        }
    }

    pub fn part2_from(cards: [Card; 5], bid: Bid) -> Hand {
        let mut counts = [0u8; 15];
        let mut jokers = 0;
        for card in cards {
            if card == Card::Jack {
                jokers += 1;
            } else {
                counts[card as usize] += 1;
            }
        }

        Hand {
            cards,
            bid,
            hand_type: HandType::from_counts(counts, jokers),
        }
    }

    pub fn cmp(&self, other: &Hand, part: Part) -> Ordering {
        let cmp_type = self.hand_type.cmp(&other.hand_type);
        if cmp_type.is_eq() {
            self.cards
                .iter()
                .zip(other.cards)
                .find_map(|(card1, card2)| {
                    let cmp_card = match part {
                        Part1 => card1.cmp(&card2),
                        Part2 => card1.cmp_part2(&card2),
                    };
                    if cmp_card.is_eq() {
                        None
                    } else {
                        Some(cmp_card)
                    }
                })
                .unwrap_or(Ordering::Equal)
        } else {
            cmp_type
        }
    }
}

impl HandType {
    fn from_counts(mut counts: [u8; 15], jokers: u8) -> HandType {
        counts.sort();
        match counts[14] + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts[13] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts[13] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<Vec<([Card; 5], Bid)>> {
        all_consuming(separated_list1(line_ending, Parser::hand_bid))(s)
    }

    fn hand_bid(s: &'static str) -> IResult<([Card; 5], Bid)> {
        separated_pair(Parser::hand, space1, u64)(s)
    }

    fn hand(s: &'static str) -> IResult<[Card; 5]> {
        map(
            tuple((
                Parser::card,
                Parser::card,
                Parser::card,
                Parser::card,
                Parser::card,
            )),
            |(card1, card2, card3, card4, card5)| [card1, card2, card3, card4, card5],
        )(s)
    }

    fn card(s: &'static str) -> IResult<Card> {
        alt((
            map(char('2'), |_| Card::Two),
            map(char('3'), |_| Card::Three),
            map(char('4'), |_| Card::Four),
            map(char('5'), |_| Card::Five),
            map(char('6'), |_| Card::Six),
            map(char('7'), |_| Card::Seven),
            map(char('8'), |_| Card::Eight),
            map(char('9'), |_| Card::Nine),
            map(char('T'), |_| Card::Ten),
            map(char('J'), |_| Card::Jack),
            map(char('Q'), |_| Card::Queen),
            map(char('K'), |_| Card::King),
            map(char('A'), |_| Card::Ace),
        ))(s)
    }
}
