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
fn update(values: &[i64]) -> Vec<i64> {
    values.iter().flat_map(|v| next_value(*v)).collect()
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res2 = 0;
    let line = input.lines().next().unwrap().unwrap();
    let mut values: Vec<_> = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for _ in 0..25 {
        values = update(&values);
    }

    let res1 = values.len();

    (res1, res2).into()
}
