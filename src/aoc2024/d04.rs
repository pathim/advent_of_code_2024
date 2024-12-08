use crate::{AocInput, Grid};

fn find_dir(grid: &Grid, x: isize, y: isize) -> Vec<(isize, isize)> {
    let mut res = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let new_x = x + dx;
            if new_x < 0 {
                continue;
            }
            let new_y = y + dy;
            if new_y < 0 {
                continue;
            }
            if grid
                .index_2d(new_x, new_y)
                .map(|x| x == 'M')
                .unwrap_or(false)
            {
                res.push((dx, dy));
            }
        }
    }
    res
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let grid = Grid::new(input, &['X', 'A']);
    for loc in grid.locations.get(&'X').unwrap().iter().copied() {
        let x = loc.0;
        let y = loc.1;
        let dirs = find_dir(&grid, x, y);
        for (dx, dy) in dirs {
            if grid
                .index_2d(x + 2 * dx, y + 2 * dy)
                .map(|x| x == 'A')
                .unwrap_or(false)
                && grid
                    .index_2d(x + 3 * dx, y + 3 * dy)
                    .map(|x| x == 'S')
                    .unwrap_or(false)
            {
                res1 += 1;
            }
        }
    }

    let mut res2 = 0;

    for loc in grid.locations.get(&'A').unwrap().iter().copied() {
        let x = loc.0;
        let y = loc.1;
        let mut found = false;
        for dx in [-1, 1] {
            if let Some(c1) = grid.index_2d(x + dx, y + dx.abs()) {
                if let Some(c2) = grid.index_2d(x - dx, y - dx.abs()) {
                    if (c1, c2) == ('M', 'S') || (c1, c2) == ('S', 'M') {
                        if found {
                            res2 += 1;
                        } else {
                            found = true;
                        }
                    }
                }
            }
        }
    }
    (res1, res2).into()
}
