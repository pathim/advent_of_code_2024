use crate::{AocInput, AocResult};

#[derive(Clone, Debug, Default)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    x: i64,
    y: i64,
}

impl Machine {
    fn det(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }

    fn inv(&self) -> (Self, i64) {
        let a = self.d;
        let b = -self.b;
        let c = -self.c;
        let d = self.a;

        let x = self.x;
        let y = self.y;
        (Self { a, b, c, d, x, y }, self.det())
    }

    fn solve(&self) -> Option<(i64, i64)> {
        let (i, d) = self.inv();
        if d == 0 {
            None
        } else {
            Some((
                (i.a * self.x + i.b * self.y) / d,
                (i.c * self.x + i.d * self.y) / d,
            ))
        }
    }

    fn check(&self, a: i64, b: i64) -> bool {
        self.a * a + self.b * b == self.x && self.c * a + self.d * b == self.y
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut machines = Vec::new();

    let mut current = Machine::default();
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            machines.push(current.clone());
        }
        if i % 4 == 0 {
            let (_, button_a) = line.split_once(':').unwrap();
            let (x, y) = button_a.split_once(',').unwrap();
            let a: i64 = x.split_once('+').unwrap().1.trim().parse().unwrap();
            let c: i64 = y.split_once('+').unwrap().1.trim().parse().unwrap();
            current.a = a;
            current.c = c;
        } else if i % 4 == 1 {
            let (_, button_b) = line.split_once(':').unwrap();
            let (x, y) = button_b.split_once(',').unwrap();
            let b: i64 = x.split_once('+').unwrap().1.trim().parse().unwrap();
            let d: i64 = y.split_once('+').unwrap().1.trim().parse().unwrap();
            current.b = b;
            current.d = d;
        } else if i % 4 == 2 {
            let (_, prize) = line.split_once(':').unwrap();
            let (x, y) = prize.split_once(',').unwrap();
            let x: i64 = x.split_once('=').unwrap().1.trim().parse().unwrap();
            let y: i64 = y.split_once('=').unwrap().1.trim().parse().unwrap();
            current.x = x;
            current.y = y;
        }
    }
    machines.push(current.clone());
    let mut res1 = 0;
    let mut res2 = 0;
    for mut m in machines {
        if let Some((a, b)) = m.solve() {
            if m.check(a, b) {
                if a <= 100 && b <= 100 && a >= 0 && b >= 0 {
                    res1 += 3 * a + b;
                }
            }
        }
        m.x += 10000000000000;
        m.y += 10000000000000;
        if let Some((a, b)) = m.solve() {
            if m.check(a, b) {
                if a >= 0 && b >= 0 {
                    res2 += 3 * a + b;
                }
            }
        }
    }

    (res1, res2).into()
}
