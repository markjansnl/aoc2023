use std::{cell::RefCell, rc::Rc};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day11;
impl Day for Day11 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Image;
    type Output = usize;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(Image(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Rc::new(RefCell::new(Galaxy([y, x]))))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        ))
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let image = parsed.clone();
        image.expand(X, 2);
        image.expand(Y, 2);
        Ok(image.sum_lengths())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let image = parsed.clone();
        image.expand(X, 1_000_000);
        image.expand(Y, 1_000_000);
        Ok(image.sum_lengths())
    }
}

#[derive(Clone)]
pub struct Image(Vec<Rc<RefCell<Galaxy>>>);

pub struct Galaxy([usize; 2]);

pub const Y: usize = 0;
pub const X: usize = 1;

impl Image {
    pub fn expand(&self, key: usize, times: usize) {
        let mut sorted = self.0.clone();
        sorted.sort_by_key(|galaxy| galaxy.borrow().0[key]);
        let mut i = 0;

        while i < sorted.last().unwrap().borrow().0[key] {
            if sorted.first().unwrap().borrow().0[key] > i {
                for galaxy in &sorted {
                    galaxy.borrow_mut().0[key] += times - 1;
                }
                i += times;
            } else {
                while sorted.first().unwrap().borrow().0[key] == i {
                    sorted.remove(0);
                }
                i += 1;
            }
        }
    }

    pub fn sum_lengths(&self) -> usize {
        let len = self.0.len();
        (0..len)
            .flat_map(|i| (i + 1..len).map(move |j| self.0[i].borrow().length(&self.0[j].borrow())))
            .sum()
    }
}

impl Galaxy {
    pub fn length(&self, other: &Galaxy) -> usize {
        let len = if self.0[X] > other.0[X] {
            self.0[X] - other.0[X]
        } else {
            other.0[X] - self.0[X]
        };
        len + if self.0[Y] > other.0[Y] {
            self.0[Y] - other.0[Y]
        } else {
            other.0[Y] - self.0[Y]
        }
    }
}
