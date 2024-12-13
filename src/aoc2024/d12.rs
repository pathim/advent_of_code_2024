use std::{collections::HashSet, fmt::Debug};

use crate::{grid::V2d, AocInput, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Edge {
    H(V2d, isize),
    V(V2d, isize),
}

impl Edge {
    fn new(a: V2d, b: V2d) -> Option<Self> {
        let delta = a - b;
        if delta.0 == 0 {
            Some(Self::H(V2d(a.0, a.1.max(b.1)), a.1))
        } else if delta.1 == 0 {
            Some(Self::V(V2d(a.0.max(b.0), a.1), a.0))
        } else {
            None
        }
    }

    fn neighbors(&self) -> [Self; 2] {
        match self {
            Self::H(v2d, inside) => [
                Self::H(*v2d - V2d(1, 0), *inside),
                Self::H(*v2d + V2d(1, 0), *inside),
            ],
            Self::V(v2d, inside) => [
                Self::V(*v2d - V2d(0, 1), *inside),
                Self::V(*v2d + V2d(0, 1), *inside),
            ],
        }
    }
}

fn fill_group(locs: &mut HashSet<V2d>, start: V2d) -> HashSet<V2d> {
    let mut res = HashSet::new();
    if locs.remove(&start) {
        res.insert(start);
        for n in start.neighbors4() {
            res = res.union(&fill_group(locs, n)).copied().collect();
        }
    }
    res
}

fn find_groups(mut locs: HashSet<V2d>) -> Vec<HashSet<V2d>> {
    let mut res = Vec::new();
    while let Some(l) = locs.iter().next().copied() {
        res.push(fill_group(&mut locs, l));
    }
    res
}

fn count_perimeter(group: &HashSet<V2d>) -> (usize, HashSet<Edge>) {
    let mut edges = HashSet::new();
    let perimeter = group
        .iter()
        .map(|a| {
            let mut res = 0;
            for n in a.neighbors4() {
                if !group.contains(&n) {
                    res += 1;
                    edges.insert(Edge::new(*a, n).unwrap());
                }
            }
            res
        })
        .sum();
    (perimeter, edges)
}

fn remove_side_edges(edges: &mut HashSet<Edge>, start: &Edge) {
    if edges.remove(start) {
        for n in start.neighbors() {
            remove_side_edges(edges, &n);
        }
    }
}

fn count_sides(mut edges: HashSet<Edge>) -> usize {
    let mut res = 0;
    while let Some(e) = edges.iter().copied().next() {
        remove_side_edges(&mut edges, &e);
        res += 1;
    }
    res
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    let grid = Grid::new(input, &('A'..='Z').collect::<Vec<_>>());
    let l = grid.locations;
    let mut groups = Vec::new();
    for (_, x) in l {
        groups.append(&mut find_groups(x));
    }
    for g in groups {
        let (perimeter, edges) = count_perimeter(&g);
        res1 += g.len() * perimeter;
        res2 += g.len() * count_sides(edges);
    }
    (res1, res2).into()
}
