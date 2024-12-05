use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::AocInput;

fn compare(rules: &HashMap<i64, HashSet<i64>>, a: &i64, b: &i64) -> Ordering {
    if let Some(gt) = rules.get(a) {
        if gt.contains(b) {
            return Ordering::Less;
        }
    }
    if let Some(lt) = rules.get(b) {
        if lt.contains(a) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
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
        let data_vec = line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let mut sorted_data = data_vec.clone();
        sorted_data.sort_unstable_by(|a, b| compare(&rules, a, b));
        let middle_value = sorted_data[sorted_data.len() / 2];
        if data_vec == sorted_data {
            res1 += middle_value;
        } else {
            res2 += middle_value;
        }
    }
    (res1, res2).into()
}
