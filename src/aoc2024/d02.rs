use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let mut values = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap());
        let mut current = values.next().unwrap();
        let mut dir = None;
        let mut is_safe = true;
        for v in values {
            let delta = v - current;
            if delta == 0 || delta.abs() > 3 {
                is_safe = false;
                break;
            }
            let sign = delta.signum();
            if *dir.get_or_insert(sign) != sign {
                is_safe = false;
                break;
            }
            current = v;
        }
        if is_safe {
            res1 += 1;
        }
    }
    (res1).into()
}
