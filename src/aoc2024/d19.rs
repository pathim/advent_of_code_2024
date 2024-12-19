use std::collections::HashMap;

use crate::{AocInput, AocResult};

fn count_matches(pattern: &str, towels: &[String], cache: &mut HashMap<usize, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(&pattern.len()) {
        return *count;
    }
    let mut res = 0;
    for t in towels {
        if pattern.starts_with(t) {
            res += count_matches(&pattern[t.len()..], towels, cache);
        }
    }
    cache.insert(pattern.len(), res);
    res
}

pub fn f(input: AocInput) -> AocResult {
    let mut lines_iter = input.lines();
    let towels = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .to_owned()
        .map(|s| s.trim().to_owned())
        .collect::<Vec<_>>();

    lines_iter.next();

    let mut res1 = 0;
    let mut res2 = 0;
    for l in lines_iter {
        let l = l.unwrap();
        let count = count_matches(&l, &towels, &mut HashMap::new());
        res2 += count;
        if count > 0 {
            res1 += 1;
        }
    }
    (res1, res2).into()
}
