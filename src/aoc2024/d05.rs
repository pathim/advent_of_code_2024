use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::AocInput;

fn check(rules: &HashMap<i64, HashSet<i64>>, data_map: &HashMap<i64, usize>) -> bool {
    for (val, pos) in data_map {
        if let Some(must_be_after) = rules.get(val) {
            for after_val in must_be_after {
                if let Some(after_pos) = data_map.get(after_val) {
                    if pos > after_pos {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn compare(rules: &HashMap<i64, HashSet<i64>>, a: &i64, b: &i64) -> cmp::Ordering {
    if let Some(gt) = rules.get(a) {
        if gt.contains(b) {
            return cmp::Ordering::Less;
        }
    }
    if let Some(lt) = rules.get(b) {
        if lt.contains(a) {
            return cmp::Ordering::Greater;
        }
    }
    cmp::Ordering::Equal
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    let mut lines = input.lines();
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (first, second) = line.split_once('|').unwrap();
        let first: i64 = first.parse().unwrap();
        let second: i64 = second.parse().unwrap();
        rules.entry(first).or_default().insert(second);
    }
    for line in lines {
        let line = line.unwrap();
        let mut data_map = HashMap::new();
        let mut data_vec = Vec::new();
        for (i, val) in line.split(',').enumerate() {
            let val: i64 = val.parse().unwrap();
            data_map.insert(val, i);
            data_vec.push(val);
        }
        if check(&rules, &data_map) {
            res1 += data_vec[data_vec.len() / 2];
        } else {
            data_vec.sort_unstable_by(|a, b| compare(&rules, a, b));
            res2 += data_vec[data_vec.len() / 2];
        }
    }
    (res1, res2).into()
}
