use crate::{AocInput, AocResult};

struct Machine<'a> {
    prog: &'a Vec<u8>,
    pc: usize,
    regs: [u64; 3],
    running: bool,
}

impl<'a> Machine<'a> {
    fn new(prog: &'a Vec<u8>, regs: [u64; 3]) -> Self {
        Self {
            prog,
            pc: 0,
            regs,
            running: true,
        }
    }

    fn combo_op(&self, val: u64) -> u64 {
        if val >= 7 {
            panic!("invalid combo operand: {}", val);
        }
        if val <= 3 {
            val
        } else {
            self.regs[(val - 4) as usize]
        }
    }

    fn step(&mut self) -> Option<u64> {
        if let Some(cmd) = self.prog.get(self.pc) {
            self.pc += 1;
            let arg = *self.prog.get(self.pc).unwrap() as u64;
            self.pc += 1;
            match cmd {
                0 => {
                    self.regs[0] >>= self.combo_op(arg);
                }
                1 => {
                    self.regs[1] ^= arg;
                }
                2 => self.regs[1] = self.combo_op(arg) & 7,
                3 => {
                    if self.regs[0] != 0 {
                        self.pc = self.combo_op(arg) as usize
                    };
                }
                4 => {
                    self.regs[1] ^= self.regs[2];
                }
                5 => {
                    return Some(self.combo_op(arg) & 7);
                }
                6 => {
                    self.regs[1] = self.regs[0] >> self.combo_op(arg);
                }
                7 => {
                    self.regs[2] = self.regs[0] >> self.combo_op(arg);
                }
                _ => {
                    panic!("Invalid opcode: {}", cmd);
                }
            }
        } else {
            self.running = false;
        }
        None
    }
}

fn next_a(prog: &Vec<u8>, start_a: u64, target: u64) -> Vec<u64> {
    let mut res = Vec::new();
    for i in 0..8 {
        let a = (start_a << 3) | i;
        let mut machine = Machine::new(prog, [a, 0, 0]);
        while machine.running {
            if let Some(out) = machine.step() {
                if out == target {
                    res.push(a);
                }
                break;
            }
        }
    }
    res
}

fn find_a(prog: &Vec<u8>, start_a: u64, n: usize) -> Option<u64> {
    let a_cand = next_a(prog, start_a, prog[n] as u64);
    if n == 0 {
        return a_cand.first().copied();
    }
    for a in a_cand {
        if let Some(r) = find_a(prog, a, n - 1) {
            return Some(r);
        }
    }
    None
}

pub fn f(input: AocInput) -> AocResult {
    let mut lines_iter = input.lines();
    let a: u64 = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let b: u64 = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let c: u64 = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    lines_iter.next();
    let program: Vec<u8> = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut machine = Machine::new(&program, [a, b, c]);
    let mut output = Vec::new();
    while machine.running {
        if let Some(out) = machine.step() {
            output.push(out);
        }
    }

    let res1 = output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");

    let res2 = find_a(&program, 0, program.len() - 1).unwrap();

    (res1, res2).into()
}
