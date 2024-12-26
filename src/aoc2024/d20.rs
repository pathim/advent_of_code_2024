use std::collections::{HashMap, HashSet};

use crate::{grid::V2d, AocInput, AocResult, Grid};

fn neighbors_manhattan(pos: V2d, dist: isize) -> Vec<V2d> {
    let mut res = Vec::new();
    for x in -dist..=dist {
        for y in -(dist - x.abs())..=dist - x.abs() {
            res.push(V2d(x, y) + pos);
        }
    }
    res
}

pub fn f(input: AocInput) -> AocResult {
    let mut grid = Grid::new(input, &['S', 'E']);
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
    *grid.get_mut(end).unwrap() = '.';
    let mut distances = HashMap::new();
    let mut current = end;
    let mut current_dist = 0;
    loop {
        distances.insert(current, current_dist);
        if current == start {
            break;
        }
        current_dist += 1;
        for n in current.neighbors4() {
            if !grid.is_char(n, '#') {
                if !distances.contains_key(&n) {
                    current = n;
                    break;
                }
            }
        }
    }

    let mut cheats = HashSet::new();

    for (cheat_start, d) in &distances {
        for cheat_end in neighbors_manhattan(*cheat_start, 2) {
            if !grid.is_char(cheat_end, '.') {
                continue;
            }
            let delta = d - distances.get(&cheat_end).unwrap();
            if delta > 100 {
                cheats.insert((cheat_start, cheat_end));
            }
        }
    }

    let res1 = cheats.len();

    res1.into()
}
