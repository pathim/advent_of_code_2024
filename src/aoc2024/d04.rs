use crate::AocInput;

fn index_2d(grid: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    let y: usize = y.try_into().ok()?;
    let x: usize = x.try_into().ok()?;
    grid.get(y).and_then(|l| l.get(x)).copied()
}

fn find_dir(grid: &[Vec<char>], x: isize, y: isize) -> Vec<(isize, isize)> {
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
            if index_2d(grid, new_x, new_y)
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
    let (grid, xs) = input.to_2d_array_finding_chars(&['X', 'A']);
    for (x, y) in xs.get(&'X').unwrap() {
        let x = *x as isize;
        let y = *y as isize;
        let dirs = find_dir(&grid, x, y);
        for (dx, dy) in dirs {
            if index_2d(&grid, x + 2 * dx, y + 2 * dy)
                .map(|x| x == 'A')
                .unwrap_or(false)
                && index_2d(&grid, x + 3 * dx, y + 3 * dy)
                    .map(|x| x == 'S')
                    .unwrap_or(false)
            {
                res1 += 1;
            }
        }
    }

    let mut res2 = 0;

    for (x, y) in xs.get(&'A').unwrap() {
        let x = *x as isize;
        let y = *y as isize;
        let mut found = false;
        for dx in [-1, 1] {
            if let Some(c1) = index_2d(&grid, x + dx, y + dx.abs()) {
                if let Some(c2) = index_2d(&grid, x - dx, y - dx.abs()) {
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
