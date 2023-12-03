use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct DayXX;
impl Day for DayXX {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = todo!();
    type Output = todo!();

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(todo!())
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
    fn parse_todo(s: &str) -> IResult<todo()> {
        todo!()
    }
}
