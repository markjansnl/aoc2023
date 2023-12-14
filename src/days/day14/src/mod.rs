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
        for i in 0..1_000_000_000 {
            if let Some(cycle_start) = platform.cycle(i) {
                let finish = (1_000_000_000 - i - 1) % (i - cycle_start + 1);
                for j in 0..finish {
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
    fn from(input: &<Day14 as Day>::Parsed) -> Self {
        let width = input[0].len() as u8;
        let height = input[1].len() as u8;
        let mut rocks = Vec::new();

        for (y, line) in input.iter().enumerate() {
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

impl Platform {
    pub fn cycle(&mut self, i: usize) -> Option<usize> {
        self.cache.insert(self.rocks.clone(), i);

        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();

        if let Some(cycle_start) = self.cache.get(&self.rocks).copied() {
            self.cache_hits += 1;
            self.cache.clear();
            if self.cache_hits == 2 {
                return Some(cycle_start);
            }
        }
        None
    }

    #[inline]
    pub fn tilt_north(&mut self) {
        self.rocks
            .sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y).reverse()));

        let mut last_rock = Rock::default();
        for rock in self.rocks.iter_mut() {
            if rock.x != last_rock.x {
                last_rock.x = rock.x;
                last_rock.y = self.height + 1;
            }
            match rock.shape {
                RockShape::Square => {
                    last_rock.y = rock.y;
                }
                RockShape::Round => {
                    last_rock.y -= 1;
                    rock.y = last_rock.y;
                }
            }
        }
    }

    #[inline]
    pub fn tilt_south(&mut self) {
        self.rocks
            .sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

        let mut last_rock = Rock::default();
        for rock in self.rocks.iter_mut() {
            if rock.x != last_rock.x {
                last_rock.x = rock.x;
                last_rock.y = 0;
            }
            match rock.shape {
                RockShape::Square => {
                    last_rock.y = rock.y;
                }
                RockShape::Round => {
                    last_rock.y += 1;
                    rock.y = last_rock.y;
                }
            }
        }
    }

    #[inline]
    pub fn tilt_east(&mut self) {
        self.rocks
            .sort_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x).reverse()));

        let mut last_rock = Rock::default();
        for rock in self.rocks.iter_mut() {
            if rock.y != last_rock.y {
                last_rock.y = rock.y;
                last_rock.x = self.width + 1;
            }
            match rock.shape {
                RockShape::Square => {
                    last_rock.x = rock.x;
                }
                RockShape::Round => {
                    last_rock.x -= 1;
                    rock.x = last_rock.x;
                }
            }
        }
    }

    #[inline]
    pub fn tilt_west(&mut self) {
        self.rocks
            .sort_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)));

        let mut last_rock = Rock::default();
        for rock in self.rocks.iter_mut() {
            if rock.y != last_rock.y {
                last_rock.x = 0;
                last_rock.y = rock.y;
            }
            match rock.shape {
                RockShape::Square => {
                    last_rock.x = rock.x;
                }
                RockShape::Round => {
                    last_rock.x += 1;
                    rock.x = last_rock.x;
                }
            }
        }
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
