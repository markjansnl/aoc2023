use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

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
        Self::solve(parsed)
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        // Not computable in reasonable time (and killed by OS after almost 6 hours or running; approx. 10% was finished)
        // For now: return 0, so that tests and benches will not be affected
        Ok(0)
        
        // let input_part2 = parsed
        //     .iter()
        //     .map(|(springs, validation)| {
        //         (
        //             repeat(springs)
        //                 .take(5)
        //                 .map(|springs| springs.clone())
        //                 .collect::<Vec<String>>()
        //                 .join("?"),
        //             repeat(validation)
        //                 .take(5)
        //                 .flatten()
        //                 .cloned()
        //                 .collect::<Vec<_>>(),
        //         )
        //     })
        //     .collect::<Vec<_>>();
        // Self::solve(&input_part2)
    }
}

impl Day12 {
    fn solve(parsed: &<Day12 as Day>::Parsed) -> Result<<Day12 as Day>::Output> {
        Ok(parsed
            .par_iter()
            .map(|(springs, validation)| {
                let mut dp_masks: HashMap<(String, &[usize]), Vec<String>> = HashMap::new();
                // println!("{springs} {validation:?} START");
                // let count = 
                Self::masks(springs, validation, &mut dp_masks).len()
                // println!("{springs} {validation:?} {count}");
                // count
            })
            .sum())
    }

    fn masks<'a>(
        springs: &String,
        validation: &'a [usize],
        dp_masks: &mut HashMap<(String, &'a [usize]), Vec<String>>,
    ) -> Vec<String> {
        if let Some(masks_vec) = dp_masks.get(&(springs.clone(), validation)) {
            return masks_vec.clone();
        }
        let mut masks_vec = Vec::new();
        let shifts =
            springs.len() - validation.iter().sum::<usize>() + if validation.len() == 1 { 1 } else { 0 };
        for n in 0..shifts {
            let operational = ".".repeat(n);
            let damaged = "#".repeat(validation[0]);
            if validation.len() == 1 {
                let tail = ".".repeat(springs.len() - n - validation[0]);
                let mask = format!("{}{}{}", operational, damaged, tail);
                if Self::validate(springs.as_ref(), &mask) {
                    masks_vec.push(mask);
                }
            } else {
                for tail in Self::masks(&springs[(validation[0] + n + 1)..].to_string(), &validation[1..], dp_masks)
                {
                    let mask = format!("{}{}.{}", operational, damaged, tail);
                    if Self::validate(springs.as_ref(), &mask) {
                        masks_vec.push(mask);
                    }
                }
            }
        }
        dp_masks.insert((springs.clone(), validation), masks_vec.clone());
        masks_vec
    }

    fn validate(springs: &str, mask: &str) -> bool {
        springs
            .chars()
            .zip(mask.chars())
            .all(|(s, m)| s == m || s == '?')
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day12 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            separated_pair(Parser::springs, tag(" "), Parser::validation),
        ))(s)
    }

    fn springs(s: &str) -> IResult<String> {
        map(is_not(" "), String::from)(s)
    }

    fn validation(s: &str) -> IResult<Vec<usize>> {
        separated_list1(tag(","), map(u64, |i| i as usize))(s)
    }
}
