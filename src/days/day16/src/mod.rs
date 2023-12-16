use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

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
            direction: Direction::Right,
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
    x: isize,
    y: isize,
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
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

impl Contraption {
    fn borders(&self) -> Vec<Location> {
        let size = self.0.len() as isize;
        let mut borders = Vec::with_capacity(4 * (size as usize - 1));
        
        for i in 1..size {
            borders.push(Location::new(i, 1, Direction::Down));
            borders.push(Location::new(i, size, Direction::Up));
            borders.push(Location::new(0, i, Direction::Right));
            borders.push(Location::new(size, i, Direction::Left));
        }

        borders
    }

    fn energized(&self, start: Location) -> <Day16 as Day>::Output {
        let mut beams = vec![start];
        let mut visited = HashSet::new();
        while let Some(mut beam) = beams.pop() {
            while let Some(c) = self.get(beam.x, beam.y) {
                if visited.contains(&beam) {
                    break;
                } else {
                    visited.insert(beam);
                }

                match c {
                    '-' => match beam.direction {
                        Direction::Up | Direction::Down => {
                            beams.push(Location::new(beam.x - 1, beam.y, Direction::Left));
                            beam = Location::new(beam.x + 1, beam.y, Direction::Right);
                        }
                        Direction::Left => {
                            beam.x -= 1;
                        }
                        Direction::Right => {
                            beam.x += 1;
                        }
                    },
                    '|' => match beam.direction {
                        Direction::Up => {
                            beam.y -= 1;
                        }
                        Direction::Down => {
                            beam.y += 1;
                        }
                        Direction::Left | Direction::Right => {
                            beams.push(Location::new(beam.x, beam.y - 1, Direction::Up));
                            beam = Location::new(beam.x, beam.y + 1, Direction::Down);
                        }
                    },
                    '\\' => {
                        beam = match beam.direction {
                            Direction::Up => Location::new(beam.x - 1, beam.y, Direction::Left),
                            Direction::Down => Location::new(beam.x + 1, beam.y, Direction::Right),
                            Direction::Left => Location::new(beam.x, beam.y - 1, Direction::Up),
                            Direction::Right => Location::new(beam.x, beam.y + 1, Direction::Down),
                        }
                    }
                    '/' => {
                        beam = match beam.direction {
                            Direction::Up => Location::new(beam.x + 1, beam.y, Direction::Right),
                            Direction::Down => Location::new(beam.x - 1, beam.y, Direction::Left),
                            Direction::Left => Location::new(beam.x, beam.y + 1, Direction::Down),
                            Direction::Right => Location::new(beam.x, beam.y - 1, Direction::Up),
                        }
                    }
                    '.' => match beam.direction {
                        Direction::Up => beam.y -= 1,
                        Direction::Down => beam.y += 1,
                        Direction::Left => beam.x -= 1,
                        Direction::Right => beam.x += 1,
                    },
                    _ => unreachable!(),
                }
            }
        }

        visited
            .into_iter()
            .map(|location| (location.x, location.y))
            .collect::<HashSet<_>>()
            .len()
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 1 || y < 1 {
            None
        } else {
            self.0
                .get(y as usize - 1)
                .map(|line| line.get(x as usize - 1).copied())
                .flatten()
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
