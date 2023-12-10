use std::{
    fmt::{self, Debug},
    iter::repeat,
};

use super::inputs::{Inputs, INPUTS};
use crate::prelude::*;

pub struct Day10;
impl Day for Day10 {
    const INPUTS: Self::Inputs = INPUTS;
    type Inputs = Inputs;
    type Parsed = Map;
    type Output = i32;

    fn reuse_parsed() -> bool {
        true
    }

    fn parse(input: &'static str, _part: Part) -> Result<Self::Parsed> {
        let mut start = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (y as i32, x as i32);
                            None
                        } else {
                            Some(c)
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            map,
            start,
            double: false,
        })
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut map = parsed.clone();
        let (depth, _) = Day10::bfs_loop(&mut map);
        Ok(depth)
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut map = parsed.double();
        let (_, last) = Day10::bfs_loop(&mut map);
        map.close_loop(last);
        map.flood_borders();
        Ok(map.undouble().len())
    }
}

impl Day10 {
    fn bfs_loop(map: &mut Map) -> (i32, ((i32, i32), char)) {
        let mut pipes = vec![(map.start, 'S')];
        let mut depth = -1;
        let mut last = Vec::new();
        while !pipes.is_empty() {
            depth += 1;
            last = pipes.clone();
            pipes = pipes
                .into_iter()
                .fold(Vec::new(), |mut pipes, ((y, x), c)| {
                    match c {
                        'S' => {
                            map.move_left(y, x, &mut pipes);
                            map.move_right(y, x, &mut pipes);
                            map.move_up(y, x, &mut pipes);
                            map.move_down(y, x, &mut pipes);
                        }
                        '|' => {
                            map.move_up(y, x, &mut pipes);
                            map.move_down(y, x, &mut pipes);
                        }
                        '-' => {
                            map.move_left(y, x, &mut pipes);
                            map.move_right(y, x, &mut pipes);
                        }
                        'L' => {
                            map.move_up(y, x, &mut pipes);
                            map.move_right(y, x, &mut pipes);
                        }
                        'J' => {
                            map.move_up(y, x, &mut pipes);
                            map.move_left(y, x, &mut pipes);
                        }
                        '7' => {
                            map.move_down(y, x, &mut pipes);
                            map.move_left(y, x, &mut pipes);
                        }
                        'F' => {
                            map.move_down(y, x, &mut pipes);
                            map.move_right(y, x, &mut pipes);
                        }
                        '.' => {}
                        _ => {
                            panic!("Unknown character {c}");
                        }
                    }
                    pipes
                });
        }
        (depth, last[0])
    }
}

#[derive(Clone)]
pub struct Map {
    map: Vec<Vec<Option<char>>>,
    start: (i32, i32),
    double: bool,
}

impl Map {
    pub fn move_left(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        if self.move_checked(y, x - self.one(), pipes, "-LF") && self.double {
            self.take(y, x - 1);
        }
    }

    pub fn move_right(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        if self.move_checked(y, x + self.one(), pipes, "-J7") && self.double {
            self.take(y, x + 1);
        }
    }

    pub fn move_up(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        if self.move_checked(y - self.one(), x, pipes, "|7F") && self.double {
            self.take(y - 1, x);
        }
    }

    pub fn move_down(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        if self.move_checked(y + self.one(), x, pipes, "|LJ") && self.double {
            self.take(y + 1, x);
        }
    }

    pub fn double(&self) -> Map {
        let mut new_map = Map {
            map: Vec::with_capacity(self.map.len() * 2),
            start: (self.start.0 * 2, self.start.1 * 2),
            double: true,
        };

        for line in &self.map {
            let line_len = line.len() * 2;
            let mut new_line = Vec::with_capacity(line_len);
            for option_c in line {
                new_line.push(*option_c);
                new_line.push(Some('.'));
            }
            new_map.map.push(new_line);
            new_map
                .map
                .push(Vec::from_iter(repeat(Some('.')).take(line_len)));
        }

        new_map
    }

    pub fn close_loop(&mut self, last: ((i32, i32), char)) {
        let ((y, x), c) = last;
        match c {
            '|' => {
                self.take(y - 1, x);
                self.take(y + 1, x);
            }
            '-' => {
                self.take(y, x - 1);
                self.take(y, x + 1);
            }
            'L' => {
                self.take(y - 1, x);
                self.take(y, x + 1);
            }
            'J' => {
                self.take(y - 1, x);
                self.take(y, x - 1);
            }
            '7' => {
                self.take(y + 1, x);
                self.take(y, x - 1);
            }
            'F' => {
                self.take(y + 1, x);
                self.take(y, x + 1);
            }
            _ => {
                panic!("Unknown last character {c}");
            }
        }
    }

    pub fn flood_borders(&mut self) {
        for x in 0..self.map[0].len() as i32 {
            self.flood(0, x);
            self.flood(self.map.len() as i32 - 1, x);
        }
        for y in 1..self.map.len() as i32 - 1 {
            self.flood(y, 0);
            self.flood(y, self.map[0].len() as i32 - 1);
        }
    }

    pub fn undouble(self) -> Map {
        let mut new_map = Map {
            map: Vec::with_capacity(self.map.len() / 2),
            start: (self.start.0 / 2, self.start.1 / 2),
            double: false,
        };

        for (_, line) in self.map.iter().enumerate().filter(|(y, _)| y % 2 == 0) {
            let line_len = line.len() / 2;
            let mut new_line = Vec::with_capacity(line_len);
            for (_, option_c) in line.iter().enumerate().filter(|(x, _)| x % 2 == 0) {
                new_line.push(*option_c);
            }
            new_map.map.push(new_line);
        }

        new_map
    }

    pub fn len(&self) -> i32 {
        self.map
            .iter()
            .flat_map(|line| line.iter().filter_map(|option_c| *option_c))
            .count() as i32
    }

    fn one(&self) -> i32 {
        if self.double {
            2
        } else {
            1
        }
    }

    fn flood(&mut self, y: i32, x: i32) {
        if self.take(y, x).is_some() {
            for (y, x) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                self.flood(y, x)
            }
        }
    }

    fn move_checked(
        &mut self,
        y: i32,
        x: i32,
        pipes: &mut Vec<((i32, i32), char)>,
        valid: &str,
    ) -> bool {
        if let Some(c) = self.get(y, x) {
            if valid.contains(c) {
                pipes.push(((y, x), self.take(y, x).unwrap()));
                return true;
            }
        }
        false
    }

    fn get(&mut self, y: i32, x: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            *self.map.get(y as usize)?.get(x as usize)?
        }
    }

    fn take(&mut self, y: i32, x: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.map.get_mut(y as usize)?.get_mut(x as usize)?.take()
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for option_c in line {
                if let Some(c) = option_c {
                    write!(f, "{c}")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
