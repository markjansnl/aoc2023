use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1, u32, u8},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day04;
impl Day for Day04 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Card>;
    type Output = u32;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.iter().map(|card| card.points()).sum())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut count = parsed.iter().map(|_| 1).collect::<Vec<_>>();
        for (i, card) in parsed.iter().enumerate() {
            // i is 1-based and card.number is 1-based, so is already the next card. And it is the right type: u32
            for number in card.number..card.number + card.wins() {
                count[number as usize] += count[i];
            }
        }
        Ok(count.into_iter().sum())
    }
}

#[derive(Debug)]
pub struct Card {
    number: u32,
    winning: HashSet<u8>,
    mine: HashSet<u8>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let wins = self.wins();
        if wins == 0 {
            0
        } else {
            2u32.pow(wins - 1)
        }
    }

    fn wins(&self) -> u32 {
        self.winning.intersection(&self.mine).count() as u32
    }
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<Vec<Card>> {
        all_consuming(separated_list1(line_ending, Parser::line))(s)
    }

    fn line(s: &str) -> IResult<Card> {
        map(
            separated_pair(
                preceded(tag("Card"), preceded(space1, u32)),
                tag(": "),
                separated_pair(Parser::numbers, tag(" | "), Parser::numbers),
            ),
            |(number, (winning, mine))| Card {
                number,
                winning: winning.into_iter().collect(),
                mine: mine.into_iter().collect(),
            },
        )(s)
    }

    fn numbers(s: &str) -> IResult<Vec<u8>> {
        preceded(space0, separated_list1(space1, u8))(s)
    }
}
