#![allow(clippy::zero_prefixed_literal)]

use anyhow::Result;

pub mod prelude;

mod days;
mod def;
mod error;

use Part::*;

pub use days::{get_input, run_day};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}

impl From<u8> for Part {
    fn from(number: u8) -> Part {
        match number {
            1 => Part1,
            2 => Part2,
            _ => panic!("There is only part one and two"),
        }
    }
}

pub enum Reuse<T> {
    SingleUse(T),
    Reusable(T),
}

pub trait Day {
    const INPUTS: Self::Inputs;
    type Inputs;
    type Parsed;
    type Output: ToString;

    fn reuse_parsed() -> bool;
    fn parse(input: &'static str, part: Part) -> Result<Self::Parsed>;
    fn part1(parsed: &Self::Parsed) -> Result<Self::Output>;
    fn part2(parsed: &Self::Parsed) -> Result<Self::Output>;
}

fn run_day_generic<D: Day>(
    input: &'static str,
    part1: bool,
    part2: bool,
) -> Result<(Option<String>, Option<String>)> {
    let mut parsed_part1 = None;
    let mut output = (None, None);

    if part1 {
        let parsed = D::parse(input, Part1)?;
        output.0 = Some(D::part1(&parsed)?.to_string());
        parsed_part1 = Some(parsed);
    }

    if part2 {
        let parsed = if part1 && D::reuse_parsed() {
            parsed_part1.unwrap()
        } else {
            D::parse(input, Part2)?
        };
        output.1 = Some(D::part2(&parsed)?.to_string());
    }

    Ok(output)
}

pub fn run_day_part(day: u8, part: Part, input: &'static str) -> Result<String> {
    let (part1_output, part2_output) = run_day(day, input, part == Part1, part == Part2)?;
    Ok(match part {
        Part1 => part1_output.unwrap(),
        Part2 => part2_output.unwrap(),
    })
}

pub fn run_input(day: u8, part: Part, index: usize) -> Result<String> {
    run_day_part(day, part, get_input(day, index)?)
}

pub fn test_example(day: u8, part: Part, example: usize, expected: String) -> Result<()> {
    assert_eq!(expected, run_day_part(day, part, get_input(day, example)?)?);
    Ok(())
}

#[macro_export]
macro_rules! days {
    ($(Day $day:literal { $(example $example:literal { $(part $part:literal expected $expected:literal,)+ })+ })+) => {
        paste::paste! {
            $(
                mod [< day $day >];
            )+

            pub const DAYS: $crate::def::Days = &[
                $(
                    $crate::def::Day {
                        day: $day,
                        examples: &[
                            $(
                                $crate::def::Example {
                                    example: $example,
                                    parts:  &[
                                        $(
                                            $crate::def::Part {
                                                part: $crate :: Part :: [< Part $part >],
                                                expected: $expected,
                                            },
                                        )+
                                    ],
                                }
                            )+
                        ],
                    },
                )+
            ];

            pub fn run_day(
                day: u8,
                input: &'static str,
                part1: bool,
                part2: bool,
            ) -> anyhow::Result<(Option<String>, Option<String>)> {
                match day {
                    $(
                        $day => super::run_day_generic::< [< day $day >] :: [< Day $day >] >(input, part1, part2),
                    )+
                    _ => return Err(anyhow::anyhow!(format!("Day {day} is not implemented"))),
                }
            }

            pub fn _day_reuse_parts(day: u8) -> anyhow::Result<bool> {
                use $crate::Day;
                Ok(match day {
                    $(
                        $day => < [< day $day >] :: [< Day $day >] >::reuse_parsed(),
                    )+
                    _ => return Err(anyhow::anyhow!(format!("Day {day} is not implemented"))),
                })
            }

            pub fn get_input(
                day: u8,
                index: usize,
            ) -> anyhow::Result<&'static str> {
                use $crate::Day;
                Ok(match day {
                    $(
                        $day => < [< day $day >] :: [< Day $day >] >::INPUTS[index],
                    )+
                    _ => return Err(anyhow::anyhow!(format!("Day {day} is not implemented"))),
                })
            }

            #[cfg(test)]
            mod tests {
                $(
                    mod [< day $day >] {
                        $(
                            mod [< example $example >] {
                                $(
                                    #[test]
                                    fn [< part $part >] () -> anyhow::Result<()> {
                                        $crate::test_example($day, $part.into(), $example, $expected.to_string())
                                    }
                                )*
                            }
                        )+
                    }
                )+
            }
        }
    };
}
