use std::{
    collections::HashSet,
    fmt::{self, Debug},
    sync::OnceLock,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u64},
    combinator::{all_consuming, map},
    multi::{many1, many1_count, separated_list1},
    sequence::separated_pair,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day12;
impl Day for Day12 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<(Groups, Vec<u64>)>;
    type Output = u64;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, part: Part) -> Result<Self::Parsed> {
        Ok(match part {
            Part1 => Parser::input(input)?.1,
            Part2 => {
                let part2_input = input.lines().map(|line| {
                    let (groups, group_validations) = line.split_once(' ').unwrap();
                    format!("{0}?{0}?{0}?{0}?{0} {1},{1},{1},{1},{1}", groups, group_validations)
                }).collect::<Vec<_>>().join("\n");
                Parser::input(part2_input.as_str()).unwrap().1
            },
        })
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Self::solve(parsed)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Self::solve(parsed)
    }
}

impl Day12 {
    fn solve(parsed: &<Day12 as Day>::Parsed) -> Result<<Day12 as Day>::Output> {
        Ok(parsed
            .iter()
            .map(|(groups, group_validations)| {
                let mut arrangements = vec![(GroupValidation(vec![]), 1)];
                for group in groups {
                    let mut arrangements_group = Vec::new();
                    for permutation in group.permutations() {
                        let multiplier = permutation.arrangements(group.length);
                        for arrangement in arrangements.iter_mut() {
                            let mut arrangement_new = arrangement.clone();
                            arrangement_new.0 .0.append(&mut permutation.0.clone());
                            if group.condition == Condition::Unknown {
                                arrangement_new.1 *= multiplier
                            }
                            arrangements_group.push(arrangement_new);
                        }
                    }
                    arrangements = arrangements_group;
                }

                arrangements
                    .into_iter()
                    .map(|arrangement| {
                        let validation = arrangement
                            .0
                             .0
                            .iter()
                            .fold(Vec::new(), |mut parts, next| {
                                if let ValidationPart::Length(next_length) = next {
                                    if let Some(ValidationPart::Length(prev_length)) =
                                        parts.last_mut()
                                    {
                                        *prev_length += next_length;
                                    } else {
                                        parts.push(*next);
                                    }
                                } else {
                                    parts.push(*next);
                                }
                                parts
                            })
                            .into_iter()
                            .filter_map(|part| {
                                if let ValidationPart::Length(length) = part {
                                    Some(length)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        if &validation == group_validations {
                            arrangement.1
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            })
            .sum())
    }
}

static PERMUTATIONS: OnceLock<Vec<Vec<GroupValidation>>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Group {
    condition: Condition,
    length: u64,
}

pub type Groups = Vec<Group>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ValidationPart {
    Separator,
    Length(u64),
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct GroupValidation(Vec<ValidationPart>);

type GroupValidations = Vec<GroupValidation>;

impl ValidationPart {
    pub fn is_separator(&self) -> bool {
        *self == ValidationPart::Separator
    }

    pub fn is_length(&self) -> bool {
        *self != ValidationPart::Separator
    }
}

impl Group {
    fn permutations(&self) -> GroupValidations {
        match self.condition {
            Condition::Operational => vec![GroupValidation(vec![ValidationPart::Separator])],
            Condition::Damaged => vec![GroupValidation(vec![ValidationPart::Length(self.length)])],
            Condition::Unknown => permutations(self.length),
        }
    }
}

impl GroupValidation {
    fn first(&self) -> ValidationPart {
        *self.0.first().unwrap()
    }

    fn last(&self) -> ValidationPart {
        *self.0.last().unwrap()
    }

    fn separator_count(&self) -> u64 {
        self.0
            .iter()
            .filter(|part| part == &&ValidationPart::Separator)
            .count() as u64
    }

    fn length_count(&self) -> u64 {
        self.0.len() as u64 - self.separator_count()
    }

    fn prepend_separator(&self) -> Self {
        let mut next = self.clone();
        next.0.insert(0, ValidationPart::Separator);
        next
    }

    fn append_separator(&self) -> Self {
        let mut next = self.clone();
        next.0.push(ValidationPart::Separator);
        next
    }

    fn prepend_one(&self) -> Self {
        let mut next = self.clone();
        next.0.insert(0, ValidationPart::Length(1));
        next
    }

    fn append_one(&self) -> Self {
        let mut next = self.clone();
        next.0.push(ValidationPart::Length(1));
        next
    }

    fn arrangements(&self, length: u64) -> u64 {
        let separator_count = self.separator_count();
        let divide = length
            - self
                .0
                .iter()
                .map(|part| match part {
                    ValidationPart::Separator => 1,
                    ValidationPart::Length(length) => *length,
                })
                .sum::<u64>();
        // print!("{self:#?}, {length}, {divide} -> ");
        // let a =
        Self::arrangements_recursive(separator_count, divide)
        // println!("{a}");
        // a
    }

    fn arrangements_recursive(separator_count: u64, divide: u64) -> u64 {
        if separator_count <= 1 || divide == 0 {
            1
        } else {
            (0..=divide)
                .map(|i| Self::arrangements_recursive(separator_count - 1, i))
                .sum::<u64>()
        }
    }
}

impl Debug for GroupValidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for part in &self.0 {
            match part {
                ValidationPart::Separator => {
                    write!(f, ",")?;
                }
                ValidationPart::Length(length) => {
                    write!(f, "{length}")?;
                }
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

fn permutations(length: u64) -> Vec<GroupValidation> {
    let permutations = PERMUTATIONS.get_or_init(|| {
        let mut permutations = vec![vec![
            GroupValidation(vec![ValidationPart::Separator]),
            GroupValidation(vec![ValidationPart::Length(1)]),
        ]];

        for _ in 2..18 {
            let mut permutation = HashSet::new();
            for prev in permutations.last().unwrap() {
                if prev.first().is_separator() {
                    permutation.insert(prev.prepend_one());
                } else {
                    permutation.insert(prev.prepend_separator());
                }

                if prev.last().is_separator() {
                    permutation.insert(prev.append_one());
                } else {
                    permutation.insert(prev.append_separator());
                }

                if prev.separator_count() > 0 {
                    permutation.insert(prev.clone());
                }

                for i in 0..prev.length_count() {
                    let mut next = prev.clone();
                    for (j, part) in next
                        .0
                        .iter_mut()
                        .filter(|part| part.is_length())
                        .enumerate()
                    {
                        if i == j as u64 {
                            if let ValidationPart::Length(length) = part {
                                *length += 1;
                            }
                        }
                    }
                    permutation.insert(next);
                }
            }
            permutations.push(permutation.into_iter().collect());
        }

        permutations
    });

    permutations[length as usize - 1].clone()
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<<Day12 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            separated_pair(Parser::groups, tag(" "), Parser::group_validation),
        ))(s)
    }

    fn groups(s: &str) -> IResult<Groups> {
        many1(alt((
            map(many1_count(char('.')), |length| Group {
                condition: Condition::Operational,
                length: length as u64,
            }),
            map(many1_count(char('#')), |length| Group {
                condition: Condition::Damaged,
                length: length as u64,
            }),
            map(many1_count(char('?')), |length| Group {
                condition: Condition::Unknown,
                length: length as u64,
            }),
        )))(s)
    }

    fn group_validation(s: & str) -> IResult<Vec<u64>> {
        separated_list1(tag(","), u64)(s)
    }
}
