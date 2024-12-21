// TODO: Optimize. Save uncheated distance of each path element to end. Find cheated length by finding differece of sufficiently close tiles.

use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{grid::V2d, AocInput, AocResult, Grid};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    coords: V2d,
    used_cheat: Option<V2d>,
}

impl Pos {
    fn get_neighbors(&self, grid: &Grid) -> Vec<Self> {
        let neighbors = self.coords.neighbors4();
        let mut res = Vec::with_capacity(neighbors.len());
        for n in neighbors {
            if let Some(c) = grid.index_2d(n.0, n.1) {
                if c == '#' {
                    if self.used_cheat.is_none() {
                        res.push(Self {
                            coords: n,
                            used_cheat: Some(n),
                        });
                    }
                } else {
                    res.push(Self {
                        coords: n,
                        used_cheat: self.used_cheat,
                    });
                }
            }
        }
        res
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CostPos {
    pos: Pos,
    cost: i64,
}

impl CostPos {
    fn get_neighbors(&self, grid: &Grid) -> Vec<Self> {
        self.pos
            .get_neighbors(grid)
            .into_iter()
            .map(|n| Self {
                pos: n,
                cost: self.cost + 1,
            })
            .collect()
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
    let mut grid = Grid::new(input, &['S', 'E', '.']);
    let start = *grid
        .locations
        .get(&'S')
        .and_then(|l| l.iter().next())
        .unwrap();
    let end = *grid
        .locations
        .get(&'E')
        .and_then(|l| l.iter().next())
        .unwrap();
    let baseline = grid.locations.get(&'.').map(|x| x.len()).unwrap() + 1;

    let mut shortest = BinaryHeap::new();
    let mut costs = HashMap::new();
    let baseline_start = Pos {
        coords: start,
        used_cheat: None,
    };
    shortest.push(CostPos {
        pos: baseline_start.clone(),
        cost: 0,
    });
    costs.insert(baseline_start, 0);
    let mut used_cheats = HashSet::new();

    let mut res1 = 0;
    while let Some(p) = shortest.pop() {
        if p.pos
            .used_cheat
            .map(|c| used_cheats.contains(&c))
            .unwrap_or(false)
        {
            continue;
        }
        if p.pos.coords == end {
            if let Some(cheat) = p.pos.used_cheat {
                used_cheats.insert(cheat);
            }
            if (p.cost as usize) <= baseline - 100 {
                res1 += 1;
                continue;
            } else {
                break;
            }
        }
        for n in p.get_neighbors(&grid) {
            let c = costs.entry(n.pos.clone()).or_insert(i64::MAX);
            if *c > n.cost {
                if n.pos
                    .used_cheat
                    .map(|c| used_cheats.contains(&c))
                    .unwrap_or(false)
                {
                    continue;
                }
                *c = n.cost;
                shortest.push(n);
            }
        }
    }

    println!("{}", baseline);

    res1.into()
}
