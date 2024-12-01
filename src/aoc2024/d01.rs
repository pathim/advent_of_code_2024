use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let mut list1 = Vec::<i64>::new();
    let mut list2 = Vec::<i64>::new();
    for line in input.lines() {
        let line = line.unwrap();
        let (n1, n2) = line.split_once(" ").unwrap();
        list1.push(n1.trim().parse().unwrap());
        list2.push(n2.trim().parse().unwrap());
    }
    list1.sort();
    list2.sort();
    let res1 = list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0, std::ops::Add::add);
    let res2 = list1
        .iter()
        .map(|x| x * (list2.iter().filter(|a| **a == *x).count() as i64))
        .fold(0, std::ops::Add::add);
    (res1, res2).into()
}
