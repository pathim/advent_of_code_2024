use std::collections::HashMap;

use crate::AocInput;

fn next_value(n: i64) -> Vec<i64> {
    if n == 0 {
        vec![1]
    } else {
        let s = n.to_string();
        let l = s.len();
        if l % 2 == 0 {
            let v1 = s[..l / 2].parse().unwrap();
            let v2 = s[l / 2..].parse().unwrap();
            vec![v1, v2]
        } else {
            vec![n * 2024]
        }
    }
}

fn count_after(val: i64, n: usize, memo: &mut HashMap<(i64, usize), usize>) -> usize {
    if n == 0 {
        1
    } else if let Some(v) = memo.get(&(val, n)) {
        *v
    } else {
        let v = next_value(val)
            .into_iter()
            .map(|v| count_after(v, n - 1, memo))
            .sum();
        memo.insert((val, n), v);
        v
    }
}

pub fn f(input: AocInput) -> crate::AocResult {
    let line = input.lines().next().unwrap().unwrap();
    let values: Vec<_> = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut memo = HashMap::new();

    let res1: usize = values.iter().map(|v| count_after(*v, 25, &mut memo)).sum();
    let res2: usize = values.iter().map(|v| count_after(*v, 75, &mut memo)).sum();

    (res1, res2).into()
}
