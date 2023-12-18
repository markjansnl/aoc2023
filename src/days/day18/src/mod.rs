use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, i64, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

use self::bitmap::Bitmap;

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

mod bitmap;
mod terminal;

use Command::*;
use Direction::*;

pub struct Day18;
impl Day for Day18 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<PlanItem>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut bitmap = Bitmap::default();
        let mut cursor = Cursor {
            position: Position { x: 1, y: 1 },
            direction: Right,
        };

        for plan_item in parsed {
            cursor.direction = plan_item.direction;
            for _ in 0..plan_item.length {
                cursor.go();
                bitmap.execute(Dig(cursor.position))?;
                bitmap.execute(Paint(cursor.position, plan_item.rgb))?;
            }
        }

        let (boundary_top, boundary_down, boundary_left, boundary_right) = bitmap.boundaries();
        let mut filled = HashSet::new();
        let mut fill = vec![Position {
            x: boundary_left - 1,
            y: boundary_top - 1,
        }];
        while let Some(position) = fill.pop() {
            if position.y >= boundary_top - 1
                && position.y <= boundary_down + 1
                && position.x >= boundary_left - 1
                && position.x <= boundary_right + 1
                && bitmap.get(position.x, position.y).is_none()
                && filled.insert(position)
            {
                for direction in [Up, Down, Left, Right] {
                    let mut cursor = Cursor {
                        position,
                        direction,
                    };
                    cursor.go();
                    fill.push(cursor.position);
                }
            }
        }
        for y in boundary_top - 1..=boundary_down + 1 {
            for x in boundary_left - 1..=boundary_right + 1 {
                let position = Position { x, y };
                if bitmap.get(x, y).is_none() && !filled.contains(&position) {
                    bitmap.execute(Dig(position))?;
                }
            }
        }

        // for y in boundary_top..=boundary_down {
        //     let mut iter = boundary_left..=boundary_right;
        //     let mut out = true;
        //     while let Some(x) = iter.next() {
        //         println!("pos {y}, {x}");
        //         let mut x = x;
        //         if bitmap.get(x, y).is_some() {
        //             if bitmap.get(x + 1, y).is_none() {
        //                 out = !out;
        //                 while let Some(x2) = iter.next() {
        //                     x = x2;
        //                     if bitmap.get(x2, y).is_none() {
        //                         break;
        //                     }
        //                 }
        //             }
        //         }
        //         if !out {
        //             println!("  dig {y}, {x}");
        //             bitmap.execute(Dig(Position { x, y }))?;
        //         }
        //     }
        // }

        // 36627 is too high

        Ok(bitmap.len())
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct PlanItem {
    direction: Direction,
    length: isize,
    rgb: RGB,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum Tile {
    Digged,
    Painted(RGB),
}

#[derive(Clone, Copy)]
pub struct RGB {
    _r: u8,
    _g: u8,
    _b: u8,
}

struct Parser;
impl Parser {
    fn input(s: &str) -> IResult<<Day18 as Day>::Parsed> {
        all_consuming(separated_list1(line_ending, Parser::plan_item))(s)
    }

    fn plan_item(s: &str) -> IResult<PlanItem> {
        map(
            tuple((
                Parser::direction,
                tag(" "),
                map(i64, |i| i as isize),
                tag(" "),
                Parser::color,
            )),
            |(direction, _, length, _, rgb)| PlanItem {
                direction,
                length,
                rgb,
            },
        )(s)
    }

    fn direction(s: &str) -> IResult<Direction> {
        alt((
            map(char('U'), |_| Up),
            map(char('D'), |_| Down),
            map(char('L'), |_| Left),
            map(char('R'), |_| Right),
        ))(s)
    }

    fn color(s: &str) -> IResult<RGB> {
        map(
            delimited(
                tag("(#"),
                tuple((
                    Parser::hex_primary,
                    Parser::hex_primary,
                    Parser::hex_primary,
                )),
                tag(")"),
            ),
            |(r, g, b)| RGB {
                _r: r,
                _g: g,
                _b: b,
            },
        )(s)
    }

    fn hex_primary(s: &str) -> IResult<u8> {
        map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |hex_str| {
            u8::from_str_radix(hex_str, 16)
        })(s)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy)]
pub enum Command {
    Dig(Position),
    Paint(Position, RGB),
}

pub trait Executor {
    fn execute(&mut self, command: Command) -> Result<()>;
}

pub struct Executors<'a>(Vec<&'a mut dyn Executor>);

impl<'a> Executor for Executors<'a> {
    fn execute(&mut self, command: Command) -> Result<()> {
        for executor in self.0.iter_mut() {
            executor.execute(command)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Cursor {
    position: Position,
    direction: Direction,
}

impl Cursor {
    fn go(&mut self) {
        match self.direction {
            Up => self.position.y -= 1,
            Down => self.position.y += 1,
            Left => self.position.x -= 1,
            Right => self.position.x += 1,
        }
    }
}
