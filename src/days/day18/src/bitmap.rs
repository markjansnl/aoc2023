use std::collections::HashMap;

use super::{
    Tile,
    Command::{self, *},
    Executor, Position,
};
use crate::prelude::*;

#[derive(Default)]
pub struct Bitmap {
    map: HashMap<Position, Tile>,
    // TODO: Boundaries
}

impl Executor for Bitmap {
    fn execute(&mut self, command: Command) -> Result<()> {
        match command {
            Dig(position) => {
                self.map.insert(position, Tile::Digged);
            }
            Paint(position, rgb) => {
                let tile = self
                    .map
                    .get_mut(&position)
                    .context("You can only paint digged positions")?;
                *tile = Tile::Painted(rgb);
            }
        }
        Ok(())
    }
}

impl Bitmap {
    pub fn boundaries(&self) -> (isize, isize, isize, isize) {
        self.map.keys().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(top, down, left, right), Position { x, y }| {
                (top.min(*y), down.max(*y), left.min(*x), right.max(*x))
            },
        )
    }

    pub fn get(&self, x: isize, y: isize) -> Option<Tile> {
        self.map.get(&Position { x, y }).copied()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
