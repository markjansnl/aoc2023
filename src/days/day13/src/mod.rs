use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map},
    multi::{many1, many_m_n, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day13;
impl Day for Day13 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<HashedPattern>;
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
            .map(|hashed_pattern| hashed_pattern.find_mirror(Part1))
            .sum())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .map(|hashed_pattern| hashed_pattern.find_mirror(Part2))
            .sum())
    }
}

pub type Pattern = Vec<Vec<char>>;

pub struct HashedPattern {
    rows: Vec<(String, u64)>,
    columns: Vec<(String, u64)>,
}

impl HashedPattern {
    pub fn hash(pattern: Pattern) -> Self {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        for (y, line) in pattern.iter().enumerate() {
            rows.push((String::new(), DefaultHasher::new()));
            for (x, c) in line.iter().enumerate() {
                if y == 0 {
                    columns.push((String::new(), DefaultHasher::new()));
                }
                rows[y].0.push(*c);
                columns[x].0.push(*c);
                c.hash(&mut rows[y].1);
                c.hash(&mut columns[x].1);
            }
        }

        Self {
            rows: rows
                .into_iter()
                .map(|(s, hasher)| (s, hasher.finish()))
                .collect(),
            columns: columns
                .into_iter()
                .map(|(s, hasher)| (s, hasher.finish()))
                .collect(),
        }
    }

    pub fn find_mirror(&self, part: Part) -> usize {
        Self::find_mirror_line(&self.columns, part).unwrap_or_else(|| {
            Self::find_mirror_line(&self.rows, part)
                .map(|i| i * 100)
                .unwrap_or_default()
        })
    }

    fn find_mirror_line(hashes: &Vec<(String, u64)>, part: Part) -> Option<usize> {
        for i in 1..hashes.len() {
            let mut error_found = false;
            if hashes[0..i].iter().rev().zip(hashes[i..].iter()).all(
                |((left_string, left_hash), (right_string, right_hash))| {
                    if left_hash == right_hash {
                        true
                    } else if part == Part1 || error_found {
                        false
                    } else {
                        let diff = left_string
                            .chars()
                            .zip(right_string.chars())
                            .filter(|(left_char, right_char)| left_char != right_char)
                            .count();
                        if diff > 1 {
                            false
                        } else {
                            error_found = true;
                            true
                        }
                    }
                },
            ) && (part == Part1 || error_found)
            {
                return Some(i);
            }
        }
        None
    }
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<<Day13 as Day>::Parsed> {
        all_consuming(separated_list1(
            many_m_n(2, 2, line_ending),
            Parser::pattern,
        ))(s)
    }

    fn pattern(s: &str) -> IResult<HashedPattern> {
        map(
            separated_list1(line_ending, Parser::line),
            HashedPattern::hash,
        )(s)
    }

    fn line(s: &str) -> IResult<Vec<char>> {
        many1(alt((char('.'), char('#'))))(s)
    }
}
