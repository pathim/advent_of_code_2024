use std::collections::HashMap;

use crate::{AocInput, AocResult};

fn count_matches(
    pattern: &str,
    towels: &[String],
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    let mut res = 0;
    for t in towels {
        if pattern.starts_with(t) {
            res += if let Some(count) = cache.get(&(t.clone(), pattern.len())) {
                *count
            } else {
                let count = count_matches(&pattern[t.len()..], towels, cache);
                cache.insert((t.clone(), pattern.len()), count);
                count
            };
        }
    }
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
