use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

use Direction::*;

pub struct Day16;
impl Day for Day16 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Contraption;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed.energized(Location {
            x: 1,
            y: 1,
            direction: Right,
        }))
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .borders()
            .into_iter()
            .map(|start| parsed.energized(start))
            .max()
            .unwrap())
    }
}

pub struct Contraption(Vec<Vec<char>>);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Location {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn go(mut self) -> Self {
        match self.direction {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
        self
    }

    fn turn(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

impl Contraption {
    fn borders(&self) -> Vec<Location> {
        let size = self.0.len();
        let mut borders = Vec::with_capacity(4 * (size - 1));

        for i in 1..size {
            borders.push(Location::new(i, 1, Down));
            borders.push(Location::new(i, size, Up));
            borders.push(Location::new(1, i, Right));
            borders.push(Location::new(size, i, Left));
        }

        borders
    }

    fn energized(&self, start: Location) -> <Day16 as Day>::Output {
        let mut locations = vec![start];
        let mut visited = HashSet::new();
        while let Some(mut location) = locations.pop() {
            while let Some(c) = self.get(location.x, location.y) {
                if visited.contains(&location) {
                    break;
                } else {
                    visited.insert(location);
                }

                let next = match c {
                    '-' => match location.direction {
                        Up | Down => {
                            locations.push(location.turn(Left).go());
                            location.turn(Right)
                        }
                        Left | Right => location,
                    },
                    '|' => match location.direction {
                        Up | Down => location,
                        Left | Right => {
                            locations.push(location.turn(Up).go());
                            location.turn(Down)
                        }
                    },
                    '\\' => location.turn(match location.direction {
                        Up => Left,
                        Down => Right,
                        Left => Up,
                        Right => Down,
                    }),
                    '/' => location.turn(match location.direction {
                        Up => Right,
                        Down => Left,
                        Left => Down,
                        Right => Up,
                    }),
                    '.' => location,
                    _ => unreachable!(),
                };

                location = next.go();
            }
        }

        visited
            .into_iter()
            .map(|location| (location.x, location.y))
            .collect::<HashSet<_>>()
            .len()
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x == 0 || y == 0 {
            None
        } else {
            self.0.get(y - 1).and_then(|line| line.get(x - 1).copied())
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day16 as Day>::Parsed> {
        map(
            all_consuming(separated_list1(
                line_ending,
                many1(alt((
                    char('.'),
                    char('-'),
                    char('|'),
                    char('\\'),
                    char('/'),
                ))),
            )),
            Contraption,
        )(s)
    }
}
