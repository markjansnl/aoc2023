use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{i64, line_ending, multispace1, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};

use self::mappings::{MappingRange, Mappings};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

mod mappings;

pub struct Day05;
impl Day for Day05 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Almanac;
    type Output = i64;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .seeds
            .iter()
            .map(|seed| {
                let mut source = "seed";
                let mut number = *seed;
                while let Some(map) = parsed.maps.get(source) {
                    source = map.destination;
                    number += map.mappings.get(&MappingRange::Get(number)).unwrap_or(&0);
                }
                number
            })
            .min()
            .context("There should always be minimal one seed")?)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .seeds
            .par_windows(2)
            .map(|range| {
                let start = range[0];
                let end = range[0] + range[1];

                (start..end)
                    .into_par_iter()
                    .map(|seed| {
                        let mut source = "seed";
                        let mut number = seed;
                        while let Some(map) = parsed.maps.get(source) {
                            source = map.destination;
                            number += map.mappings.get(&MappingRange::Get(number)).unwrap_or(&0);
                        }
                        number
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .min()
            .context("There should always be minimal one seed")?)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<i64>,
    maps: HashMap<&'static str, Map>,
}

#[derive(Debug)]
pub struct Map {
    destination: &'static str,
    mappings: Mappings<i64>,
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<Almanac> {
        map(
            all_consuming(pair(Parser::seeds, Parser::maps)),
            |(seeds, maps)| Almanac { seeds, maps },
        )(s)
    }

    fn seeds(s: &str) -> IResult<Vec<i64>> {
        preceded(tag("seeds: "), separated_list1(space1, i64))(s)
    }

    fn maps(s: &'static str) -> IResult<HashMap<&'static str, Map>> {
        map(
            preceded(multispace1, separated_list1(multispace1, Parser::map)),
            |map| {
                map.into_iter()
                    .map(|((source, destination), mappings)| {
                        (
                            source,
                            Map {
                                destination,
                                mappings,
                            },
                        )
                    })
                    .collect()
            },
        )(s)
    }

    fn map(s: &'static str) -> IResult<((&'static str, &'static str), Mappings<i64>)> {
        separated_pair(
            Parser::source_destination,
            preceded(tag(" map:"), line_ending),
            Parser::mappings,
        )(s)
    }

    fn source_destination(s: &'static str) -> IResult<(&'static str, &'static str)> {
        separated_pair(is_not("-"), tag("-to-"), is_not(" "))(s)
    }

    fn mappings(s: &str) -> IResult<Mappings<i64>> {
        map(
            separated_list1(
                line_ending,
                separated_pair(separated_pair(i64, space1, i64), space1, i64),
            ),
            |vec| {
                vec.into_iter()
                    .map(|((destination_range_start, source_range_start), length)| {
                        (
                            MappingRange::Key(source_range_start..source_range_start + length),
                            destination_range_start - source_range_start,
                        )
                    })
                    .collect()
            },
        )(s)
    }
}
