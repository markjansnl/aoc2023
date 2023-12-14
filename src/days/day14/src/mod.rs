use std::{collections::HashMap, fmt::Debug};

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day14;
impl Day for Day14 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Vec<char>>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut platform = Platform::from(parsed);
        platform.tilt_north();
        Ok(platform.total_load())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut platform = Platform::from(parsed);
        for i in 1..1_000_000_000 {
            if let Some(prev) = platform.cycle(i) {
                for j in 0..(1_000_000_000 - i) % (i - prev) {
                    platform.cycle(j);
                }
                return Ok(platform.total_load());
            }
        }
        Err(anyhow!("We should have found the cycle by now"))
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RockShape {
    #[default]
    Square,
    Round,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rock {
    x: u8,
    y: u8,
    shape: RockShape,
}

pub struct Platform {
    width: u8,
    height: u8,
    rocks: Vec<Rock>,
    cache: HashMap<Vec<Rock>, usize>,
    cache_hits: u8,
}

impl From<&<Day14 as Day>::Parsed> for Platform {
    fn from(parsed: &<Day14 as Day>::Parsed) -> Self {
        let width = parsed[0].len() as u8;
        let height = parsed.len() as u8;
        let mut rocks = Vec::new();

        for (y, line) in parsed.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                match c {
                    '#' => rocks.push(Rock {
                        x: x as u8 + 1,
                        y: height - y as u8,
                        shape: RockShape::Square,
                    }),
                    'O' => rocks.push(Rock {
                        x: x as u8 + 1,
                        y: height - y as u8,
                        shape: RockShape::Round,
                    }),
                    _ => {}
                }
            }
        }

        Self {
            width,
            height,
            rocks,
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }
}

macro_rules! tilt {
    ($method:ident $c1:ident $c2:ident $sort:tt $($reset:tt)?) => {
        #[inline]
        pub fn $method(&mut self) {
            self.rocks.sort_by(|a, b| {
                a.$c1.cmp(&b.$c1).then_with(|| {
                    #[allow(unused_mut)]
                    let mut cmp = a.$c2.cmp(&b.$c2);
                    reverse!($sort cmp);
                    cmp
                })
            });

            let mut last_rock = Rock::default();
            for rock in self.rocks.iter_mut() {
                if rock.$c1 != last_rock.$c1 {
                    last_rock.$c1 = rock.$c1;
                    last_rock.$c2 = reset!($sort self $($reset)?);
                }
                match rock.shape {
                    RockShape::Square => {
                        last_rock.$c2 = rock.$c2;
                    }
                    RockShape::Round => {
                        next!($sort last_rock $c2);
                        rock.$c2 = last_rock.$c2;
                    }
                }
            }
        }
    };
}

macro_rules! reverse {
    (ascending $x:ident) => {
        $x = $x.reverse();
    };
    (descending $x:ident) => {};
}

macro_rules! reset {
    (ascending $self:ident $reset:tt) => {
        $self.$reset + 1
    };
    (descending $self:ident) => {
        0
    };
}

macro_rules! next {
    (ascending $last_rock:ident $c:ident) => {
        $last_rock.$c -= 1;
    };
    (descending $last_rock:ident $c:ident) => {
        $last_rock.$c += 1;
    };
}

impl Platform {
    tilt!(tilt_north x y ascending height );
    tilt!(tilt_south x y descending );
    tilt!(tilt_east y x ascending width );
    tilt!(tilt_west y x descending );

    pub fn cycle(&mut self, i: usize) -> Option<usize> {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();

        if let Some(prev) = self.cache.get(&self.rocks).copied() {
            self.cache_hits += 1;
            self.cache.clear();
            if self.cache_hits == 2 {
                return Some(prev);
            }
        } else {
            self.cache.insert(self.rocks.clone(), i);
        }
        None
    }

    pub fn total_load(&self) -> <Day14 as Day>::Output {
        self.rocks
            .iter()
            .filter_map(|rock| match rock.shape {
                RockShape::Square => None,
                RockShape::Round => Some(rock.y as usize),
            })
            .sum()
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 1..self.width + 1 {
                if let Some(rock) = self
                    .rocks
                    .iter()
                    .find(|rock| rock.x == x && rock.y == self.height - y)
                {
                    match rock.shape {
                        RockShape::Square => {
                            write!(f, "#")?;
                        }
                        RockShape::Round => {
                            write!(f, "O")?;
                        }
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day14 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            many1(alt((char('#'), char('O'), char('.')))),
        ))(s)
    }
}
