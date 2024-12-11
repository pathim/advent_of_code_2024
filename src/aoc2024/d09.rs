use std::collections::BinaryHeap;

use crate::AocInput;

#[derive(Debug)]
struct Block {
    id: usize,
    pos: usize,
    size: usize,
}

impl Block {
    fn eval(&self) -> usize {
        let mut res = 0;
        for i in self.pos..self.pos + self.size {
            res += self.id * i;
        }
        res
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Block {}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug)]
struct Space {
    pos: usize,
    size: usize,
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
    }
}

impl Eq for Space {}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}

fn unpack(data: &[(Option<usize>, usize)]) -> Vec<Option<usize>> {
    data.iter()
        .flat_map(|(val, rep)| vec![val; *rep])
        .copied()
        .collect()
}

fn get_last(data: &mut [Option<usize>]) -> (usize, usize) {
    let pos = data.iter().rposition(|x| x.is_some()).unwrap();
    let v = data[pos].take().unwrap();
    (pos, v)
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut blocks = BinaryHeap::new();

    let mut spaces = Vec::new();
    let mut pos = 0;
    let mut data = Vec::new();
    for (id, size) in input
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
    {
        data.push((id, size));
        if let Some(id) = id {
            blocks.push(Block { id, pos, size });
        } else {
            spaces.push(Space { pos, size });
        }
        pos += size;
    }
    let mut rdata = unpack(&data);
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

    let res1: usize = rdata
        .iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(a, b)| a * b.unwrap())
        .sum();

    let mut used_blocks = Vec::with_capacity(blocks.len());
    while let Some(mut b) = blocks.pop() {
        for s in spaces.iter_mut() {
            if s.pos >= b.pos {
                break;
            }
            if s.size >= b.size {
                b.pos = s.pos;
                s.size -= b.size;
                s.pos += b.size;
                break;
            }
        }
        used_blocks.push(b);
    }

    let res2: usize = used_blocks.iter().map(Block::eval).sum();

    (res1, res2).into()
}
