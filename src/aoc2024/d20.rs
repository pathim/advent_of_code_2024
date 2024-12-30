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

fn dist_manhattan(p1: V2d, p2: V2d) -> isize {
    let delta = p1 - p2;
    delta.0.abs() + delta.1.abs()
}

fn find_cheats(grid: &Grid, distances: &HashMap<V2d, isize>, dist: isize) -> usize {
    let mut cheats = HashSet::new();

    for (cheat_start, d) in distances {
        for cheat_end in neighbors_manhattan(*cheat_start, dist) {
            if !grid.is_char(cheat_end, '.') {
                continue;
            }
            let delta = d - distances.get(&cheat_end).unwrap();
            if delta - dist_manhattan(*cheat_start, cheat_end) >= 100 {
                cheats.insert((cheat_start, cheat_end));
            }
        }
    }
    cheats.len()
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
            if !grid.is_char(n, '#') && !distances.contains_key(&n) {
                current = n;
                break;
            }
        }
    }

    let res1 = find_cheats(&grid, &distances, 2);

    let res2 = find_cheats(&grid, &distances, 20);

    (res1, res2).into()
}
