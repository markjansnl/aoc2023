use std::{collections::HashMap, iter::repeat};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, multispace1, u64, alpha1},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day08;
impl Day for Day08 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = (Vec<Instruction>, Vec<(Node, (Node, Node))>);
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let map = parsed.1.iter().copied().collect::<HashMap<_, _>>();
        let mut instructions = repeat(parsed.0.iter()).flatten();
        let mut current = "AAA";
        let mut steps = 0;
        while current != "ZZZ" {
            let map_item = map.get(current).context(format!("Node {current} is not found in the network"))?;
            current = match instructions.next().unwrap() {
                Instruction::Left => map_item.0,
                Instruction::Right => map_item.1,
            };
            steps += 1;
        }
        Ok(steps)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(todo!())
    }
}

pub enum Instruction {
    Left,
    Right,
}

pub type Node = &'static str;

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day08 as Day>::Parsed> {
        all_consuming(separated_pair(
            Parser::instructions,
            multispace1,
            Parser::network,
        ))(s)
    }

    fn instructions(s: &'static str) -> IResult<Vec<Instruction>> {
        many1(alt((
            map(tag("L"), |_| Instruction::Left),
            map(tag("R"), |_| Instruction::Right),
        )))(s)
    }

    fn network(s: &'static str) -> IResult<Vec<(Node, (Node, Node))>> {
        separated_list1(line_ending, Parser::map_item)(s)
    }

    fn map_item(s: &'static str) -> IResult<(Node, (Node, Node))> {
        separated_pair(
            Parser::node,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(Parser::node, tag(", "), Parser::node),
                tag(")"),
            ),
        )(s)
    }

    fn node(s: &'static str) -> IResult<Node> {
        alpha1(s)
    }
}
