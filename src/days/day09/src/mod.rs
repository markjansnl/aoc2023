use nom::{
    character::complete::{i64, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day09;
impl Day for Day09 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Vec<Self::Output>>;
    type Output = i64;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .map(|history| {
                let mut history = history.clone();
                let mut predicted = 0;
                while history.iter().any(|i| i != &0i64) {
                    let mut next_history = Vec::with_capacity(history.len() - 1);
                    predicted += history
                        .into_iter()
                        .reduce(|prev, next| {
                            next_history.push(next - prev);
                            next
                        })
                        .unwrap_or_default();
                    history = next_history;
                }
                predicted
            })
            .sum())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .map(|history| {
                let mut history = history.clone();
                let mut firsts = vec![history[0]];
                while history.iter().any(|i| i != &0i64) {
                    let mut next_history = Vec::with_capacity(history.len() - 1);
                    history
                        .into_iter()
                        .reduce(|prev, next| {
                            next_history.push(next - prev);
                            next
                        })
                        .unwrap_or_default();
                    history = next_history;
                    firsts.push(history[0]);
                }
                firsts.into_iter().rev().fold(0, |predicted, first| first - predicted)

            })
            .sum())
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day09 as Day>::Parsed> {
        all_consuming(separated_list1(line_ending, Parser::history))(s)
    }

    fn history(s: &'static str) -> IResult<Vec<<Day09 as Day>::Output>> {
        separated_list1(space1, i64)(s)
    }
}
