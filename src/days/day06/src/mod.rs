use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day06;
impl Day for Day06 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Race>;
    type Output = u32;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.iter().map(Race::wins).product())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(todo!())
    }
}

#[derive(Debug)]
pub struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    pub fn wins(&self) -> u32 {
        let d = ((self.time.pow(2) - 4 * self.distance) as f32).sqrt();
        let first = Race::ceil((-1f32 * self.time as f32 + d) / -2f32);
        let last = Race::floor((-1f32 * self.time as f32 - d) / -2f32);
        last - first + 1
    }

    fn floor(i: f32) -> u32 {
        let f = i.floor() as u32;
        let c = i.ceil() as u32;
        if f == c {
            f - 1
        } else {
            f
        }
    }

    fn ceil(i: f32) -> u32 {
        let f = i.floor() as u32;
        let c = i.ceil() as u32;
        if f == c {
            c + 1
        } else {
            c
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<Vec<Race>> {
        map(
            all_consuming(separated_pair(
                Parser::line("Time:"),
                line_ending,
                Parser::line("Distance:"),
            )),
            |(times, distances)| {
                times
                    .into_iter()
                    .zip(distances.into_iter())
                    .map(|(time, distance)| Race { time, distance })
                    .collect()
            },
        )(s)
    }

    fn line(preceding: &'static str) -> impl Fn(&'static str) -> IResult<Vec<u32>> {
        move |s| {
            preceded(
                tag(preceding),
                preceded(space1, separated_list1(space1, u32)),
            )(s)
        }
    }
}
