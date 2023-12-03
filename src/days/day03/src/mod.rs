use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, none_of, u32},
    combinator::map,
    multi::{fold_many1, many1, separated_list1},
    number,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day03;
impl Day for Day03 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<SchematicEnginePart>;
    type Output = u32;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::parse_input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .filter_map(SchematicEnginePart::number)
            .filter(|number| {
                parsed
                    .iter()
                    .filter_map(SchematicEnginePart::symbol)
                    .any(|symbol| number.is_adjecent(symbol))
            })
            .map(|number| number.number)
            .sum())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .iter()
            .filter_map(SchematicEnginePart::symbol)
            .filter(|symbol| symbol.symbol == '*')
            .filter_map(|symbol| {
                let adjecent: Vec<_> = parsed
                    .iter()
                    .filter_map(SchematicEnginePart::number)
                    .filter(|number| number.is_adjecent(symbol))
                    .map(|number| number.number)
                    .collect();
                (adjecent.len() == 2).then(|| adjecent.into_iter().product::<Self::Output>())
            })
            .sum())
    }
}

#[derive(Debug)]
pub struct Number {
    number: u32,
    x: RangeInclusive<usize>,
    y: usize,
}

#[derive(Debug)]
pub struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum SchematicEnginePart {
    Number(Number),
    Symbol(Symbol),
    _Dot,
}

impl SchematicEnginePart {
    pub fn number(&self) -> Option<&Number> {
        if let SchematicEnginePart::Number(number) = self {
            Some(number)
        } else {
            None
        }
    }

    pub fn symbol(&self) -> Option<&Symbol> {
        if let SchematicEnginePart::Symbol(symbol) = self {
            Some(symbol)
        } else {
            None
        }
    }
}

impl Number {
    pub fn is_adjecent(&self, symbol: &Symbol) -> bool {
        (symbol.y - 1..=symbol.y + 1).contains(&self.y)
            && (symbol.x - 1..=symbol.x + 1).any(|x| self.x.contains(&x))
    }
}

struct Parser;
impl Parser {
    fn parse_input(s: &str) -> IResult<Vec<SchematicEnginePart>> {
        fold_many1(
            separated_list1(line_ending, Parser::parse_line),
            Vec::new,
            |mut parts, line_parts| {
                for (y, line_parts) in line_parts.into_iter().enumerate() {
                    for mut part in line_parts {
                        match &mut part {
                            SchematicEnginePart::Number(ref mut number) => number.y = y + 1,
                            SchematicEnginePart::Symbol(ref mut symbol) => symbol.y = y + 1,
                            _ => {}
                        }
                        parts.push(part);
                    }
                }
                parts
            },
        )(s)
    }

    fn parse_line(s: &str) -> IResult<Vec<SchematicEnginePart>> {
        map(many1(Parser::parse_schemetic_engine_part), |mut vec| {
            let mut x = 0;
            for part in vec.iter_mut() {
                x += match part {
                    SchematicEnginePart::Number(number) => {
                        let len = format!("{}", number.number).len();
                        number.x = x + 1..=x + len;
                        len
                    }
                    SchematicEnginePart::Symbol(symbol) => {
                        symbol.x = x + 1;
                        1
                    }
                    SchematicEnginePart::_Dot => 1,
                }
            }
            vec
        })(s)
    }

    fn parse_schemetic_engine_part(s: &str) -> IResult<SchematicEnginePart> {
        alt((Parser::parse_dot, Parser::parse_number, Parser::parse_symbol))(s)
    }

    fn parse_dot(s: &str) -> IResult<SchematicEnginePart> {
        map(tag("."), |_| SchematicEnginePart::_Dot)(s)
    }

    fn parse_number(s: &str) -> IResult<SchematicEnginePart> {
        map(u32, |number| {
            SchematicEnginePart::Number(Number {
                number,
                x: 0..=0,
                y: 0,
            })
        })(s)
    }

    fn parse_symbol(s: &str) -> IResult<SchematicEnginePart> {
        map(none_of("\r\n"), |symbol| {
            SchematicEnginePart::Symbol(Symbol { symbol, x: 0, y: 0 })
        })(s)
    }
}

#[test]
fn part1() -> Result<()> {
    let part1 = crate::run_day_part(3, Part2, INPUTS[0])?;
    println!("{part1}");
    Ok(())
}
