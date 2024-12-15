use crate::{
    grid::{self, V2d},
    AocInput, AocResult, Grid,
};

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn new(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }

    fn as_v2d(&self) -> V2d {
        match self {
            Dir::Up => V2d(0, -1),
            Dir::Down => V2d(0, 1),
            Dir::Left => V2d(-1, 0),
            Dir::Right => V2d(1, 0),
        }
    }
}

fn move_on_grid(grid: &mut Grid, pos: V2d, dir: Dir) -> Option<V2d> {
    let new_pos = pos + dir.as_v2d();
    let pos_char = grid.index_2d(pos.0, pos.1).unwrap();
    let new_pos_char = grid.index_2d(new_pos.0, new_pos.1).unwrap();
    if new_pos_char == '.' {
        *grid.index_2d_mut(new_pos.0, new_pos.1).unwrap() = pos_char;
        *grid.index_2d_mut(pos.0, pos.1).unwrap() = '.';
        Some(new_pos)
    } else if new_pos_char == '#' {
        None
    } else {
        if let Some(p) = move_on_grid(grid, new_pos, dir) {
            *grid.index_2d_mut(new_pos.0, new_pos.1).unwrap() = pos_char;
            *grid.index_2d_mut(pos.0, pos.1).unwrap() = '.';
            Some(new_pos)
        } else {
            None
        }
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut lines_iter = input.lines().map(|x| x.unwrap());
    let mut grid = Grid::new_empty();

    while let Some(l) = lines_iter.next() {
        if l.is_empty() {
            break;
        }
        grid.add_line(&l, &['@']);
    }

    let mut pos = grid
        .locations
        .get(&'@')
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone();
    *grid.index_2d_mut(pos.0, pos.1).unwrap() = '.';

    let mut dirs = Vec::new();
    for l in lines_iter {
        for c in l.chars().filter_map(Dir::new) {
            dirs.push(c);
        }
    }

    for d in dirs {
        if let Some(new_pos) = move_on_grid(&mut grid, pos, d) {
            pos = new_pos;
        }
    }
    let mut res1 = 0;
    for (y, l) in grid.data.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == 'O' {
                res1 += 100 * y + x;
            }
        }
    }
    res1.into()
}
