use regex::Regex;

use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\)").unwrap();
    let haystack = input
        .lines()
        .fold("".to_owned(), |acc, l| acc + l.unwrap().trim());
    let mut res1 = 0;
    let mut res2 = 0;
    let mut active = true;
    for m in re.captures_iter(&haystack) {
        let l = m.get(0).unwrap().len();
        if l == 4 {
            active = true;
        } else if l == 7 {
            active = false;
        } else {
            let v1: i64 = m.get(1).unwrap().as_str().parse().unwrap();
            let v2: i64 = m.get(2).unwrap().as_str().parse().unwrap();
            let p = v1 * v2;
            res1 += p;
            if active {
                res2 += v1 * v2;
            }
        }
    }
    (res1, res2).into()
}
