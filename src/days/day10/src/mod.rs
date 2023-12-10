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
                        }
                        Some(c)
                    })
                    .collect()
            })
            .collect();
        Ok(Map { map, start })
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut map = parsed.clone();
        let mut pipes = vec![(map.start, map.take(map.start.0, map.start.1).unwrap())];
        let mut depth = -1;
        while pipes.len() > 0 {
            depth += 1;
            pipes = pipes
                .into_iter()
                .fold(Vec::new(), |mut pipes, ((y, x), c)| {
                    match c {
                        '|' => {
                            map.push_up(y, x, &mut pipes);
                            map.push_down(y, x, &mut pipes);
                        }
                        '-' => {
                            map.push_left(y, x, &mut pipes);
                            map.push_right(y, x, &mut pipes);
                        }
                        'L' => {
                            map.push_up(y, x, &mut pipes);
                            map.push_right(y, x, &mut pipes);
                        }
                        'J' => {
                            map.push_up(y, x, &mut pipes);
                            map.push_left(y, x, &mut pipes);
                        }
                        '7' => {
                            map.push_down(y, x, &mut pipes);
                            map.push_left(y, x, &mut pipes);
                        }
                        'F' => {
                            map.push_down(y, x, &mut pipes);
                            map.push_right(y, x, &mut pipes);
                        }
                        'S' => {
                            map.push_left(y, x, &mut pipes);
                            map.push_right(y, x, &mut pipes);
                            map.push_up(y, x, &mut pipes);
                            map.push_down(y, x, &mut pipes);
                        }
                        '.' => {}
                        _ => {
                            panic!("Unknown character {c}");
                        }
                    }
                    pipes
                });
            dbg!(&pipes);
        }

        Ok(depth)
    }

    fn part2(_parsed: &Self::Parsed) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    map: Vec<Vec<Option<char>>>,
    start: (i32, i32),
}

impl Map {
    pub fn push_left(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.push(y, x - 1, pipes, "-LF")
    }

    pub fn push_right(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.push(y, x + 1, pipes, "-J7")
    }

    pub fn push_up(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.push(y - 1, x, pipes, "|7F")
    }

    pub fn push_down(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.push(y + 1, x, pipes, "|LJ")
    }

    fn push(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>, validate: &str) {
        if let Some(c) = self.get(y, x) {
            if validate.contains(c) {
                pipes.push(((y, x), self.take(y, x).unwrap()));
            }
        }
    }

    pub fn get(&mut self, y: i32, x: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.map.get(y as usize)?.get(x as usize)?.clone()
        }
    }

    pub fn take(&mut self, y: i32, x: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.map.get_mut(y as usize)?.get_mut(x as usize)?.take()
        }
    }
}
