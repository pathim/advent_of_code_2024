use std::collections::HashSet;

use crate::{
    grid::{Positions, V2d},
    AocInput, Grid,
};

fn within<T: Ord>(val: T, lower: T, upper: T) -> bool {
    val >= lower && val < upper
}

fn pos_within<T: Ord>(val: (T, T), lower: (T, T), upper: (T, T)) -> bool {
    within(val.0, lower.0, upper.0) && within(val.1, lower.1, upper.1)
}

fn check_cycle(
    obstacles: &Positions,
    grid: &Grid,
    mut pos: V2d,
    mut dir: V2d,
    visited_so_far: &HashSet<(V2d, V2d)>,
) -> bool {
    let mut visited = HashSet::new();
    visited.insert((pos, dir));
    while grid.is_inside(pos) {
        let new_pos = V2d(pos.0 + dir.0, pos.1 + dir.1);
        if obstacles.contains(&new_pos) {
            dir = V2d(-dir.1, dir.0);
            continue;
        }
        pos = new_pos;
        let pd = (pos, dir);
        if visited_so_far.contains(&pd) || !visited.insert(pd) {
            return true;
        }
    }
    false
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res2 = 0;
    let grid = Grid::new(input, &['#', '^']);
    let start = grid
        .locations
        .get(&'^')
        .unwrap()
        .iter()
        .next()
        .copied()
        .unwrap();
    let mut obstacles = grid.locations.get(&'#').unwrap().clone();
    let mut dir = V2d(0, -1);
    let mut visited = HashSet::new();
    let mut visited_with_dir = HashSet::new();
    let mut used_obstacles = HashSet::new();
    let mut pos = start;
    visited.insert(pos);
    visited_with_dir.insert((pos, dir));

    while grid.is_inside(pos) {
        let new_pos = V2d(pos.0 + dir.0, pos.1 + dir.1);
        if obstacles.contains(&new_pos) {
            dir = V2d(-dir.1, dir.0);
            continue;
        }
        if !used_obstacles.contains(&new_pos) {
            used_obstacles.insert(new_pos);
            obstacles.insert(new_pos);
            if check_cycle(&obstacles, &grid, pos, dir, &visited_with_dir) {
                res2 += 1;
            }
            obstacles.remove(&new_pos);
        }
        pos = new_pos;
        visited.insert(pos);
        visited_with_dir.insert((pos, dir));
    }
    visited.remove(&pos);
    let res1 = visited.len();

    (res1, res2).into()
}
