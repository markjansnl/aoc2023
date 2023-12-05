use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::{all_consuming, map, opt},
    multi::{fold_many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day02;
impl Day for Day02 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Games;
    type Output = u32;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Games::try_from(input)
    }

    fn part1(input: &Self::Parsed) -> Result<Self::Output> {
        let max = Grab {
            red: 12,
            green: 13,
            blue: 14,
        };

        Ok(input
            .iter()
            .filter(|game| game.possible(&max))
            .map(|game| game.number)
            .sum())
    }

    fn part2(input: &Self::Parsed) -> Result<Self::Output> {
        Ok(input.iter().map(Game::power).sum())
    }
}

#[derive(Debug)]
pub struct Games(Vec<Game>);

#[derive(Debug)]
pub struct Game {
    number: u32,
    grabs: Vec<Grab>,
}

#[derive(Debug, Default)]
pub struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl TryFrom<&'static str> for Games {
    type Error = anyhow::Error;

    fn try_from(input: &'static str) -> std::result::Result<Self, Self::Error> {
        Ok(Games::parse(input)?.1)
    }
}

impl Games {
    pub fn iter(&self) -> impl Iterator<Item = &Game> {
        self.0.iter()
    }

    fn parse(s: &str) -> IResult<&str, Games> {
        map(all_consuming(separated_list1(newline, Game::parse)), Games)(s)
    }
}

impl Game {
    pub fn possible(&self, max: &Grab) -> bool {
        self.grabs
            .iter()
            .all(|grab| grab.red <= max.red && grab.green <= max.green && grab.blue <= max.blue)
    }

    pub fn power(&self) -> u32 {
        let max = self.grabs.iter().fold(Grab::default(), |max, grab| Grab {
            red: grab.red.max(max.red),
            green: grab.green.max(max.green),
            blue: grab.blue.max(max.blue),
        });
        max.red * max.blue * max.green
    }

    fn parse(s: &str) -> IResult<&str, Game> {
        map(
            separated_pair(preceded(tag("Game "), u32), tag(": "), Game::parse_grabs),
            |(number, grabs)| Game { number, grabs },
        )(s)
    }

    fn parse_grabs(s: &str) -> IResult<&str, Vec<Grab>> {
        separated_list1(tag("; "), Grab::parse)(s)
    }
}

impl Grab {
    fn parse(s: &str) -> IResult<&str, Grab> {
        fold_many1(
            pair(opt(tag(", ")), Grab::parse_grab_color),
            Grab::default,
            |mut grab, (_, (count, color))| {
                match color {
                    "red" => grab.red = count,
                    "green" => grab.green = count,
                    "blue" => grab.blue = count,
                    _ => unreachable!(),
                }
                grab
            },
        )(s)
    }

    fn parse_grab_color(s: &str) -> IResult<&str, (u32, &str)> {
        separated_pair(u32, tag(" "), Grab::parse_color)(s)
    }

    fn parse_color(s: &str) -> IResult<&str, &str> {
        alt((tag("red"), tag("green"), tag("blue")))(s)
    }
}
