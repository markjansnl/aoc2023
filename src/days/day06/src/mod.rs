use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, space1, u64},
    combinator::{all_consuming, map, map_res},
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
    type Output = u64;

    fn reuse_parsed() -> bool {
        false
    }

    fn parse(input: &'static str, part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input, part)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.iter().map(Race::wins).product())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.iter().map(Race::wins).product())
    }
}

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn wins(&self) -> u64 {
        let d = ((self.time.pow(2) - 4 * self.distance) as f64).sqrt();
        let first = Race::ceil((-1f64 * self.time as f64 + d) / -2f64);
        let last = Race::floor((-1f64 * self.time as f64 - d) / -2f64);
        last - first + 1
    }

    fn floor(i: f64) -> u64 {
        let f = i.floor() as u64;
        let c = i.ceil() as u64;
        if f == c {
            f - 1
        } else {
            f
        }
    }

    fn ceil(i: f64) -> u64 {
        let f = i.floor() as u64;
        let c = i.ceil() as u64;
        if f == c {
            c + 1
        } else {
            c
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str, part: Part) -> IResult<Vec<Race>> {
        map(
            all_consuming(separated_pair(
                Parser::line("Time:", part),
                line_ending,
                Parser::line("Distance:", part),
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

    fn line(preceding: &'static str, part: Part) -> impl Fn(&'static str) -> IResult<Vec<u64>> {
        move |s| {
            preceded(
                tag(preceding),
                preceded(
                    space1,
                    match part {
                        Part1 => Parser::part1,
                        Part2 => Parser::part2,
                    },
                ),
            )(s)
        }
    }

    fn part1(s: &'static str) -> IResult<Vec<u64>> {
        separated_list1(space1, u64)(s)
    }

    fn part2(s: &'static str) -> IResult<Vec<u64>> {
        map_res(is_not("\r\n"), |line: &'static str| {
            Ok::<_, anyhow::Error>(vec![line.replace(" ", "").parse::<u64>()?])
        })(s)
    }
}
