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

fn find_block_to_move(data: &[Option<(usize, bool)>]) -> Option<(usize, usize)> {
    let mut pos2 = None;
    let mut pos1 = None;
    for pos in (0..data.len()).rev() {
        if let Some((val, is_moved)) = data.get(pos).unwrap() {
            if *is_moved {
                continue;
            }
            pos1 = Some(pos);
            pos2 = Some(pos + 1);
            for p2 in (0..pos).rev() {
                if let Some((val2, _)) = data.get(p2).unwrap() {
                    if *val2 == *val {
                        pos1 = Some(p2);
                    } else {
                        break;
                    }
                }
            }
            break;
        }
    }
    pos1.and_then(|p| pos2.map(|p2| (p, p2 - p)))
}

fn find_space(data: &[Option<(usize, bool)>], size: usize) -> Option<usize> {
    let mut p = 0;
    while let Some(d) = data.get(p) {
        if d.is_some() {
            p += 1;
            continue;
        }
        if p + size > data.len() {
            break;
        }
        if data[p..p + size].iter().all(|x| x.is_none()) {
            return Some(p);
        }
        p += size;
    }
    None
}

fn take_block(data: &mut [Option<(usize, bool)>], pos: usize, size: usize) -> usize {
    let res = data.get(pos).unwrap().unwrap().0;
    for d in data[pos..pos + size].iter_mut() {
        *d = None;
    }
    res
}

fn place_block(data: &mut [Option<(usize, bool)>], pos: usize, size: usize, value: usize) {
    for v in data[pos..pos + size].iter_mut() {
        *v = Some((value, true));
    }
}

pub fn f(input: AocInput) -> crate::AocResult {
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
    let mut r2data = rdata
        .iter()
        .copied()
        .map(|x| x.map(|x| (x, false)))
        .collect::<Vec<_>>();
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

    let res1: usize = rdata
        .iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(a, b)| a * b.unwrap())
        .sum();

    let mut val = 1000000000;
    while let Some((pos, size)) = find_block_to_move(&r2data) {
        let s = find_space(&r2data, size)
            .filter(|s| *s < pos)
            .unwrap_or(pos);
        let value = take_block(&mut r2data, pos, size);
        if val - value != 1 {
            println!("{}", value);
        }
        val = value;
        place_block(&mut r2data, s, size, value);
    }
    let res2: usize = r2data
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|(a, _)| i * a))
        .sum();
    let defrag_length = r2data.iter().filter_map(|x| *x).count();
    println!(
        "total: {}, unpacked: {}, compressed: {}, defrag: {}",
        total_length, unpacked_length, compressed_length, defrag_length
    );
    (res1, res2).into()
}
