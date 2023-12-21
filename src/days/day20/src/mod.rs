use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use num_integer::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day20;
impl Day for Day20 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<((ModuleType, ModuleName), Vec<ModuleName>)>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Parser::input(input)?.1)
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut system = System::from(parsed);
        let mut counter = Part1Counter::default();

        for _ in 1..=1000 {
            system.push_button(&mut counter);
        }

        Ok(counter.product())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let system = System::from(parsed);
        let rx_grandparents = system.find_rx_grandparents()?;

        let cycles = rx_grandparents
            .par_iter()
            .map(|rx_grandparent| {
                let mut system = system.clone();
                let mut counter = Part2Counter::new(rx_grandparent);

                for count in 1usize.. {
                    system.push_button(&mut counter);
                    if counter.reached() {
                        return count;
                    }
                }

                unreachable!()
            })
            .collect::<Vec<_>>();

        cycles
            .into_iter()
            .reduce(|a, b| a.lcm(&b))
            .context("Cannot reduce when no grandparents are found")
    }
}

pub type ModuleName = &'static str;

pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone)]
struct System<'a> {
    modules: HashMap<ModuleName, Module>,
    destinations: HashMap<ModuleName, &'a Vec<ModuleName>>,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,
    FlipFlop(State),
    Conjunction(HashMap<ModuleName, Pulse>),
}

type State = bool;
type Pulse = bool;
const LOW: Pulse = false;
const HIGH: Pulse = true;

#[derive(Clone, Copy)]
struct Signal {
    source: ModuleName,
    destination: ModuleName,
    pulse: Pulse,
}

trait Counter {
    fn add(&mut self, system: &System, signal: Signal);
}

#[derive(Default)]
struct Part1Counter {
    low: usize,
    high: usize,
}

struct Part2Counter {
    destination: ModuleName,
    reached: bool,
}

impl<'a> From<&'a <Day20 as Day>::Parsed> for System<'a> {
    fn from(parsed: &'a <Day20 as Day>::Parsed) -> Self {
        let mut modules: HashMap<_, Module> = HashMap::new();
        let mut destinations = HashMap::new();

        for ((module_type, module_name), parsed_destinations) in parsed {
            modules.insert(*module_name, module_type.into());
            destinations.insert(*module_name, parsed_destinations);
        }

        for ((_, source), parsed_destinations) in parsed {
            for destination in parsed_destinations {
                if let Some(module) = modules.get_mut(destination) {
                    module.add_source(source);
                }
            }
        }

        Self {
            modules,
            destinations,
        }
    }
}

impl<'a> System<'a> {
    fn push_button(&mut self, counter: &mut impl Counter) {
        let mut dequeue = VecDeque::from([Signal {
            source: "button",
            destination: "broadcaster",
            pulse: LOW,
        }]);

        while let Some(signal) = dequeue.pop_front() {
            counter.add(self, signal);
            self.process_signal(signal, &mut dequeue);
        }
    }

    fn process_signal(&mut self, signal: Signal, dequeue: &mut VecDeque<Signal>) -> Option<()> {
        let pulse = self.modules.get_mut(signal.destination)?.signal(signal)?;

        for next_destination in self.destinations.get(signal.destination)?.iter() {
            dequeue.push_back(Signal {
                source: signal.destination,
                destination: next_destination,
                pulse,
            });
        }

        Some(())
    }

    fn find_rx_grandparents(&self) -> Result<Vec<ModuleName>> {
        let rx_parent = *self
            .destinations
            .iter()
            .find(|(_, destinations)| destinations.contains(&"rx"))
            .context("Cannot find parent of rx")?
            .0;

        Ok(self
            .destinations
            .iter()
            .filter(|(_, destinations)| destinations.contains(&rx_parent))
            .map(|(source, _)| *source)
            .collect())
    }
}

impl From<&ModuleType> for Module {
    fn from(module_type: &ModuleType) -> Self {
        match module_type {
            ModuleType::Broadcaster => Module::Broadcaster,
            ModuleType::FlipFlop => Module::FlipFlop(Default::default()),
            ModuleType::Conjunction => Module::Conjunction(Default::default()),
        }
    }
}

impl Module {
    fn add_source(&mut self, source: ModuleName) {
        if let Module::Conjunction(memory) = self {
            memory.insert(source, Default::default());
        }
    }

    fn signal(&mut self, Signal { source, pulse, .. }: Signal) -> Option<Pulse> {
        match self {
            Module::Broadcaster => Some(pulse),
            Module::FlipFlop(state) => match pulse {
                LOW => {
                    *state = !*state;
                    Some(*state)
                }
                HIGH => None,
            },
            Module::Conjunction(memory) => {
                memory.entry(source).and_modify(|state| *state = pulse);

                if memory.values().all(|state| *state == HIGH) {
                    Some(LOW)
                } else {
                    Some(HIGH)
                }
            }
        }
    }
}

impl Counter for Part1Counter {
    fn add(&mut self, _system: &System, signal: Signal) {
        match signal.pulse {
            LOW => self.low += 1,
            HIGH => self.high += 1,
        }
    }
}

impl Part1Counter {
    fn product(&self) -> usize {
        self.low * self.high
    }
}

impl Counter for Part2Counter {
    fn add(&mut self, _system: &System, signal: Signal) {
        if signal.destination == self.destination && signal.pulse == LOW {
            self.reached = true;
        }
    }
}

impl Part2Counter {
    fn new(destination: ModuleName) -> Self {
        Self {
            destination,
            reached: false,
        }
    }

    fn reached(&self) -> bool {
        self.reached
    }
}

struct Parser;
impl Parser {
    fn input(s: &'static str) -> IResult<<Day20 as Day>::Parsed> {
        all_consuming(separated_list1(
            line_ending,
            separated_pair(
                Parser::sender,
                tag(" -> "),
                separated_list1(tag(", "), Parser::module_name),
            ),
        ))(s)
    }

    fn sender(s: &'static str) -> IResult<(ModuleType, ModuleName)> {
        alt((
            map(tag("broadcaster"), |name| (ModuleType::Broadcaster, name)),
            map(preceded(tag("%"), Parser::module_name), |name| {
                (ModuleType::FlipFlop, name)
            }),
            map(preceded(tag("&"), Parser::module_name), |name| {
                (ModuleType::Conjunction, name)
            }),
        ))(s)
    }

    fn module_name(s: &'static str) -> IResult<ModuleName> {
        alpha1(s)
    }
}

#[cfg(test)]
#[test]
fn print_graphviz() -> Result<()> {
    let input = Day20::INPUTS[0];
    let parsed = Parser::input(input)?.1;
    println!("digraph G {{");
    for ((module_type, source), destinations) in &parsed {
        for destination in destinations {
            println!("{source} -> {destination}");
        }
        let shape = match module_type {
            ModuleType::Broadcaster => "cds",
            ModuleType::FlipFlop => "diamond",
            ModuleType::Conjunction => "doublecircle",
        };
        println!("{source}[shape={shape}]");
    }
    println!("}}");
    Ok(())
}
