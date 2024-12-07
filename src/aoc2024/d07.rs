use crate::AocInput;

fn calc(num: &[(u64, usize)], start_val: u64, target: u64, first: bool) -> (bool, bool) {
    if let Some((n, l)) = num.first() {
        let n = *n;
        let mut found = false;
        for o in [|a: u64, b: u64| a + b, |a: u64, b: u64| a * b] {
            let (f, t) = calc(&num[1..], o(start_val, n), target, first);
            if t {
                if f {
                    return (first, true);
                }
                found = true;
            }
        }
        let val = start_val * 10u64.pow(*l as u32) + n;
        found |= calc(&num[1..], val, target, false).1;
        return (false, found);
    }
    (first, start_val == target)
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (r, nums) = line.trim().split_once(':').unwrap();
        let r: u64 = r.parse().unwrap();
        let nums = nums
            .split_ascii_whitespace()
            .map(|x| (x.parse().unwrap(), x.len()))
            .collect::<Vec<_>>();

        let (first, found) = calc(&nums[1..], nums[0].0, r, true);
        if found {
            if first {
                res1 += r;
            }
            res2 += r;
        }
    }
    (res1, res2).into()
}
