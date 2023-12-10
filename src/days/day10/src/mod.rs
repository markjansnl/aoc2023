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
        Ok(Map { map, start })
    }

    fn part1(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut map = parsed.clone();
        Ok(Day10::bfs_loop(&mut map))
    }

    fn part2(parsed: &Self::Parsed) -> Result<Self::Output> {
        let mut map = parsed.clone();
        Day10::bfs_loop(&mut map);
        for x in 0..map.map[0].len() as i32 {
            map.flood(0, x);
            map.flood(map.map.len() as i32 - 1, x);
        }
        for y in 1..map.map.len() as i32 - 1 {
            map.flood(y, 0);
            map.flood(y, map.map[0].len() as i32 - 1);
        }

        // 622 is too high

        Ok(map.len())
    }
}

impl Day10 {
    fn bfs_loop(map: &mut Map) -> i32 {
        let mut pipes = vec![(map.start, 'S')];
        let mut depth = -1;
        while pipes.len() > 0 {
            depth += 1;
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
        depth
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    map: Vec<Vec<Option<char>>>,
    start: (i32, i32),
}

impl Map {
    pub fn move_left(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.move_checked(y, x - 1, pipes, "-LF")
    }

    pub fn move_right(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.move_checked(y, x + 1, pipes, "-J7")
    }

    pub fn move_up(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.move_checked(y - 1, x, pipes, "|7F")
    }

    pub fn move_down(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>) {
        self.move_checked(y + 1, x, pipes, "|LJ")
    }

    pub fn flood(&mut self, y: i32, x: i32) {
        if self.take(y, x).is_some() {
            for (y, x) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                self.flood(y, x)
            }
        }
    }

    pub fn len(&self) -> i32 {
        self.map
            .iter()
            .flat_map(|line| line.iter().filter_map(|option_c| *option_c))
            .count() as i32
    }

    fn move_checked(&mut self, y: i32, x: i32, pipes: &mut Vec<((i32, i32), char)>, valid: &str) {
        if let Some(c) = self.get(y, x) {
            if valid.contains(c) {
                pipes.push(((y, x), self.take(y, x).unwrap()));
            }
        }
    }

    fn get(&mut self, y: i32, x: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.map.get(y as usize)?.get(x as usize)?.clone()
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
