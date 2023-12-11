use std::{cell::RefCell, rc::Rc};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day11;
impl Day for Day11 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Vec<Galaxy>;
    type Output = usize;

    fn reuse_parsed() -> bool {
        false
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Galaxy([y, x]))
                    } else {
                        None
                    }
                })
            })
            .collect())
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let image = Image::from(parsed);
        image.expand(2);
        Ok(image.sum_lengths())
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let image = Image::from(parsed);
        image.expand(1_000_000);
        Ok(image.sum_lengths())
    }
}

#[derive(Clone)]
pub struct Image(Vec<Rc<RefCell<Galaxy>>>);

#[derive(Clone, Copy)]
pub struct Galaxy([usize; 2]);

pub const Y: usize = 0;
pub const X: usize = 1;

impl From<&Vec<Galaxy>> for Image {
    fn from(vec: &Vec<Galaxy>) -> Self {
        Image(
            vec.iter()
                .copied()
                .map(|galaxy| Rc::new(RefCell::new(galaxy)))
                .collect(),
        )
    }
}

impl Image {
    pub fn expand(&self, times: usize) {
        self.expand_key(X, times);
        self.expand_key(Y, times);
    }

    pub fn expand_key(&self, key: usize, times: usize) {
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
        self.0[X].abs_diff(other.0[X]) + self.0[Y].abs_diff(other.0[Y])
    }
}
