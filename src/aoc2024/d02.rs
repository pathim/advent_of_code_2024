use crate::AocInput;

pub fn is_safe(data: &[i64]) -> bool {
    let mut values = data.iter();
    let mut current = values.next().unwrap();
    let mut dir = None;
    for v in values {
        let delta = v - current;
        if delta == 0 || delta.abs() > 3 {
            return false;
        }
        let sign = delta.signum();
        if *dir.get_or_insert(sign) != sign {
            return false;
        }
        current = v;
    }
    true
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let values: Vec<_> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if is_safe(&values) {
            res1 += 1;
            res2 += 1;
            continue;
        }

        for i in 0..values.len() {
            let mut v2 = values.clone();
            v2.remove(i);
            if is_safe(&v2) {
                res2 += 1;
                break;
            }
        }
    }
    (res1, res2).into()
}
