use crate::{AocInput, AocResult};

fn read_key(lines: &mut impl Iterator<Item = String>) -> [u8; 5] {
    let mut res = [5; 5];

    for l in lines {
        if l.is_empty() {
            break;
        }
        for (i, c) in l.chars().enumerate() {
            if c == '.' {
                res[i] -= 1;
            }
        }
    }
    res
}

fn read_lock(lines: &mut impl Iterator<Item = String>) -> [u8; 5] {
    let mut res = [0; 5];

    for l in lines {
        if l.is_empty() {
            break;
        }
        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                res[i] += 1;
            }
        }
    }
    res
}

fn check_key(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    key.iter().zip(lock).all(|(k, l)| k + l < 6)
}

pub fn f(input: AocInput) -> AocResult {
    let mut input_iter = input.lines().map(|x| x.unwrap());
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    while let Some(l) = input_iter.next() {
        if l.starts_with('#') {
            let lock = read_lock(&mut input_iter);
            locks.push(lock);
        } else if l.starts_with('.') {
            let key = read_key(&mut input_iter);
            keys.push(key);
        } else {
            panic!("Invalid line '{}'", l);
        }
    }

    let mut res1 = 0;
    for key in keys {
        for lock in &locks {
            if check_key(&key, lock) {
                res1 += 1;
            }
        }
    }
    res1.into()
}
