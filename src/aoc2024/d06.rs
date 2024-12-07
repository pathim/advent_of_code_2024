use std::collections::HashSet;

use crate::AocInput;

fn within<T: Ord>(val: T, lower: T, upper: T) -> bool {
    val >= lower && val < upper
}

fn pos_within<T: Ord>(val: (T, T), lower: (T, T), upper: (T, T)) -> bool {
    within(val.0, lower.0, upper.0) && within(val.1, lower.1, upper.1)
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res2 = 0;
    let (grid, mut found) = input.to_2d_array_finding_chars(&['#', '^']);
    let height = grid.len();
    let width = grid.first().unwrap().len();
    let size = (height as isize, width as isize);
    let start = found.get(&'^').unwrap().iter().next().copied().unwrap();
    let obstacles = found.get_mut(&'#').unwrap();
    let mut dir = (0, -1);
    let mut visited = HashSet::new();
    let mut pos = start;
    visited.insert(pos);
    while pos_within(pos, (0, 0), size) {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if obstacles.contains(&new_pos) {
            dir = (-dir.1, dir.0);
            continue;
        }
        pos = new_pos;
        visited.insert(pos);
    }
    visited.remove(&pos);
    let res1 = visited.len();
    visited.remove(&start);

    for new_obstacle_pos in visited {
        pos = start;
        dir = (0, -1);
        let mut visited2 = HashSet::new();
        visited2.insert((pos, dir));
        obstacles.insert(new_obstacle_pos);
        while pos_within(pos, (0, 0), size) {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if obstacles.contains(&new_pos) {
                dir = (-dir.1, dir.0);
                continue;
            }
            pos = new_pos;

            if !visited2.insert((pos, dir)) {
                res2 += 1;
                break;
            }
        }
        obstacles.remove(&new_obstacle_pos);
    }

    (res1, res2).into()
}
