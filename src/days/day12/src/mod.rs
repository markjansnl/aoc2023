use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

// After 2 complete rewrites part 2 crashed after 5:45h at 8,5%
// Found the following tutorial and ported it to rust. Thanks /u/StaticMoose!
// https://www.reddit.com/r/adventofcode/comments/18hbbxe/2023_day_12python_stepbystep_tutorial_with_bonus/?utm_source=share&utm_medium=web2x&context=3

pub struct Day12;
impl Day for Day12 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<(String, Vec<usize>)>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .map(|(springs, groups)| solve(springs.clone(), groups.clone()))
            .sum())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .map(|(springs, groups)| {
                solve(format!("{0}?{0}?{0}?{0}?{0}", springs), groups.repeat(5))
            })
            .sum())
    }
}

#[memoize::memoize]
fn solve(springs: String, groups: Vec<usize>) -> <Day12 as Day>::Output {
    if groups.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    } else if springs.is_empty() {
        return 0;
    }

    let next_spring = springs.chars().next().unwrap();
    let next_group = groups[0];

    let pound = || {
        if next_group > springs.len()
            || springs[0..next_group].to_string().replace('?', "#") != "#".repeat(next_group)
        {
            0
        } else if springs.len() == next_group {
            if groups.len() == 1 {
                1
            } else {
                0
            }
        } else if [".", "?"].contains(&&springs[next_group..next_group + 1]) {
            solve(
                springs[next_group + 1..].to_string(),
                groups.iter().skip(1).copied().collect(),
            )
        } else {
            0
        }
    };

    let dot = || solve(springs[1..].to_string(), groups.clone());

    match next_spring {
        '#' => pound(),
        '.' => dot(),
        '?' => pound() + dot(),
        _ => unreachable!(),
    }
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<<Day12 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            separated_pair(Parser::springs, tag(" "), Parser::groups),
        ))(s)
    }

    fn springs(s: &str) -> IResult<String> {
        map(is_not(" "), String::from)(s)
    }

    fn groups(s: &str) -> IResult<Vec<usize>> {
        separated_list1(tag(","), map(u64, |i| i as usize))(s)
    }
}
