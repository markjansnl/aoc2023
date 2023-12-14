use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day14;
impl Day for Day14 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Vec<char>>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.transpose().into_iter().map(|column| {
            let mut last_rock = column.len() + 1;
            let column_score = column.iter().enumerate().map(|(y, c)| {
                match c {
                    '#' => { last_rock = column.len() - y; 0 },
                    'O' => { last_rock -= 1; last_rock },
                    '.' => { 0 },
                    _ => unreachable!()
                }                
            }).sum::<Self::Output>();
            println!("{column_score}");
            column_score
        }).sum())
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

trait Transpose {
    fn transpose(&self) -> <Day14 as Day>::Parsed;
}
impl Transpose for <Day14 as Day>::Parsed {
    fn transpose(&self) -> <Day14 as Day>::Parsed {
        let mut columns = Vec::new();
        for (y, line) in self.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if y == 0 {
                    columns.push(Vec::new());
                }
                columns[x].push(*c);
            }
        }
        columns
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day14 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            many1(alt((char('#'), char('O'), char('.')))),
        ))(s)
    }
}
