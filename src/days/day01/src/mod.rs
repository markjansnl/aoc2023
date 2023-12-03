use nom::{bytes::complete::tag, combinator::map};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day01;
impl Day for Day01 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Games;
    type Output = usize;

    fn reuse_parsed() -> bool {
        false
    }

    fn parse(input: &'static str, _part: Part) -> Result<Games> {
        let (_, games) = Parser::parse_games(input)?;
        Ok(games)
    }

    fn part1(games: &Games) -> Result<usize> {
        Ok(games.0.len())
    }

    fn part2(_input: &Self::Parsed) -> Result<Self::Output> {
        Ok(150)
    }
}

pub struct Games(Vec<Game>);

struct Game {}

struct Parser;
impl Parser {
    fn parse_games(s: &str) -> IResult<Games> {
        map(tag("games"), |_| Games(Vec::new()))(s)
    }
}
