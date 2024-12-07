use crate::AocInput;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn incr(&self) -> (Op, bool) {
        match self {
            Op::Add => (Self::Mul, false),
            Op::Mul => (Self::Concat, false),
            Op::Concat => (Self::Add, true),
        }
    }
}
#[derive(Debug)]
struct Ops(Vec<Op>);

impl Ops {
    fn new(n: usize) -> Self {
        Self((0..n).map(|_| Op::Add).collect())
    }

    fn incr(&mut self) -> bool {
        for v in self.0.iter_mut() {
            let (new_val, carry) = v.incr();
            *v = new_val;
            if !carry {
                return false;
            }
        }
        true
    }
}

fn calc(num: &[&str], op: &Ops) -> (bool, u64) {
    let mut res = num.first().unwrap().parse::<u64>().unwrap();
    let mut is_first = true;
    for (n, o) in num[1..].iter().zip(op.0.iter()) {
        let val: u64 = n.parse().unwrap();
        match o {
            Op::Add => {
                res += val;
            }
            Op::Mul => {
                res *= val;
            }
            Op::Concat => {
                is_first = false;
                res = res * 10u64.pow(n.len() as u32) + val;
            }
        }
    }
    (is_first, res)
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (r, nums) = line.trim().split_once(':').unwrap();
        let r: u64 = r.parse().unwrap();
        let nums = nums.split_ascii_whitespace().collect::<Vec<_>>();
        let mut op = Ops::new(nums.len() - 1);
        let mut end = false;
        let mut found = false;
        while !end {
            let (is_first, op_r) = calc(&nums, &op);
            if r == op_r {
                found = true;
                if is_first {
                    res1 += r;
                    break;
                }
            }
            end = op.incr();
        }
        if found {
            res2 += r;
        }
    }
    (res1, res2).into()
}
