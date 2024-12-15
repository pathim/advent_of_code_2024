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

fn count_one_area(robots: &mut HashSet<V2d>, start: V2d) -> usize {
    let mut res = 0;
    if robots.remove(&start) {
        res = 1;
        for n in start.neighbors8() {
            res += count_one_area(robots, n);
        }
    }
    res
}

fn count_area(mut robots: HashSet<V2d>) -> Vec<usize> {
    let mut res = Vec::new();
    while let Some(current) = robots.iter().copied().next() {
        res.push(count_one_area(&mut robots, current.clone()));
    }
    res
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

    let mut max_area = 0;
    let mut max_n = 0;
    for n in 0..HEIGHT * WIDTH {
        let new_r = robots
            .iter()
            .map(|r| r.wrapped_pos_after(WIDTH, HEIGHT, n))
            .collect::<HashSet<_>>();
        let new_max = count_area(new_r).iter().max().unwrap().clone();
        if new_max > max_area {
            max_area = new_max;
            max_n = n;
        }
    }

    (res1, max_n).into()
}
