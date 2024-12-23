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
    let mut prices = Vec::new();
    for s in input
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .map(|v| Secret::new(v))
    {
        let values = s.take(2000).collect::<Vec<_>>();
        res1 += values.last().unwrap();
        prices.push(
            values
                .into_iter()
                .map(|v| (v % 10) as i8)
                .collect::<Vec<_>>(),
        );
    }

    res1.into()
}
