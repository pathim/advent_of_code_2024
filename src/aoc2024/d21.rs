use std::collections::HashMap;

use crate::{AocInput, AocResult};

fn create_dirmap() -> HashMap<(char, char), Vec<char>> {
    let mut base = HashMap::new();
    base.insert(('A', '<'), vec!['v', '<', '<', 'A']);
    base.insert(('A', '>'), vec!['v', 'A']);
    base.insert(('A', '^'), vec!['<', 'A']);
    base.insert(('A', 'v'), vec!['<', 'v', 'A']);

    base.insert(('^', 'v'), vec!['v', 'A']);
    base.insert(('^', '<'), vec!['v', '<', 'A']);
    base.insert(('^', '>'), vec!['v', '>', 'A']);

    base.insert(('v', '<'), vec!['<', 'A']);
    base.insert(('v', '>'), vec!['>', 'A']);

    base.insert(('<', '>'), vec!['>', '>', 'A']);

    let mut res = HashMap::new();
    for ((p, c), v) in base {
        let rev = v
            .iter()
            .rev()
            .skip(1)
            .map(|c| match c {
                '<' => '>',
                '>' => '<',
                '^' => 'v',
                'v' => '^',
                _ => panic!("Invalid direction: {}", c),
            })
            .chain(['A'])
            .collect();
        res.insert((c, p), rev);
        res.insert((p, c), v);
    }
    for c in ['A', '<', '>', '^', 'v'] {
        res.insert((c, c), vec!['A']);
    }
    res
}

fn create_num_dirmap() -> HashMap<(char, char), Vec<char>> {
    let mut base = HashMap::new();
    base.insert(('A', '0'), vec!['<', 'A']);
    base.insert(('A', '1'), vec!['^', '<', '<', 'A']);
    base.insert(('A', '2'), vec!['<', '^', 'A']);
    base.insert(('A', '3'), vec!['^', 'A']);
    base.insert(('A', '4'), vec!['^', '^', '<', '<', 'A']);
    base.insert(('A', '5'), vec!['<', '^', '^', 'A']);
    base.insert(('A', '6'), vec!['^', '^', 'A']);
    base.insert(('A', '7'), vec!['^', '^', '^', '<', '<', 'A']);
    base.insert(('A', '8'), vec!['<', '^', '^', '^', 'A']);
    base.insert(('A', '9'), vec!['^', '^', '^', 'A']);

    base.insert(('0', '1'), vec!['^', '<', 'A']);
    base.insert(('0', '2'), vec!['^', 'A']);
    base.insert(('0', '3'), vec!['^', '>', 'A']);
    base.insert(('0', '4'), vec!['^', '^', '<', 'A']);
    base.insert(('0', '5'), vec!['^', '^', 'A']);
    base.insert(('0', '6'), vec!['^', '^', '>', 'A']);
    base.insert(('0', '7'), vec!['^', '^', '^', '<', 'A']);
    base.insert(('0', '8'), vec!['^', '^', '^', 'A']);
    base.insert(('0', '9'), vec!['^', '^', '^', '>', 'A']);

    base.insert(('1', '2'), vec!['>', 'A']);
    base.insert(('1', '3'), vec!['>', '>', 'A']);
    base.insert(('1', '4'), vec!['^', 'A']);
    base.insert(('1', '5'), vec!['>', '^', 'A']);
    base.insert(('1', '6'), vec!['>', '>', '^', 'A']);
    base.insert(('1', '7'), vec!['^', '^', 'A']);
    base.insert(('1', '8'), vec!['>', '^', '^', 'A']);
    base.insert(('1', '9'), vec!['>', '>', '^', '^', 'A']);

    base.insert(('2', '3'), vec!['>', 'A']);
    base.insert(('2', '4'), vec!['<', '^', 'A']);
    base.insert(('2', '5'), vec!['^', 'A']);
    base.insert(('2', '6'), vec!['>', '^', 'A']);
    base.insert(('2', '7'), vec!['<', '^', '^', 'A']);
    base.insert(('2', '8'), vec!['^', '^', 'A']);
    base.insert(('2', '9'), vec!['>', '^', '^', 'A']);

    base.insert(('3', '4'), vec!['<', '<', '^', 'A']);
    base.insert(('3', '5'), vec!['<', '^', 'A']);
    base.insert(('3', '6'), vec!['^', 'A']);
    base.insert(('3', '7'), vec!['<', '<', '^', '^', 'A']);
    base.insert(('3', '8'), vec!['<', '^', '^', 'A']);
    base.insert(('3', '9'), vec!['^', '^', 'A']);

    base.insert(('4', '5'), vec!['>', 'A']);
    base.insert(('4', '6'), vec!['>', '>', 'A']);
    base.insert(('4', '7'), vec!['^', 'A']);
    base.insert(('4', '8'), vec!['>', '^', 'A']);
    base.insert(('4', '9'), vec!['>', '>', '^', 'A']);

    base.insert(('5', '6'), vec!['>', 'A']);
    base.insert(('5', '7'), vec!['<', '^', 'A']);
    base.insert(('5', '8'), vec!['^', 'A']);
    base.insert(('5', '9'), vec!['>', '^', 'A']);

    base.insert(('6', '7'), vec!['<', '<', '^', 'A']);
    base.insert(('6', '8'), vec!['<', '^', 'A']);
    base.insert(('6', '9'), vec!['^', 'A']);

    base.insert(('7', '8'), vec!['>', 'A']);
    base.insert(('7', '9'), vec!['>', '>', 'A']);

    base.insert(('8', '9'), vec!['>', 'A']);

    let mut res = HashMap::new();
    for ((p, c), v) in base {
        let rev = v
            .iter()
            .rev()
            .skip(1)
            .map(|c| match c {
                '<' => '>',
                '>' => '<',
                '^' => 'v',
                'v' => '^',
                _ => panic!("Invalid direction: {}", c),
            })
            .chain(['A'])
            .collect();
        res.insert((c, p), rev);
        res.insert((p, c), v);
    }
    for c in '0'..='9' {
        res.insert((c, c), vec!['A']);
    }
    res.insert(('A', 'A'), vec!['A']);
    res
}

fn push_direction(
    src: char,
    dst: char,
    depth: u8,
    dirmap: &HashMap<(char, char), Vec<char>>,
    cache: &mut HashMap<(char, char, u8), u64>,
) -> u64 {
    if depth == 0 {
        return 1;
    }
    let k = (src, dst, depth);
    if let Some(r) = cache.get(&k) {
        return *r;
    }
    let mut prev = 'A';
    let mut res = 0;
    for c in dirmap.get(&(src, dst)).unwrap() {
        res += push_direction(prev, *c, depth - 1, dirmap, cache);
        prev = *c;
    }
    cache.insert(k, res);
    res
}

pub fn f(input: AocInput) -> AocResult {
    let num_dirmap = create_num_dirmap();
    let dirmap = create_dirmap();

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
            let d = num_dirmap.get(&(prev, c)).unwrap();
            let mut pprev = 'A';
            for cc in d {
                min_presses1 += push_direction(pprev, *cc, 2, &dirmap, &mut cache);
                min_presses2 += push_direction(pprev, *cc, 25, &dirmap, &mut cache);
                pprev = *cc;
            }
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
