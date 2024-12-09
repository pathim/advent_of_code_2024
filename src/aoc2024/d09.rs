use crate::AocInput;

fn unpack(data: &Vec<(Option<usize>, usize)>) -> Vec<Option<usize>> {
    data.iter()
        .flat_map(|(val, rep)| vec![val; *rep])
        .copied()
        .collect()
}

fn get_last(data: &mut Vec<Option<usize>>) -> (usize, usize) {
    let pos = data.iter().rposition(|x| x.is_some()).unwrap();
    let v = data[pos].take().unwrap();
    (pos, v)
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res2 = 0;
    let data = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            (
                if i % 2 == 0 { Some(i / 2) } else { None },
                (c.to_digit(10).unwrap() as usize),
            )
        })
        .collect::<Vec<_>>();
    let total_length: usize = data.iter().filter_map(|(a, b)| a.map(|_| b)).sum();
    let mut rdata = unpack(&data);
    let unpacked_length = rdata.iter().filter_map(|x| *x).count();
    let mut last_pos = rdata.len();
    for i in 0..rdata.len() {
        if rdata[i].is_some() {
            continue;
        }
        if last_pos <= i {
            break;
        }
        let (new_last_pos, last_val) = get_last(&mut rdata);
        last_pos = new_last_pos;
        rdata[i].replace(last_val);
    }
    let compressed_length = rdata.iter().take_while(|x| x.is_some()).count();
    println!(
        "total: {}, unpacked: {}, compressed: {}",
        total_length, unpacked_length, compressed_length
    );
    let res1: usize = rdata
        .iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(a, b)| a * b.unwrap())
        .sum();
    (res1, res2).into()
}
