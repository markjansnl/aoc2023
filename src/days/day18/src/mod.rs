use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char, i64, line_ending, not_line_ending},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

use Direction::*;

pub struct Day18;
impl Day for Day18 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<PlanItem>;
    type Output = isize;

    fn reuse_parsed() -> bool {
        false
    }

    fn parse(input: &'static str, part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input, part)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Self::cubic_meters(parsed)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Self::cubic_meters(parsed)
    }
}

impl Day18 {
    fn cubic_meters(parsed: &<Self as Day>::Parsed) -> Result<<Self as Day>::Output> {
        let (horizontal_trench_map, vertical_trench_map) = Self::trench_maps(parsed);
        let horizontal_corners = vertical_trench_map.keys();
        let vertical_corners = horizontal_trench_map.keys();

        let mut areas = HashMap::new();
        let mut horizontal_edges = HashMap::new();
        let mut vertical_edges = HashMap::new();

        for (y, v) in vertical_corners.windows(2).enumerate() {
            for (x, h) in horizontal_corners.windows(2).enumerate() {
                let position = Position {
                    x: x as isize + 1,
                    y: y as isize + 1,
                };
                areas.insert(
                    position,
                    Area {
                        width: h[1] - h[0],
                        height: v[1] - v[0],
                    },
                );
                horizontal_edges.insert(position, horizontal_trench_map.edge(v[0], h[0], h[1]));
                horizontal_edges.insert(
                    position.next(Down, 1),
                    horizontal_trench_map.edge(v[1], h[0], h[1]),
                );
                vertical_edges.insert(position, vertical_trench_map.edge(h[0], v[0], v[1]));
                vertical_edges.insert(
                    position.next(Right, 1),
                    vertical_trench_map.edge(h[1], v[0], v[1]),
                );
            }
        }

        let mut did_remove = true;
        while did_remove {
            did_remove = false;
            for y in 0..=vertical_corners.len() {
                for x in 0..=horizontal_corners.len() {
                    let position = Position {
                        x: x as isize + 1,
                        y: y as isize + 1,
                    };
                    if let Some(edge) = horizontal_edges.get(&position).copied() {
                        let up = position.next(Up, 1);
                        if !edge.trench
                            && (areas.get(&position).is_none() || areas.get(&up).is_none())
                        {
                            horizontal_edges.remove(&position);
                            areas.remove(&position);
                            areas.remove(&up);
                            did_remove = true;
                        }
                    }
                    if let Some(edge) = vertical_edges.get(&position).copied() {
                        let left = position.next(Left, 1);
                        if !edge.trench
                            && (areas.get(&position).is_none() || areas.get(&left).is_none())
                        {
                            vertical_edges.remove(&position);
                            areas.remove(&position);
                            areas.remove(&left);
                            did_remove = true;
                        }
                    }
                }
            }
        }

        Ok(areas
            .iter()
            .map(|(position, area)| {
                let mut sum = area.width * area.height;
                let right = position.next(Right, 1);
                let down = position.next(Down, 1);
                if areas.get(&right).is_none() {
                    sum += area.height;
                }
                if areas.get(&down).is_none() {
                    sum += area.width;
                }
                sum
            })
            .sum::<isize>()
            + 1)
    }

    fn trench_maps(parsed: &<Self as Day>::Parsed) -> (TrenchMap, TrenchMap) {
        let mut horizontal = TrenchMap::new();
        let mut vertical = TrenchMap::new();
        let mut position = Position { x: 1, y: 1 };

        for PlanItem {
            direction,
            distance,
        } in parsed.iter().copied()
        {
            let next = position.next(direction, distance);
            match direction {
                Left | Right => {
                    horizontal.insert(position.y, Range::new(position.x, next.x));
                }
                Up | Down => {
                    vertical.insert(position.x, Range::new(position.y, next.y));
                }
            }
            position = next;
        }
        (horizontal, vertical)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlanItem {
    direction: Direction,
    distance: isize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct TrenchMap(HashMap<isize, Vec<Range>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    length: isize,
    trench: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: isize,
    end: isize,
}


#[derive(Debug, Clone, Copy)]
struct Area {
    width: isize,
    height: isize,
}

impl Position {
    fn next(&self, direction: Direction, distance: isize) -> Self {
        let mut next = *self;
        match direction {
            Up => next.y -= distance,
            Down => next.y += distance,
            Left => next.x -= distance,
            Right => next.x += distance,
        }
        next
    }
}

impl TrenchMap {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, i: isize, range: Range) {
        self.0
            .entry(i)
            .and_modify(|ranges| ranges.push(range))
            .or_insert(vec![range]);
    }

    fn find(&self, i: isize, x: isize) -> Option<Range> {
        self.0
            .get(&i)
            .and_then(|ranges| ranges.iter().find(|range| range.contains(x)).copied())
    }

    fn keys(&self) -> Vec<isize> {
        let mut keys = self.0.keys().copied().collect::<Vec<_>>();
        keys.sort();
        keys
    }

    fn edge(&self, i: isize, x1: isize, x2: isize) -> Edge {
        Edge {
            length: x2 - x1,
            trench: match (self.find(i, x1), self.find(i, x2)) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
        }
    }
}

impl Range {
    fn new(i1: isize, i2: isize) -> Self {
        Self {
            start: i1.min(i2),
            end: i1.max(i2),
        }
    }

    fn contains(&self, i: isize) -> bool {
        self.start <= i && self.end >= i
    }
}

struct Parser;
impl Parser {
    fn input(s: &str, part: Part) -> IResult<<Day18 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            match part {
                Part1 => Parser::part1,
                Part2 => Parser::part2,
            },
        ))(s)
    }

    fn part1(s: &str) -> IResult<PlanItem> {
        map(
            tuple((
                Parser::direction_char,
                delimited(tag(" "), map(i64, |i| i as isize), not_line_ending),
            )),
            |(direction, distance)| PlanItem {
                direction,
                distance,
            },
        )(s)
    }

    fn part2(s: &str) -> IResult<PlanItem> {
        map(
            preceded(
                tuple((anychar, tag(" "), i64, tag(" "))),
                delimited(
                    tag("(#"),
                    tuple((Parser::distance_hex, Parser::direction_hex)),
                    tag(")"),
                ),
            ),
            |(distance, direction)| PlanItem {
                direction,
                distance,
            },
        )(s)
    }

    fn direction_char(s: &str) -> IResult<Direction> {
        alt((
            map(char('U'), |_| Up),
            map(char('D'), |_| Down),
            map(char('L'), |_| Left),
            map(char('R'), |_| Right),
        ))(s)
    }

    fn direction_hex(s: &str) -> IResult<Direction> {
        alt((
            map(char('0'), |_| Right),
            map(char('1'), |_| Down),
            map(char('2'), |_| Left),
            map(char('3'), |_| Up),
        ))(s)
    }

    fn distance_hex(s: &str) -> IResult<isize> {
        map_res(take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit()), |hex_str| {
            isize::from_str_radix(hex_str, 16)
        })(s)
    }
}
