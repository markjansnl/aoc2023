use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, u64},
    combinator::{all_consuming, map},
    multi::{many_m_n, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day19;
impl Day for Day19 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = (System, Vec<PartRating>);
    type Output = Value;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let (system, part_ratings) = parsed;
        system.sum(part_ratings)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let (system, _) = parsed;
        system.possibilities_accepted("in", &mut PartRatingRange::default())
    }
}

#[derive(Clone, Copy)]
pub enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

pub type Value = usize;

pub struct System(HashMap<Label, Workflow>);

pub type Label = &'static str;

pub struct Workflow(Vec<Rule>);

pub enum Rule {
    Condition(Condition),
    Destination(Destination),
}

pub struct Condition {
    category: Category,
    operator: Operator,
    value: usize,
    destination: Destination,
}

pub enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Clone, Copy)]
pub enum Destination {
    Decision(Decision),
    Workflow(Label),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PartRating {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl System {
    fn sum(&self, part_ratings: &Vec<PartRating>) -> Result<Value> {
        let mut sum = 0;
        for part_rating in part_ratings {
            if self.process(*part_rating)? == Decision::Accepted {
                sum += part_rating.sum();
            }
        }
        Ok(sum)
    }

    fn process(&self, part_rating: PartRating) -> Result<Decision> {
        let mut workflow = "in";
        loop {
            match self
                .0
                .get(workflow)
                .context(format!("Workflow {workflow} not found"))?
                .process(part_rating)?
            {
                Destination::Decision(decision) => return Ok(decision),
                Destination::Workflow(next) => workflow = next,
            }
        }
    }

    fn possibilities_accepted(
        &self,
        workflow: Label,
        part_rating_range: &mut PartRatingRange,
    ) -> Result<Value> {
        self.0
            .get(workflow)
            .context(format!("Workflow {workflow} not found"))?
            .possibilities_accepted(self, part_rating_range)
    }
}

impl Workflow {
    fn process(&self, part_rating: PartRating) -> Result<Destination> {
        Ok(self
            .0
            .iter()
            .find(|rule| rule.matches(part_rating))
            .context("None of the rules matches")?
            .destination())
    }

    fn possibilities_accepted(
        &self,
        system: &System,
        part_rating_range: &mut PartRatingRange,
    ) -> Result<Value> {
        self.0
            .iter()
            .map(|rule| rule.possibilities_accepted(system, part_rating_range))
            .sum::<Result<Value>>()
    }
}

impl Rule {
    fn matches(&self, part_rating: PartRating) -> bool {
        match self {
            Rule::Condition(condition) => condition.matches(part_rating),
            Rule::Destination(_) => true,
        }
    }

    fn destination(&self) -> Destination {
        match self {
            Rule::Condition(condition) => condition.destination,
            Rule::Destination(destination) => *destination,
        }
    }

    fn possibilities_accepted(
        &self,
        system: &System,
        part_rating_range: &mut PartRatingRange,
    ) -> Result<Value> {
        match self {
            Rule::Condition(condition) => {
                condition.possibilities_accepted(system, part_rating_range)
            }
            Rule::Destination(destination) => {
                destination.possibilities_accepted(system, part_rating_range)
            }
        }
    }
}

impl Condition {
    fn matches(&self, part_rating: PartRating) -> bool {
        match self.operator {
            Operator::LessThan => part_rating[self.category] < self.value,
            Operator::GreaterThan => part_rating[self.category] > self.value,
        }
    }

    fn possibilities_accepted(
        &self,
        system: &System,
        part_rating_range: &mut PartRatingRange,
    ) -> Result<Value> {
        self.destination.possibilities_accepted(
            system,
            &mut match self.operator {
                Operator::LessThan => {
                    part_rating_range.possibilities(self.category, 1, self.value - 1)
                }
                Operator::GreaterThan => {
                    part_rating_range.possibilities(self.category, self.value + 1, 4000)
                }
            },
        )
    }
}

impl Destination {
    fn possibilities_accepted(
        &self,
        system: &System,
        part_rating_range: &mut PartRatingRange,
    ) -> Result<Value> {
        match *self {
            Destination::Decision(decision) => {
                Ok(decision.possibilities_accepted(part_rating_range))
            }
            Destination::Workflow(workflow) => {
                system.possibilities_accepted(workflow, part_rating_range)
            }
        }
    }
}

impl Decision {
    fn possibilities_accepted(&self, part_rating_range: &PartRatingRange) -> Value {
        match *self {
            Decision::Accepted => part_rating_range.possibilities_accepted(),
            Decision::Rejected => 0,
        }
    }
}

impl Index<Category> for PartRating {
    type Output = Value;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::ExtremelyCoolLooking => &self.x,
            Category::Musical => &self.m,
            Category::Aerodynamic => &self.a,
            Category::Shiny => &self.s,
        }
    }
}

impl PartRating {
    fn sum(&self) -> Value {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Range(Vec<SubRange>);

#[derive(Debug, Clone, Copy)]
struct SubRange {
    start: usize,
    end: usize,
}

#[derive(Debug, Default, Clone)]
struct PartRatingRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Default for Range {
    fn default() -> Self {
        Self(vec![SubRange {
            start: 1,
            end: 4000,
        }])
    }
}

impl Range {
    fn possibilities(&mut self, start: usize, end: usize) -> usize {
        let mut possibilities = 0;
        self.0 = self
            .0
            .iter()
            .copied()
            .fold(Vec::new(), |mut sub_ranges, mut sub_range| {
                if sub_range.contains(start) {
                    possibilities += end.min(sub_range.end) - start + 1;
                    if sub_range.contains(end) && end < sub_range.end {
                        sub_ranges.push(SubRange {
                            start: end + 1,
                            end: sub_range.end,
                        });
                    }
                    sub_range.end = start - 1;
                } else if sub_range.contains(end) {
                    possibilities += end - sub_range.start + 1;
                    sub_range.start = end + 1;
                }
                if sub_range.start <= sub_range.end {
                    sub_ranges.push(sub_range);
                }
                sub_ranges
            });
        possibilities
    }

    fn possibilities_accepted(&self) -> Value {
        self.0.iter().map(SubRange::possibilities_accepted).sum()
    }
}

impl SubRange {
    fn contains(&self, i: usize) -> bool {
        self.start <= i && self.end >= i
    }

    fn possibilities_accepted(&self) -> Value {
        self.end - self.start + 1
    }
}

impl PartRatingRange {
    fn possibilities(&mut self, category: Category, start: usize, end: usize) -> Self {
        let mut possibilities = self.clone();
        self[category].possibilities(start, end);
        possibilities[category].possibilities(0, start - 1);
        possibilities[category].possibilities(end + 1, 4001);
        possibilities
    }

    fn possibilities_accepted(&self) -> Value {
        [&self.x, &self.m, &self.a, &self.s]
            .into_iter()
            .map(|range| range.possibilities_accepted())
            .product()
    }
}

impl Index<Category> for PartRatingRange {
    type Output = Range;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::ExtremelyCoolLooking => &self.x,
            Category::Musical => &self.m,
            Category::Aerodynamic => &self.a,
            Category::Shiny => &self.s,
        }
    }
}

impl IndexMut<Category> for PartRatingRange {
    fn index_mut(&mut self, category: Category) -> &mut Self::Output {
        match category {
            Category::ExtremelyCoolLooking => &mut self.x,
            Category::Musical => &mut self.m,
            Category::Aerodynamic => &mut self.a,
            Category::Shiny => &mut self.s,
        }
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day19 as Day>::Parsed> {
        all_consuming(separated_pair(
            Parser::system,
            many_m_n(2, 2, line_ending),
            separated_list1(line_ending, Parser::part_rating),
        ))(s)
    }

    fn system(s: &'static str) -> IResult<System> {
        map(separated_list1(line_ending, Parser::workflow), |vec| {
            System(vec.into_iter().collect())
        })(s)
    }

    fn workflow(s: &'static str) -> IResult<(Label, Workflow)> {
        tuple((
            alpha1,
            map(
                delimited(tag("{"), separated_list1(tag(","), Parser::rule), tag("}")),
                Workflow,
            ),
        ))(s)
    }

    fn rule(s: &'static str) -> IResult<Rule> {
        alt((
            map(Parser::condition, Rule::Condition),
            map(Parser::destination, Rule::Destination),
        ))(s)
    }

    fn condition(s: &'static str) -> IResult<Condition> {
        map(
            tuple((
                Parser::category,
                Parser::operator,
                Parser::value,
                preceded(tag(":"), Parser::destination),
            )),
            |(category, operator, value, destination)| Condition {
                category,
                operator,
                value,
                destination,
            },
        )(s)
    }

    fn category(s: &'static str) -> IResult<Category> {
        alt((
            map(char('x'), |_| Category::ExtremelyCoolLooking),
            map(char('m'), |_| Category::Musical),
            map(char('a'), |_| Category::Aerodynamic),
            map(char('s'), |_| Category::Shiny),
        ))(s)
    }

    fn operator(s: &'static str) -> IResult<Operator> {
        alt((
            map(char('<'), |_| Operator::LessThan),
            map(char('>'), |_| Operator::GreaterThan),
        ))(s)
    }

    fn value(s: &'static str) -> IResult<Value> {
        map(u64, |value| value as Value)(s)
    }

    fn destination(s: &'static str) -> IResult<Destination> {
        alt((
            map(tag("A"), |_| Destination::Decision(Decision::Accepted)),
            map(tag("R"), |_| Destination::Decision(Decision::Rejected)),
            map(Parser::label, Destination::Workflow),
        ))(s)
    }

    fn label(s: &'static str) -> IResult<Label> {
        alpha1(s)
    }

    fn part_rating(s: &'static str) -> IResult<PartRating> {
        map(
            delimited(
                tag("{"),
                tuple((
                    preceded(tag("x="), Parser::value),
                    preceded(tag(",m="), Parser::value),
                    preceded(tag(",a="), Parser::value),
                    preceded(tag(",s="), Parser::value),
                )),
                tag("}"),
            ),
            |(x, m, a, s)| PartRating { x, m, a, s },
        )(s)
    }
}
