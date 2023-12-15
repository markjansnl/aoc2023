use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, u8},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::tuple,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day15;
impl Day for Day15 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<(&'static str, char, Option<u8>)>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        false
    }

    #[inline]
    fn parse(input: &'static str, part: Part) -> Result<Self::Parsed> {
        Ok(match part {
            Part1 => Vec::new(),
            Part2 => Parser::input(input)?.1,
        })
    }

    #[inline]
    #[cfg(test)]
    fn part1(_parsed: &Self::Parsed) -> Result<Self::Output> {
        const ANSWER: usize = part1(1);
        Ok(ANSWER)
    }

    #[inline]
    #[cfg(not(test))]
    fn part1(_parsed: &Self::Parsed) -> Result<Self::Output> {
        const ANSWER: usize = part1(0);
        Ok(ANSWER)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut hashmap: HashMap<usize, Vec<(&str, u8)>> = HashMap::with_capacity(256);
        for (label, operator, focal_length) in parsed.iter() {
            match operator {
                '-' => {
                    hashmap.entry(hash(label.as_bytes())).and_modify(|boxx| {
                        boxx.retain(|(box_label, _)| box_label != label);
                    });
                }
                '=' => {
                    let focal_length = focal_length.unwrap();
                    hashmap
                        .entry(hash(label.as_bytes()))
                        .and_modify(|boxx| {
                            if let Some((_, box_focal_length)) =
                                boxx.iter_mut().find(|(box_label, _)| box_label == label)
                            {
                                *box_focal_length = focal_length;
                            } else {
                                boxx.push((*label, focal_length));
                            }
                        })
                        .or_insert(vec![(*label, focal_length)]);
                }
                _ => unreachable!(),
            }
        }

        Ok(hashmap
            .iter()
            .flat_map(|(box_number, boxx)| {
                boxx.iter()
                    .enumerate()
                    .map(|(slot_number, (_, focal_length))| {
                        (*box_number + 1) * (slot_number + 1) * *focal_length as usize
                    })
            })
            .sum())
    }
}

#[inline]
const fn part1(input_index: usize) -> usize {
    let input = <Day15 as Day>::INPUTS[input_index].as_bytes();
    hash(input)
}

#[inline]
const fn hash(value: &[u8]) -> usize {
    let mut i = 0;
    let mut current: u8 = 0;
    let mut sum = 0;
    while i < value.len() {
        match value[i] {
            b'\r' | b'\n' => {}
            b',' => {
                sum += current as usize;
                current = 0;
            }
            c => current = ((current as usize + c as usize) * 17) as u8,
        }
        i += 1;
    }
    sum + current as usize
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day15 as Day>::Parsed> {
        all_consuming(separated_list1(
            tag(","),
            tuple((alpha1, alt((char('-'), char('='))), opt(u8))),
        ))(s)
    }
}
