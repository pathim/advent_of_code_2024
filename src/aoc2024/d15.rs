use crate::{grid::V2d, AocInput, AocResult, Grid};

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

    fn is_vertical(&self) -> bool {
        match self {
            Dir::Up => true,
            Dir::Down => true,
            Dir::Left => false,
            Dir::Right => false,
        }
    }
}

fn can_move(grid: &Grid, pos: V2d, dir: Dir) -> bool {
    let new_pos = pos + dir.as_v2d();
    let new_pos_char = grid.index_2d(new_pos.0, new_pos.1).unwrap();
    match new_pos_char {
        '.' => true,
        '#' => false,
        'O' => can_move(grid, new_pos, dir),
        '[' => {
            if dir.is_vertical() {
                can_move(grid, new_pos, dir) && can_move(grid, new_pos + V2d(1, 0), dir)
            } else {
                can_move(grid, new_pos, dir)
            }
        }
        ']' => {
            if dir.is_vertical() {
                can_move(grid, new_pos, dir) && can_move(grid, new_pos + V2d(-1, 0), dir)
            } else {
                can_move(grid, new_pos, dir)
            }
        }
        _ => panic!("Invalid char"),
    }
}

fn do_move(grid: &mut Grid, pos: V2d, dir: Dir) {
    let new_pos = pos + dir.as_v2d();
    let new_pos_char = grid.index_2d(new_pos.0, new_pos.1).unwrap();
    let pos_char = grid.index_2d(pos.0, pos.1).unwrap();
    match new_pos_char {
        '.' => {}
        'O' => do_move(grid, new_pos, dir),
        '[' => {
            if dir.is_vertical() {
                do_move(grid, new_pos, dir);
                do_move(grid, new_pos + V2d(1, 0), dir);
            } else {
                do_move(grid, new_pos, dir);
            }
        }
        ']' => {
            if dir.is_vertical() {
                do_move(grid, new_pos, dir);
                do_move(grid, new_pos + V2d(-1, 0), dir);
            } else {
                do_move(grid, new_pos, dir);
            }
        }
        _ => panic!("Invalid char"),
    }
    *grid.index_2d_mut(new_pos.0, new_pos.1).unwrap() = pos_char;
    *grid.index_2d_mut(pos.0, pos.1).unwrap() = '.';
}

fn widen(c: char) -> [char; 2] {
    match c {
        '#' => ['#', '#'],
        '.' => ['.', '.'],
        '@' => ['@', '.'],
        'O' => ['[', ']'],
        _ => panic!("Invalid char"),
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut lines_iter = input.lines().map(|x| x.unwrap());
    let mut grid = Grid::new_empty();
    let mut grid2 = Grid::new_empty();

    for l in lines_iter.by_ref() {
        if l.is_empty() {
            break;
        }
        grid.add_line(&l, &['@']);
        grid2.add_line(&l.chars().flat_map(widen).collect::<String>(), &['@']);
    }

    let mut pos1 = *grid.locations.get(&'@').unwrap().iter().next().unwrap();
    let mut pos2 = *grid2.locations.get(&'@').unwrap().iter().next().unwrap();

    let mut dirs = Vec::new();
    for l in lines_iter {
        for c in l.chars().filter_map(Dir::new) {
            dirs.push(c);
        }
    }

    for dir in dirs {
        if can_move(&grid, pos1, dir) {
            do_move(&mut grid, pos1, dir);
            pos1 = pos1 + dir.as_v2d();
        }
        if can_move(&grid2, pos2, dir) {
            do_move(&mut grid2, pos2, dir);
            pos2 = pos2 + dir.as_v2d();
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
    let mut res2 = 0;
    for (y, l) in grid2.data.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '[' {
                res2 += 100 * y + x;
            }
        }
    }
    (res1, res2).into()
}
