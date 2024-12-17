use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{grid::V2d, AocInput, AocResult, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn cw(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    fn as_v2d(&self) -> V2d {
        match self {
            Self::Up => V2d(0, -1),
            Self::Down => V2d(0, 1),
            Self::Left => V2d(-1, 0),
            Self::Right => V2d(1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MazePos {
    pos: V2d,
    dir: Dir,
}
impl MazePos {
    fn neighbors(&self) -> [(Self, i64); 3] {
        [
            (
                Self {
                    pos: self.pos + self.dir.as_v2d(),
                    dir: self.dir,
                },
                1,
            ),
            (
                Self {
                    pos: self.pos,
                    dir: self.dir.cw(),
                },
                1000,
            ),
            (
                Self {
                    pos: self.pos,
                    dir: self.dir.ccw(),
                },
                1000,
            ),
        ]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CostPos {
    pos: MazePos,
    cost: i64,
    path: Vec<V2d>,
}

impl CostPos {
    fn neighbors(&self) -> [Self; 3] {
        let n = self.pos.neighbors();
        std::array::from_fn(|i| Self {
            pos: n[i].0,
            cost: self.cost + n[i].1,
            path: self.path.clone(),
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

pub fn f(input: AocInput) -> AocResult {
    let grid = Grid::new(input, &['S', 'E']);
    let mut lowest_costs = BinaryHeap::new();
    let mut costs = HashMap::new();
    let start = *grid.locations.get(&'S').unwrap().iter().next().unwrap();
    let start = MazePos {
        pos: start,
        dir: Dir::Right,
    };
    let end = *grid.locations.get(&'E').unwrap().iter().next().unwrap();
    let startpos = CostPos {
        pos: start,
        cost: 0,
        path: vec![start.pos],
    };
    costs.insert(start, 0);
    lowest_costs.push(startpos);
    let mut shortest_paths = Vec::new();
    let mut min_cost = i64::MAX;
    let res1 = loop {
        let current = lowest_costs.pop().unwrap();
        if current.cost > min_cost {
            break min_cost;
        }
        if current.pos.pos == end {
            min_cost = current.cost;
            shortest_paths.push(current.path);
            continue;
        }
        for mut n in current.neighbors() {
            if grid.index_2d(n.pos.pos.0, n.pos.pos.1).unwrap() == '#' {
                continue;
            }
            let c = costs.entry(n.pos).or_insert(i64::MAX);
            if n.cost <= *c {
                *c = n.cost;
                n.path.push(n.pos.pos);
                lowest_costs.push(n);
            }
        }
    };
    let mut path_tiles = HashSet::new();
    for p in shortest_paths {
        path_tiles.extend(p.into_iter());
    }
    let res2 = path_tiles.len();
    (res1, res2).into()
}
