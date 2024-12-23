use std::collections::{HashMap, HashSet};

use crate::{AocInput, AocResult};

struct Secret {
    value: u64,
}

impl Secret {
    fn new(seed: u64) -> Self {
        Self { value: seed }
    }
}

impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        const PRUNE_VAL: u64 = (1 << 24) - 1;
        self.value ^= self.value << 6;
        self.value &= PRUNE_VAL;
        self.value ^= self.value >> 5;
        self.value &= PRUNE_VAL;
        self.value ^= self.value << 11;
        self.value &= PRUNE_VAL;
        Some(self.value)
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut res1 = 0;
    let mut total_money = HashMap::new();
    for s in input
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .map(|v| Secret::new(v))
    {
        let values = s.take(2000).collect::<Vec<_>>();
        res1 += values.last().unwrap();
        let current_prices = values
            .into_iter()
            .map(|v| (v % 10) as i64)
            .collect::<Vec<_>>();
        let current_deltas = current_prices
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<_>>();
        let mut already_used = HashSet::new();
        for (i, d) in current_deltas.windows(4).enumerate() {
            if already_used.contains(d) {
                continue;
            }
            let d = d.to_owned();
            let money = total_money.entry(d.clone()).or_insert(0);
            *money += current_prices[i + 4];
            already_used.insert(d);
        }
    }

    let res2 = total_money.iter().map(|x| x.1).max().unwrap();

    (res1, res2).into()
}
