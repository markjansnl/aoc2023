use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct DayXX;
impl Day for DayXX {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = usize;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<DayXX as Day>::Parsed> {
        todo!()
    }
}
