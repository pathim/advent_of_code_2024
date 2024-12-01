use crate::AocInput;

fn count_sorted(v: &[i64], x: i64) -> usize {
    if let Ok(idx) = v.binary_search(&x) {
        let mut res = 1;
        let mut delta = 1;
        while let Some(a) = v.get(idx - delta) {
            if *a != x {
                break;
            }
            res += 1;
            delta += 1;
        }
        let mut delta = 1;
        while let Some(a) = v.get(idx + delta) {
            if *a != x {
                break;
            }
            res += 1;
            delta += 1;
        }
        res
    } else {
        0
    }
}

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
        .map(|x| x * (count_sorted(&list2, *x) as i64))
        .fold(0, std::ops::Add::add);
    (res1, res2).into()
}
