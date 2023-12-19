use std::{collections::HashMap, ops::Index};

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

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
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

pub struct Workflow(Vec<Rule>);

pub type Label = &'static str;

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
            match self.0.get(workflow).context(format!("Workflow {workflow} not found"))?.process(part_rating)? {
                Destination::Decision(decision) => return Ok(decision),
                Destination::Workflow(next) => workflow = next,
            }
        }
    }
}

impl Workflow {
    fn process(&self, part_rating: PartRating) -> Result<Destination> {
        Ok(self.0.iter().find(|rule| rule.matches(part_rating)).context("None of the rules matches")?.destination())
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
}

impl Condition {
    fn matches(&self, part_rating: PartRating) -> bool {
        match self.operator {
            Operator::LessThan => part_rating[self.category] < self.value,
            Operator::GreaterThan => part_rating[self.category] > self.value,
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
