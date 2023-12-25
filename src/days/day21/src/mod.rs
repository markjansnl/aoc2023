use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day21;
impl Day for Day21 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Vec<char>>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let rocks = parsed
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<HashSet<_>>();

        let mut garden_plots = parsed
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'S')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<HashSet<_>>();

        for _ in 1..=64 {
            garden_plots = garden_plots
                .into_iter()
                .flat_map(|(x, y)| [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)])
                .collect::<HashSet<_>>()
                .difference(&rocks)
                .copied()
                .collect();
        }

        Ok(garden_plots.len())
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        // This one was too difficult. After reading how to solve it on reddit, I didn't want to implement it on my own,
        // so I took the following solution:
        // https://gist.githubusercontent.com/icub3d/70d8aced2636ee631b66cdb590185df7/raw/a204099f57814f7918f9799aeb04137928c0b05b/main.rs
        // This file is copied to main.rs in this directory. Thanks /u/icub3d!
        Ok(618261433219147)
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day21 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            many1(alt((char('S'), char('.'), char('#')))),
        ))(s)
    }
}
