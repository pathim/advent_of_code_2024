use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::AocInput;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct V2d(pub isize, pub isize);

impl V2d {
    pub fn is_zero(self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub fn neighbors4(&self) -> [Self; 4] {
        [
            Self(self.0 - 1, self.1),
            Self(self.0 + 1, self.1),
            Self(self.0, self.1 - 1),
            Self(self.0, self.1 + 1),
        ]
    }
    pub fn neighbors8(&self) -> [Self; 8] {
        [
            self + &Self(-1, -1),
            self + &Self(-1, 0),
            self + &Self(-1, 1),
            self + &Self(0, -1),
            self + &Self(0, 1),
            self + &Self(1, -1),
            self + &Self(1, 0),
            self + &Self(1, 1),
        ]
    }
}

impl Add for V2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for V2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Div<isize> for V2d {
    type Output = V2d;

    fn div(self, rhs: isize) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
impl Mul<isize> for V2d {
    type Output = V2d;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Rem<isize> for V2d {
    type Output = Self;

    fn rem(self, rhs: isize) -> Self::Output {
        Self(self.0 % rhs, self.1 / rhs)
    }
}

impl Add for &V2d {
    type Output = V2d;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}
impl Sub for &V2d {
    type Output = V2d;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}
impl Div<isize> for &V2d {
    type Output = V2d;

    fn div(self, rhs: isize) -> Self::Output {
        *self / rhs
    }
}
impl Mul<isize> for &V2d {
    type Output = V2d;

    fn mul(self, rhs: isize) -> Self::Output {
        *self * rhs
    }
}
impl Rem<isize> for &V2d {
    type Output = V2d;

    fn rem(self, rhs: isize) -> Self::Output {
        *self % rhs
    }
}

pub type Position = V2d;
pub type Positions = HashSet<Position>;

#[derive(Clone, Debug)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub locations: HashMap<char, Positions>,
    pub size: (isize, isize),
    pub overlay: HashMap<char, Positions>,
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
                        .insert(V2d(x as isize, y as isize));
                }
                res_line.push(c);
            }
            data.push(res_line)
        }
        let height = data.len() as isize;
        let width = data.first().unwrap().len() as isize;
        let size = (width, height);
        Self {
            data,
            locations,
            size,
            overlay: HashMap::new(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            data: Vec::new(),
            locations: HashMap::new(),
            size: (0, 0),
            overlay: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &str, needles: &[char]) {
        let y = self.data.len();
        let mut res_line = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if needles.contains(&c) {
                self.locations
                    .entry(c)
                    .or_default()
                    .insert(V2d(x as isize, y as isize));
            }
            res_line.push(c);
        }
        self.data.push(res_line);
        let height = self.data.len() as isize;
        let width = self.data.first().unwrap().len() as isize;
        self.size = (width, height);
    }

    pub fn index_2d(&self, x: isize, y: isize) -> Option<char> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        self.data.get(y).and_then(|l| l.get(x)).copied()
    }
    pub fn index_2d_mut(&mut self, x: isize, y: isize) -> Option<&mut char> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        self.data.get_mut(y).and_then(|l| l.get_mut(x))
    }

    pub fn is_inside(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1
    }

    pub fn get(&self, pos: V2d) -> Option<char> {
        self.index_2d(pos.0, pos.1)
    }
    pub fn get_mut(&mut self, pos: V2d) -> Option<&mut char> {
        self.index_2d_mut(pos.0, pos.1)
    }

    pub fn is_char(&self, pos: V2d, c: char) -> bool {
        self.get(pos).map(|x| x == c).unwrap_or(false)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = if self.overlay.is_empty() {
            None
        } else {
            let mut grid_copy = self.clone();
            for (c, poss) in self.overlay.iter() {
                for pos in poss {
                    if let Some(x) = grid_copy.index_2d_mut(pos.0, pos.1) {
                        *x = *c;
                    }
                }
            }
            Some(grid_copy.data)
        };
        let vis_data = data.as_ref().unwrap_or(&self.data);
        for line in vis_data {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}
