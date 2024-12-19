use std::collections::{BinaryHeap, HashMap};

use crate::{grid::V2d, AocInput, AocResult};

const SIZE: isize = 71;

#[derive(Debug, PartialEq, Eq)]
struct CostPos {
    pos: V2d,
    cost: i64,
}

impl CostPos {
    fn neighbors(&self) -> [Self; 4] {
        let n = self.pos.neighbors4();
        std::array::from_fn(|i| Self {
            pos: n[i],
            cost: self.cost + 1,
        })
    }
}

impl PartialOrd for CostPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CostPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn is_inside(pos: V2d) -> bool {
    pos.0 >= 0 && pos.1 < SIZE && pos.1 >= 0 && pos.0 < SIZE
}

fn find_path(obstacles: &HashMap<V2d, usize>, limit: usize) -> Option<i64> {
    let start = V2d(0, 0);
    let target = V2d(70, 70);

    let mut shortest = BinaryHeap::new();
    let mut costs = HashMap::new();
    shortest.push(CostPos {
        pos: start,
        cost: 0,
    });
    while let Some(pos) = shortest.pop() {
        if pos.pos == target {
            return Some(pos.cost);
        }
        for n in pos.neighbors().into_iter().filter(|p| {
            is_inside(p.pos) && obstacles.get(&p.pos).copied().unwrap_or(usize::MAX) > limit
        }) {
            let c = costs.entry(n.pos).or_insert(i64::MAX);
            if *c > n.cost {
                *c = n.cost;
                shortest.push(n);
            }
        }
    }
    None
}

pub fn f(input: AocInput) -> AocResult {
    let mut obstacles = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let l = l.unwrap();
        let (x, y) = l.split_once(',').unwrap();
        let x: isize = x.parse().unwrap();
        let y: isize = y.parse().unwrap();
        obstacles.insert(V2d(x, y), i + 1);
    }

    let res1 = find_path(&obstacles, 1024).unwrap();
    let mut high = obstacles.len();
    let mut low = 1025;
    while high != low + 1 {
        let limit = (high + low) / 2;
        if find_path(&obstacles, limit).is_none() {
            high = limit;
        } else {
            low = limit;
        }
    }
    let res2 = *obstacles.iter().find(|(_, v)| **v == high).unwrap().0;
    let res2 = format!("{},{}", res2.0, res2.1);
    (res1, res2).into()
}
