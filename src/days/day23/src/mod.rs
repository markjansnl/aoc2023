use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

use Direction::*;

pub struct Day23;
impl Day for Day23 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Map;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        parsed
            .longest_path(Node {
                tile: Tile { x: 2, y: 1 },
                direction: Down,
            })
            .context("Longest path not found")
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

pub struct Map(Vec<Vec<char>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    tile: Tile,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Left => '<',
            Right => '>',
            Up => '^',
            Down => 'v',
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl Map {
    fn longest_path(&self, node: Node) -> Option<<Day23 as Day>::Output> {
        if node.tile == self.end() {
            return Some(0);
        }
        self.successors(node)
            .into_iter()
            .filter_map(|successor| self.longest_path(successor))
            .max()
            .map(|n| n + 1)
    }

    fn successors(
        &self,
        Node {
            tile: Tile { x, y },
            direction,
        }: Node,
    ) -> Vec<Node> {
        [
            Node {
                tile: Tile { x: x - 1, y },
                direction: Left,
            },
            Node {
                tile: Tile { x: x + 1, y },
                direction: Right,
            },
            Node {
                tile: Tile { x, y: y - 1 },
                direction: Up,
            },
            Node {
                tile: Tile { x, y: y + 1 },
                direction: Down,
            },
        ]
        .into_iter()
        .filter(
            |&Node {
                 tile:
                     Tile {
                         x: next_x,
                         y: next_y,
                     },
                 direction: next_direction,
             }| {
                let next_c = self.get(next_x, next_y);
                next_direction != direction.opposite()
                    && (next_c == '.' || next_c == next_direction.into())
            },
        )
        .collect()
    }

    fn end(&self) -> Tile {
        Tile {
            x: self.0[0].len() - 1,
            y: self.0.len(),
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        if x == 0 || y == 0 || x > self.0[0].len() || y > self.0.len() {
            '#'
        } else {
            self.0[y - 1][x - 1]
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day23 as Day>::Parsed> {
        map(
            all_consuming(separated_list1(line_ending, many1(Parser::tile))),
            Map,
        )(s)
    }

    fn tile(s: &'static str) -> IResult<char> {
        alt((
            char('.'),
            char('#'),
            char('^'),
            char('>'),
            char('v'),
            char('<'),
        ))(s)
    }
}

#[cfg(test)]
#[test]
fn test_heap() {
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push(-4isize);
    heap.push(-10isize);
    heap.push(-1isize);
    heap.push(-6isize);

    while let Some(x) = heap.pop() {
        println!("{x}");
    }
}
