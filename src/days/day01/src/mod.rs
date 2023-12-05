use nom::{
    bytes::complete::tag, character::complete::anychar, combinator::peek, multi::fold_many0,
    IResult,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day01;
impl Day for Day01 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = &'static str;
    type Output = u32;

    fn reuse_parsed() -> bool {
        false
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(input)
    }

    fn part1(input: &Self::Parsed) -> Result<Self::Output> {
        Day01::sum_calibration_values(input, false)
    }

    fn part2(input: &Self::Parsed) -> Result<Self::Output> {
        Day01::sum_calibration_values(input, true)
    }
}

impl Day01 {
    fn sum_calibration_values(s: &'static str, part2: bool) -> Result<u32> {
        Ok(s.lines()
            .map(|line| Ok::<_, anyhow::Error>(Parser::line(line, part2)?))
            .filter_map(Result::ok)
            .filter_map(|(_, result)| result)
            .sum())
    }
}

struct Parser;
impl Parser {
    fn line(s: &'static str, part2: bool) -> IResult<&str, Option<u32>> {
        fold_many0(
            |s| {
                if part2 {
                    Parser::spelled(s)
                } else {
                    Parser::opt_u32(s)
                }
            },
            || (None, 0),
            |(first, last), i| {
                if let Some(i) = i {
                    (Some(first.unwrap_or(i)), i)
                } else {
                    (first, last)
                }
            },
        )(s).map(|(s, (first, last))| {
            (s, first.map(|first| 10 * first + last))
        })
    }

    fn spelled(s: &str) -> IResult<&str, Option<u32>> {
        use nom::error::Error;
        peek(tag::<_, _, Error<_>>("one"))(s)
            .map(|(s, _)| (s, Some(1)))
            .or_else(|_| peek(tag::<_, _, Error<_>>("two"))(s).map(|(s, _)| (s, Some(2))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("three"))(s).map(|(s, _)| (s, Some(3))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("four"))(s).map(|(s, _)| (s, Some(4))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("five"))(s).map(|(s, _)| (s, Some(5))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("six"))(s).map(|(s, _)| (s, Some(6))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("seven"))(s).map(|(s, _)| (s, Some(7))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("eight"))(s).map(|(s, _)| (s, Some(8))))
            .or_else(|_| peek(tag::<_, _, Error<_>>("nine"))(s).map(|(s, _)| (s, Some(9))))
            .or(Ok((s, None)))
            .and_then(|(s, o)| anychar(s).map(|(s, c)| (s, o.or_else(|| c.to_digit(10)))))
    }

    fn opt_u32(s: &str) -> IResult<&str, Option<u32>> {
        anychar(s).map(|(s, c)| (s, c.to_digit(10)))
    }
}
