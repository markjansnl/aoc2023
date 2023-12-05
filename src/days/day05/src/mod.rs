use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Range},
};

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{i64, line_ending, multispace1, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

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
        parsed
            .seeds
            .iter()
            .map(|seed| {
                let mut source = "seed";
                let mut number = *seed;
                while let Some(map) = parsed.maps.get(source) {
                    source = map.destination;
                    number = map.mappings.map(number);
                }
                number
            })
            .min()
            .context("There should always be minimal one seed")
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut ranges = parsed
            .seeds
            .chunks(2)
            .map(|chunks| chunks[0]..chunks[0] + chunks[1])
            .collect::<Vec<Range<Self::Output>>>();

        let mut source = "seed";
        while let Some(map) = parsed.maps.get(source) {
            source = map.destination;
            ranges = ranges
                .iter()
                .flat_map(|range| map.mappings.map_range(range))
                .collect();
        }

        ranges
            .into_iter()
            .map(|range| range.start)
            .min()
            .context("There should always be minimal one range")
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

#[derive(Debug)]
pub struct Mappings<T: Debug>(Vec<(Range<T>, T)>);

impl<T: Debug> FromIterator<(Range<T>, T)> for Mappings<T> {
    fn from_iter<I: IntoIterator<Item = (Range<T>, T)>>(iter: I) -> Self {
        Mappings(Vec::from_iter(iter))
    }
}

impl<T: Debug + Default + Copy + Ord + Add<Output = T>> Mappings<T> {
    pub fn map(&self, source: T) -> T {
        self.0
            .iter()
            .find_map(|(range, offset)| {
                if range.contains(&source) {
                    return Some(source + *offset);
                }
                None
            })
            .unwrap_or(source)
    }

    pub fn map_range(&self, source: &Range<T>) -> Vec<Range<T>> {
        let mut iter = self
            .0
            .iter()
            .filter(|(range, _)| range.end > source.start && range.start < source.end)
            .cloned()
            .peekable();

        let mut ranges = Vec::new();
        let mut number = source.start;
        while number < source.end {
            if iter.peek().is_none() {
                ranges.push((number..source.end, Default::default()));
                number = source.end;
            } else if let Some((mut range, offset)) = iter.next() {
                if number < range.start {
                    ranges.push((number..range.start, Default::default()));
                }
                if range.contains(&source.start) {
                    range.start = source.start;
                }
                if range.contains(&source.end) {
                    range.end = source.end;
                }
                number = range.end;
                ranges.push((range, offset));
            } else {
                ranges.push((number..source.end, Default::default()));
                number = source.end;
            }
        }

        ranges
            .into_iter()
            .map(|(mut range, offset)| {
                range.start = range.start + offset;
                range.end = range.end + offset;
                range
            })
            .collect::<Vec<Range<T>>>()
    }

    pub fn sort(&mut self) {
        self.0.sort_by_key(|(range, _)| range.start);
    }
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
                let mut mappings = vec
                    .into_iter()
                    .map(|((destination_range_start, source_range_start), length)| {
                        (
                            source_range_start..source_range_start + length,
                            destination_range_start - source_range_start,
                        )
                    })
                    .collect::<Mappings<i64>>();
                mappings.sort();
                mappings
            },
        )(s)
    }
}
