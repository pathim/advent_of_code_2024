use std::collections::HashSet;

use crate::{grid::V2d, AocInput, Grid};

fn count_trails(grid: &Grid, start: V2d) -> (u64, HashSet<V2d>) {
    let start_val = grid.index_2d(start.0, start.1).unwrap();
    if start_val == '9' {
        return (1, [start].iter().copied().collect());
    }
    let start_val = start_val.to_digit(10).unwrap();
    let mut res = (0, HashSet::new());
    for d in [V2d(-1, 0), V2d(1, 0), V2d(0, -1), V2d(0, 1)] {
        let new_pos = start + d;
        if let Some(val) = grid
            .index_2d(new_pos.0, new_pos.1)
            .map(|x| x.to_digit(10).unwrap())
        {
            if val.saturating_sub(start_val) == 1 {
                let (count, set) = count_trails(grid, new_pos);
                res.0 += count;
                res.1.extend(set);
            }
        }
    }
    res
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    let grid = Grid::new(input, &['0']);

    for l in grid.locations.get(&'0').unwrap() {
        let (count, peaks) = count_trails(&grid, *l);
        res1 += peaks.len();
        res2 += count;
    }

    (res1, res2).into()
}
