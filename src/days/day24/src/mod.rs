use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending, space0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day24;
impl Day for Day24 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<(Coordinates, Coordinates)>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        #[cfg(test)]
        const MIN: f64 = 7f64;
        #[cfg(test)]
        const MAX: f64 = 27f64;

        #[cfg(not(test))]
        const MIN: f64 = 200000000000000f64;
        #[cfg(not(test))]
        const MAX: f64 = 400000000000000f64;

        Ok(parsed
            .iter()
            .copied()
            .enumerate()
            .flat_map(
                |(i, (Coordinates { x: x1, y: y1, .. }, Coordinates { x: vx1, y: vy1, .. }))| {
                    parsed.iter().skip(i + 1).copied().filter(
                        move |&(
                            Coordinates { x: x2, y: y2, .. },
                            Coordinates { x: vx2, y: vy2, .. },
                        )| {
                            let x = (((vy1 / vx1) * -x1 + y1) - ((vy2 / vx2) * -x2 + y2))
                                / ((vy2 / vx2) - (vy1 / vx1));
                            let y = (vy1 / vx1) * (x - x1) + y1;

                            let future1 = if vx1 >= 0.0 { x >= x1 } else { x < x1 };
                            let future2 = if vx2 >= 0.0 { x >= x2 } else { x < x2 };
                            let inside = (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y);

                            // println!("A: {x1}, {y1} @ {vx1}, {vy1}");
                            // println!("B: {x2}, {y2} @ {vx2}, {vy2}");
                            // println!("Crosses at {x}, {y}");
                            // if !future1 {
                            //     println!("A crosses in the past");
                            // }
                            // if !future2 {
                            //     println!("B crosses in the past");
                            // }
                            // if inside {
                            //     println!("INSIDE the box");
                            // } else {
                            //     println!("NOT inside the box");
                            // }
                            // println!();

                            future1 && future2 && inside
                        },
                    )
                },
            )
            .count())
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Coordinates {
    x: f64,
    y: f64,
    _z: f64,
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day24 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            separated_pair(
                Parser::coordinates,
                preceded(space0, tag("@")),
                Parser::coordinates,
            ),
        ))(s)
    }

    fn coordinates(s: &'static str) -> IResult<Coordinates> {
        map(
            tuple((
                preceded(space0, i64),
                preceded(tag(","), preceded(space0, i64)),
                preceded(tag(","), preceded(space0, i64)),
            )),
            |(x, y, z)| Coordinates {
                x: x as f64,
                y: y as f64,
                _z: z as f64,
            },
        )(s)
    }
}
