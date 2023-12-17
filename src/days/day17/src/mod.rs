use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use pathfinding::prelude::*;

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

use Direction::*;

pub struct Day17;
impl Day for Day17 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = HeatMap;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        parsed.least_heat_loss(1, 3)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        parsed.least_heat_loss(4, 10)
    }
}

#[derive(Debug)]
pub struct HeatMap(Vec<Vec<usize>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    direction: Option<Direction>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait Opposite {
    fn opposite(&self) -> Self;
}
impl Opposite for Option<Direction> {
    fn opposite(&self) -> Self {
        match *self {
            None => None,
            Some(Up) => Some(Down),
            Some(Down) => Some(Up),
            Some(Left) => Some(Right),
            Some(Right) => Some(Left),
        }
    }
}

impl HeatMap {
    fn least_heat_loss(&self, min: usize, max: usize) -> Result<<Day17 as Day>::Output> {
        Ok(astar(
            &Position {
                x: 1,
                y: 1,
                direction: None,
            },
            |position| self.successors(position, min, max),
            |position| self.heuristic(position),
            |position| self.success(position),
        )
        .context("No path found")?
        .1)
    }

    fn successors(&self, position: &Position, min: usize, max: usize) -> Vec<(Position, usize)> {
        [Up, Down, Left, Right]
            .into_iter()
            .filter(|direction| {
                Some(*direction) != position.direction
                    && Some(*direction) != position.direction.opposite()
            })
            .flat_map(|direction| {
                let mut position = *position;
                position.direction = Some(direction);
                let (dx, dy) = match direction {
                    Up => (0, -1),
                    Down => (0, 1),
                    Left => (-1, 0),
                    Right => (1, 0),
                };
                self.get_range(position, dx, dy, min, max)
            })
            .collect()
    }

    fn heuristic(&self, position: &Position) -> usize {
        position.x.abs_diff(self.0[0].len()) + position.y.abs_diff(self.0.len())
    }

    fn success(&self, position: &Position) -> bool {
        position.x == self.0[0].len() && position.y == self.0.len()
    }

    fn get_range(
        &self,
        mut position: Position,
        dx: isize,
        dy: isize,
        min: usize,
        max: usize,
    ) -> Vec<(Position, usize)> {
        let mut range = Vec::new();
        let mut heat_loss = 0;
        for i in 1..=max {
            position.x = (position.x as isize + dx) as usize;
            position.y = (position.y as isize + dy) as usize;
            if let Some(next) = self.get(position.x, position.y) {
                heat_loss += next;
                if i >= min {
                    range.push((position, heat_loss));
                }
            } else {
                return range;
            }
        }
        range
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        if x == 0 || y == 0 {
            None
        } else {
            self.0.get(y - 1).and_then(|line| line.get(x - 1).copied())
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day17 as Day>::Parsed> {
        map(
            all_consuming(separated_list1(
                line_ending,
                many1(map(one_of("123456789"), |c| {
                    c.to_digit(10).unwrap() as usize
                })),
            )),
            HeatMap,
        )(s)
    }
}
