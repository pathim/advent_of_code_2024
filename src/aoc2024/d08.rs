use std::collections::HashSet;

use gcd::Gcd;

use crate::{AocInput, Grid};

pub fn f(input: AocInput) -> crate::AocResult {
    let stations = ('0'..='9')
        .chain('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<_>>();
    let mut targets1 = HashSet::new();
    let mut targets2 = HashSet::new();
    let grid = Grid::new(input, &stations);
    for station_pos in grid.locations.values() {
        for pos0 in station_pos {
            for pos1 in station_pos {
                if *pos0 == *pos1 {
                    continue;
                }
                let delta = pos1 - pos0;
                let delta2 = delta / (delta.0.unsigned_abs()).gcd(delta.1.unsigned_abs()) as isize;
                let target = *pos1 + delta;
                if grid.is_inside(target) {
                    targets1.insert(target);
                }
                let mut t2 = *pos1;
                while grid.is_inside(t2) {
                    targets2.insert(t2);
                    t2 = t2 + delta2;
                }
                t2 = *pos1;
                while grid.is_inside(t2) {
                    targets2.insert(t2);
                    t2 = t2 - delta2;
                }
            }
        }
    }
    let res1 = targets1.len();
    let res2 = targets2.len();
    (res1, res2).into()
}
