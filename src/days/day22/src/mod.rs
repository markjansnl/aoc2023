use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day22;
impl Day for Day22 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Brick>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut unsettled = parsed.clone();
        unsettled.sort_by_key(|brick| brick.bottom());

        let mut settled: Vec<SettledBrick> = vec![SettledBrick {
            brick: Brick {
                start: Coordinate { x: 0, y: 0, z: 0 },
                end: Coordinate {
                    x: usize::MAX,
                    y: usize::MAX,
                    z: 0,
                },
            },
            offset: 0,
            supporting: Vec::new(),
        }];

        for brick in unsettled.into_iter() {
            let mut highest = 0;
            for settled_brick in settled.iter_mut() {
                if brick.above(&settled_brick.brick)
                    && settled_brick.brick.top() - settled_brick.offset > highest
                {
                    highest = settled_brick.brick.top() - settled_brick.offset;
                }
            }
            let mut next = None;
            for settled_brick in settled.iter_mut() {
                if brick.above(&settled_brick.brick)
                    && settled_brick.brick.top() - settled_brick.offset == highest
                {
                    next = Some(brick.settle(settled_brick));
                }
            }

            if let Some(settled_brick) = next {
                settled.push(settled_brick);
            }
        }

        Ok(settled
            .iter()
            .filter(|settled_brick| {
                settled_brick.supporting.iter().all(|settled_brick2| {
                    settled.iter().any(|settled_brick3| {
                        &settled_brick3 != settled_brick
                            && settled_brick3.supporting.contains(settled_brick2)
                    })
                })
            })
            .count())
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize, z: usize) -> Coordinate {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug, PartialEq, Eq)]
struct SettledBrick {
    brick: Brick,
    offset: usize,
    supporting: Vec<Brick>,
}

impl Brick {
    fn left(&self) -> usize {
        self.start.x
    }

    fn right(&self) -> usize {
        self.end.x
    }

    fn front(&self) -> usize {
        self.start.y
    }

    fn back(&self) -> usize {
        self.end.y
    }

    fn top(&self) -> usize {
        self.end.z
    }

    fn bottom(&self) -> usize {
        self.start.z
    }

    fn above(&self, other: &Brick) -> bool {
        self.bottom() > other.top()
            && (self.left() <= other.right() && other.left() <= self.right())
            && (self.front() <= other.back() && other.front() <= self.back())
    }

    fn settle(&self, on: &mut SettledBrick) -> SettledBrick {
        on.supporting.push(*self);
        SettledBrick {
            brick: *self,
            offset: self.bottom() - (on.brick.top() - on.offset + 1),
            supporting: Vec::new(),
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day22 as Day>::Parsed> {
        all_consuming(separated_list1(line_ending, Parser::brick))(s)
    }

    fn brick(s: &'static str) -> IResult<Brick> {
        map(
            separated_pair(Parser::coordinate, tag("~"), Parser::coordinate),
            |(start, end)| Brick { start, end },
        )(s)
    }

    fn coordinate(s: &'static str) -> IResult<Coordinate> {
        map(
            tuple((u64, preceded(tag(","), u64), preceded(tag(","), u64))),
            |(x, y, z)| Coordinate::new(x as usize, y as usize, z as usize),
        )(s)
    }
}
