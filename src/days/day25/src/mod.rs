use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};
// use pathfinding::directed::bfs::bfs_reach;

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day25;
impl Day for Day25 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = HashMap<Component, Vec<Component>>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        SnowProducer::from(parsed).find_wires()
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        Ok(0)
    }
}

pub type Component = &'static str;

type Wire = (Component, Component);

struct SnowProducer {
    #[cfg(test)]
    wires: Vec<Wire>,
    successors: HashMap<Component, Vec<Component>>,
}

impl From<&<Day25 as Day>::Parsed> for SnowProducer {
    fn from(parsed: &<Day25 as Day>::Parsed) -> Self {
        #[cfg(test)]
        let mut wires = Vec::new();

        let mut successors = parsed.clone();
        for (source, destinations) in parsed {
            for destination in destinations {
                #[cfg(test)]
                wires.push((*source, *destination));

                successors
                    .entry(destination)
                    .and_modify(|destinations| destinations.push(source))
                    .or_insert(vec![*destination]);
            }
        }

        #[cfg(test)]
        return Self { wires, successors };

        #[cfg(not(test))]
        return Self { successors };
    }
}

impl SnowProducer {
    #[cfg(test)]
    fn find_wires(&self) -> Result<usize> {
        for (k, wire1) in self
            .wires
            .iter()
            .copied()
            .enumerate()
            .take(self.wires.len() - 2)
        {
            for (l, wire2) in self
                .wires
                .iter()
                .copied()
                .enumerate()
                .skip(k + 1)
                .take(self.wires.len() - 1)
            {
                for wire3 in self.wires.iter().copied().skip(l + 1) {
                    if let Some(left) = self.separated(wire1, wire2, wire3)? {
                        let right = self.successors.keys().count() - left;
                        return Ok(left * right);
                    }
                }
            }
        }

        Err(anyhow!("Wires not found"))
    }

    #[cfg(not(test))]
    fn find_wires(&self) -> Result<usize> {
        // For the real input the full approach will take approx. a year to compute
        // I used [graphviz](graphviz.svg) in docker to find the 3 wires.
        // The normal layout isn't really usable, so use layout neato.
        // Command: `dot -Tsvg graphviz.txt -Kneato > graphviz.svg`
        // Let's only take these into account.

        let wire1 = ("fsv", "spx");
        let wire2 = ("tvj", "cvx");
        let wire3 = ("kdk", "nct");

        if let Some(left) = self.separated(wire1, wire2, wire3)? {
            let right = self.successors.keys().count() - left;
            return Ok(left * right);
        }

        Err(anyhow!("Wires not found"))
    }

    fn separated(&self, wire1: Wire, wire2: Wire, wire3: Wire) -> Result<Option<usize>> {
        // println!("{wire1:?}, {wire2:?}, {wire3:?}");
        let start = wire1.0;
        let mut queue = vec![start];
        let mut visited = Vec::new();
        let mut other_side = vec![wire1.1];
        let mut unknown = vec![wire2, wire3];

        while let Some(source) = queue.pop() {
            if other_side.contains(&source) {
                // println!("Found other side {source}. return");
                return Ok(None);
            }
            if visited.contains(&source) {
                continue;
            } else {
                visited.push(source);
            }

            // println!("  {source} -->");
            for &destination in self
                .successors
                .get(source)
                .context(format!("Source {source} not found"))?
            {
                // print!("    {destination}: ");
                if (source == wire1.0 && destination == wire1.1)
                    || (source == wire1.1 && destination == wire1.0)
                    || (source == wire2.0 && destination == wire2.1)
                    || (source == wire2.1 && destination == wire2.0)
                    || (source == wire3.0 && destination == wire3.1)
                    || (source == wire3.1 && destination == wire3.0)
                    || visited.contains(&destination)
                {
                    // println!("skipped");
                    continue;
                }

                unknown.retain(|&wire| {
                    if destination == wire.0 {
                        // print!("This side: {}, other side: {}. ", wire.0, wire.1);
                        other_side.push(wire.1);
                        false
                    } else if destination == wire.1 {
                        // print!("This side: {}, other side: {}. ", wire.1, wire.0);
                        other_side.push(wire.0);
                        false
                    } else {
                        true
                    }
                });

                // println!("Push destination.");
                queue.push(destination);
            }
        }

        Ok(Some(visited.len()))
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day25 as Day>::Parsed> {
        map(
            all_consuming(separated_list1(
                line_ending,
                separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
            )),
            HashMap::from_iter,
        )(s)
    }
}

// #[cfg(test)]
// #[test]
// fn test() -> Result<()> {
//     let parsed = Parser::input(Day25::INPUTS[1])?.1;
//     let snow_producer = SnowProducer::from(&parsed);

//     snow_producer.separated(("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt"));

//     Ok(())
// }
