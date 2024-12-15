use std::{collections::HashSet, str::FromStr};

use crate::{grid::V2d, AocInput, AocResult};

struct Robot {
    p: V2d,
    v: V2d,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').unwrap();
        let (_, p) = p.split_once('=').unwrap();
        let (p0, p1) = p.split_once(',').unwrap();
        let p0 = p0.parse().unwrap();
        let p1 = p1.parse().unwrap();
        let (_, v) = v.split_once('=').unwrap();
        let (v0, v1) = v.split_once(',').unwrap();
        let v0 = v0.parse().unwrap();
        let v1 = v1.parse().unwrap();
        let p = V2d(p0, p1);
        let v = V2d(v0, v1);
        Ok(Self { p, v })
    }
}

impl Robot {
    fn wrapped_pos_after(&self, width: isize, height: isize, n: isize) -> V2d {
        let p1 = self.p + self.v * n;
        V2d(p1.0.rem_euclid(width), p1.1.rem_euclid(height))
    }
}

fn get_quadrant(p: &V2d, width: isize, height: isize) -> Option<usize> {
    if p.0 == width / 2 || p.1 == height / 2 {
        return None;
    }
    Some(match (p.0 < width / 2, p.1 < height / 2) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    })
}

fn show_robots<'a, T: IntoIterator<Item = &'a V2d>>(robots: T, width: isize, height: isize) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for r in robots {
        let c = &mut grid[r.1 as usize][r.0 as usize];
        if *c == '.' {
            *c = '1';
        } else {
            *c = char::from_u32(*c as u32 + 1).unwrap();
        }
    }
    for l in grid {
        for c in l {
            print!("{}", c);
        }
        println!()
    }
}
pub fn f(input: AocInput) -> AocResult {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let robots = input
        .lines()
        .map(|s| Robot::from_str(&s.unwrap()).unwrap())
        .collect::<Vec<_>>();
    let mut quadrants = [0; 4];
    for pos in robots
        .iter()
        .map(|r| r.wrapped_pos_after(WIDTH, HEIGHT, 100))
    {
        if let Some(q) = get_quadrant(&pos, WIDTH, HEIGHT) {
            quadrants[q] += 1
        }
    }
    let res1 = quadrants.iter().fold(1, |a, b| a * b);

    let mut n = 3589;
    let res2 = loop {
        let mut quadrants = [0; 4];
        let mut middle = 0;
        let new_r = robots
            .iter()
            .map(|r| r.wrapped_pos_after(WIDTH, HEIGHT, n))
            .collect::<HashSet<_>>();
        for pos in new_r.iter() {
            if let Some(q) = get_quadrant(&pos, WIDTH, HEIGHT) {
                quadrants[q] += 1
            } else {
                middle += 1;
            }
        }

        if quadrants[0] == quadrants[2]
            && quadrants[1] == quadrants[3]
            && new_r.contains(&V2d(WIDTH / 2, 0))
        {
            show_robots(&new_r, WIDTH, HEIGHT);
            println!("{}", n);
            break n;
        }
        n += 1;
    };

    (res1, res2).into()
}
