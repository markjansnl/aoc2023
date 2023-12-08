use nom::{
    character::complete::{line_ending, u64},
    combinator::all_consuming,
    multi::separated_list1,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct DayXX;
impl Day for DayXX {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<u64>;
    type Output = u64;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(todo!())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(todo!())
    }
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<Vec<u64>> {
        all_consuming(separated_list1(line_ending, u64))(s)
    }
}
