use std::{collections::HashMap, iter::repeat};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace1},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};
use num_integer::Integer;

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day08;
impl Day for Day08 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = (
        Vec<Instruction>,
        Vec<(Node<'static>, (Node<'static>, Node<'static>))>,
    );
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(Self::steps(parsed, "AAA", |node| node == "ZZZ"))
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(parsed
            .1
            .iter()
            .filter_map(|(start_node, _)| {
                if start_node.chars().nth(2).unwrap() == 'A' {
                    let start_to_end = Self::steps(parsed, start_node, |node| {
                        node.chars().nth(2).unwrap() == 'Z'
                    });
                    Some(start_to_end)
                } else {
                    None
                }
            })
            .fold(parsed.0.len(), |lcm, ghost_step| lcm.lcm(&ghost_step)))
    }
}

impl Day08 {
    fn steps(
        parsed: &<Self as Day>::Parsed,
        start_node: Node,
        is_end_node: impl Fn(Node) -> bool,
    ) -> <Self as Day>::Output {
        let map = parsed.1.iter().copied().collect::<HashMap<_, _>>();
        let mut instructions = repeat(parsed.0.iter()).flatten();
        let mut current = start_node;
        let mut steps = 0;
        while !is_end_node(current) || steps == 0 {
            let map_item = map
                .get(current)
                .context(format!("Node {current} is not found in the network"))
                .unwrap();
            current = match instructions.next().unwrap() {
                Instruction::Left => map_item.0,
                Instruction::Right => map_item.1,
            };
            steps += 1;
        }
        steps
    }
}

pub enum Instruction {
    Left,
    Right,
}

pub type Node<'a> = &'a str;

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
        alphanumeric1(s)
    }
}
