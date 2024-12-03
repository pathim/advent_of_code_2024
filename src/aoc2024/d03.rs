use regex::Regex;

use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    let haystack = input
        .lines()
        .fold("".to_owned(), |acc, l| acc + l.unwrap().trim());
    let mut res1 = 0;
    for m in re.captures_iter(&haystack) {
        let v1: i64 = dbg!(m.get(1).unwrap().as_str()).parse().unwrap();
        let v2: i64 = dbg!(m.get(2).unwrap().as_str()).parse().unwrap();
        res1 += v1 * v2;
    }

    res1.into()
}
