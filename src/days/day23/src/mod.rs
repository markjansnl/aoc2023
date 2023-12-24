use std::collections::HashMap;

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
            .longest_path(Node::default())
            .context("Longest path not found")
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let graph = Graph::from(parsed);
        graph
            .longest_path(&GraphNode::default())
            .context("Longest path not found")
    }
}

pub struct Map(Vec<Vec<char>>);

#[derive(Debug, Clone)]
pub struct Graph(HashMap<Tile, HashMap<Tile, usize>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    tile: Tile,
    direction: Direction,
    last: Tile,
    length: usize,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct GraphNode {
    tile: Tile,
    visited: Vec<Tile>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Tile {
    fn default() -> Self {
        Self { x: 2, y: 1 }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tile: Tile::default(),
            direction: Down,
            last: Tile::default(),
            length: 0,
        }
    }
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
        self.successors(node, Part1)
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
            last,
            length,
        }: Node,
        part: Part,
    ) -> Vec<Node> {
        [
            Node {
                tile: Tile { x: x - 1, y },
                direction: Left,
                last,
                length: length + 1,
            },
            Node {
                tile: Tile { x: x + 1, y },
                direction: Right,
                last,
                length: length + 1,
            },
            Node {
                tile: Tile { x, y: y - 1 },
                direction: Up,
                last,
                length: length + 1,
            },
            Node {
                tile: Tile { x, y: y + 1 },
                direction: Down,
                last,
                length: length + 1,
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
                 ..
             }| {
                let next_c = self.get(next_x, next_y);
                next_direction != direction.opposite()
                    && match part {
                        Part1 => next_c == '.' || next_c == next_direction.into(),
                        Part2 => next_c != '#',
                    }
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
        if y == 0 || y > self.0.len() {
            '#'
        } else {
            self.0[y - 1][x - 1]
        }
    }
}

impl From<&Map> for Graph {
    fn from(map: &Map) -> Self {
        let mut graph = Graph(HashMap::new());
        let mut queue = vec![Node::default()];

        while let Some(from) = queue.pop() {
            if graph
                .0
                .get(&from.last)
                .and_then(|successors| successors.get(&from.tile))
                .is_some()
                || graph
                    .0
                    .get(&from.tile)
                    .and_then(|successors| successors.get(&from.last))
                    .is_some()
            {
                continue;
            } else if from.tile == map.end() {
                graph
                    .0
                    .entry(from.last)
                    .and_modify(|successors| {
                        successors.insert(from.tile, from.length);
                    })
                    .or_insert(HashMap::from([(from.tile, from.length)]));
            } else {
                let mut successors = map.successors(from, Part2);
                if successors.len() > 1 {
                    graph
                        .0
                        .entry(from.last)
                        .and_modify(|successors| {
                            successors.insert(from.tile, from.length);
                        })
                        .or_insert(HashMap::from([(from.tile, from.length)]));
                    for successor in successors.iter_mut() {
                        successor.last = from.tile;
                        successor.length = 1;
                    }
                }
                queue.append(&mut successors);
            }
        }

        for (from, successors) in graph.0.clone().into_iter() {
            for (to, length) in successors {
                graph
                    .0
                    .entry(to)
                    .and_modify(|successors| {
                        successors.insert(from, length);
                    })
                    .or_insert(HashMap::from([(from, length)]));
            }
        }

        graph
    }
}

impl Graph {
    fn longest_path(&self, node: &GraphNode) -> Option<usize> {
        if node.tile == self.end() {
            Some(0)
        } else {
            self.successors(node)
                .into_iter()
                .filter_map(|(successor, length)| self.longest_path(&successor).map(|n| n + length))
                .max()
        }
    }

    fn successors(&self, node: &GraphNode) -> Vec<(GraphNode, usize)> {
        let mut successor_visited = node.visited.clone();
        successor_visited.push(node.tile);
        self.0
            .get(&node.tile)
            .unwrap()
            .iter()
            .filter_map(|(successor, length)| {
                if !node.visited.contains(successor) {
                    Some((
                        GraphNode {
                            tile: *successor,
                            visited: successor_visited.clone(),
                        },
                        *length,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    fn end(&self) -> Tile {
        let y = self.0.keys().map(|tile| tile.y).max().unwrap();
        *self.0.keys().find(|tile| tile.y == y).unwrap()
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
