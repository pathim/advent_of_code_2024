use std::collections::{HashMap, HashSet};

use crate::AocInput;

pub type Position = (isize, isize);
pub type Positions = HashSet<Position>;

pub struct Grid {
    data: Vec<Vec<char>>,
    pub locations: HashMap<char, Positions>,
    size: (isize, isize),
}

impl Grid {
    pub fn new(input: AocInput, needles: &[char]) -> Self {
        let mut locations: HashMap<char, Positions> = HashMap::new();
        let mut data = Vec::new();
        for (y, l) in input.lines().enumerate() {
            let mut res_line = Vec::new();
            for (x, c) in l.unwrap().chars().enumerate() {
                if needles.contains(&c) {
                    locations
                        .entry(c)
                        .or_default()
                        .insert((x as isize, y as isize));
                }
                res_line.push(c);
            }
            data.push(res_line)
        }
        let height = data.len() as isize;
        let width = data.get(0).unwrap().len() as isize;
        let size = (width, height);
        Self {
            data,
            locations,
            size,
        }
    }

    pub fn index_2d(&self, x: isize, y: isize) -> Option<char> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        self.data.get(y).and_then(|l| l.get(x)).copied()
    }

    pub fn is_inside(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1
    }
}
