use std::collections::HashSet;

use crate::{AocInput, Grid};

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res2 = 0;
    let stations = ('0'..='9')
        .chain('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<_>>();
    let mut targets = HashSet::new();
    let grid = Grid::new(input, &stations);
    for (_, station_pos) in &grid.locations {
        for pos0 in station_pos {
            for pos1 in station_pos {
                if *pos0 == *pos1 {
                    continue;
                }
                let delta = pos1 - pos0;
                let target = *pos1 + delta;
                if grid.is_inside(target) {
                    targets.insert(target);
                }
            }
        }
    }
    let res1 = targets.len();
    (res1, res2).into()
}
