use std::collections::HashSet;

use crate::{grid::V2d, AocInput, Grid};

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

fn count_perimeter(group: &HashSet<V2d>) -> usize {
    group
        .iter()
        .map(|a| {
            a.neighbors4()
                .iter()
                .filter(|x| !group.contains(&x))
                .count()
        })
        .sum()
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
        res1 += g.len() * count_perimeter(&g);
    }
    (res1, res2).into()
}
