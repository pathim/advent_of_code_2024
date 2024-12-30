use std::collections::{BinaryHeap, HashMap};

use crate::{grid::V2d, AocInput, AocResult};

fn create_dir_keypad() -> (HashMap<V2d, char>, HashMap<char, V2d>) {
    let mut res = HashMap::new();
    let mut res_inv = HashMap::new();

    res.insert(V2d(1, 0), '^');
    res.insert(V2d(2, 0), 'A');
    res.insert(V2d(0, 1), '<');
    res.insert(V2d(1, 1), 'v');
    res.insert(V2d(2, 1), '>');

    for (k, v) in &res {
        res_inv.insert(*v, *k);
    }

    (res, res_inv)
}

fn create_num_keypad() -> (HashMap<V2d, char>, HashMap<char, V2d>) {
    let mut res = HashMap::new();
    let mut res_inv = HashMap::new();

    res.insert(V2d(0, 0), '7');
    res.insert(V2d(1, 0), '8');
    res.insert(V2d(2, 0), '9');
    res.insert(V2d(0, 1), '4');
    res.insert(V2d(1, 1), '5');
    res.insert(V2d(2, 1), '6');
    res.insert(V2d(0, 2), '1');
    res.insert(V2d(1, 2), '2');
    res.insert(V2d(2, 2), '3');
    res.insert(V2d(1, 3), '0');
    res.insert(V2d(2, 3), 'A');

    for (k, v) in &res {
        res_inv.insert(*v, *k);
    }

    (res, res_inv)
}

#[derive(Debug, PartialEq, Eq)]
struct CostPos {
    pos: V2d,
    cost: u64,
    path: Vec<char>,
}

impl PartialOrd for CostPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CostPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn delta2dir(delta: V2d) -> char {
    match delta {
        V2d(0, 1) => 'v',
        V2d(0, -1) => '^',
        V2d(1, 0) => '>',
        V2d(-1, 0) => '<',
        _ => panic!("invalid direction"),
    }
}

fn push_direction(
    src: char,
    dst: char,
    depth: u8,
    cache: &mut HashMap<(char, char, u8), u64>,
    keypad: &dyn Fn() -> (HashMap<V2d, char>, HashMap<char, V2d>),
) -> u64 {
    if depth == 0 {
        return 1;
    }
    let k = (src, dst, depth);
    if let Some(r) = cache.get(&k) {
        return *r;
    }
    let (pos2key, key2pos) = keypad();
    let mut paths = HashMap::new();
    let mut shortest_path = BinaryHeap::new();
    let pos = *key2pos.get(&src).unwrap();
    let target = *key2pos.get(&dst).unwrap();
    shortest_path.push(CostPos {
        pos,
        cost: 0,
        path: vec!['A'],
    });
    while let Some(p) = shortest_path.pop() {
        if p.pos == V2d(-1, -1) {
            cache.insert(k, p.cost);
            return p.cost;
        }
        if p.pos == target {
            let actual_cost = p.cost
                + push_direction(
                    *p.path.last().unwrap(),
                    'A',
                    depth - 1,
                    cache,
                    &create_dir_keypad,
                );
            let mut last_path = p.path;
            last_path.push('A');
            let last_item = CostPos {
                cost: actual_cost,
                pos: V2d(-1, -1),
                path: last_path,
            };
            shortest_path.push(last_item);
        } else {
            for n in V2d(0, 0).neighbors4() {
                let new_pos = p.pos + n;
                if pos2key.contains_key(&new_pos) {
                    let new_cost = p.cost
                        + push_direction(
                            *p.path.last().unwrap(),
                            delta2dir(n),
                            depth - 1,
                            cache,
                            &create_dir_keypad,
                        );
                    let e = paths.entry((p.path.clone(), new_pos)).or_insert(u64::MAX);
                    if new_cost < *e {
                        *e = new_cost;
                        let mut path = p.path.clone();
                        path.push(delta2dir(n));
                        shortest_path.push(CostPos {
                            pos: new_pos,
                            cost: new_cost,
                            path,
                        });
                    }
                }
            }
        }
    }
    panic!("No path found")
}

pub fn f(input: AocInput) -> AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    let mut cache = HashMap::new();

    for l in input.lines() {
        let l = l.unwrap();
        let mut min_presses1 = 0;
        let mut min_presses2 = 0;
        let mut prev = 'A';
        let mut val = 0;
        for c in l.chars() {
            min_presses1 += push_direction(prev, c, 3, &mut cache, &create_num_keypad);
            min_presses2 += push_direction(prev, c, 26, &mut cache, &create_num_keypad);

            prev = c;
            if c != 'A' {
                val *= 10;
                val += c.to_digit(10).unwrap() as u64;
            }
        }
        res1 += val * min_presses1;
        res2 += val * min_presses2;
    }
    (res1, res2).into()
}
