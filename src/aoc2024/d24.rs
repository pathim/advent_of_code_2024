use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{AocInput, AocResult};

enum Op {
    And,
    Or,
    Xor,
    Const,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Invalud Op {}", value),
        }
    }
}

struct Gate {
    op: Op,
    inputs: [String; 2],
    output: String,
}

impl Gate {
    fn const_val(v: bool, output: String) -> Self {
        let input = if v { "".to_string() } else { "a".to_string() };
        Self {
            op: Op::Const,
            inputs: [input.clone(), input],
            output,
        }
    }
    fn eval(&self, gates: &HashMap<String, Self>) -> bool {
        let in1 = gates
            .get(&self.inputs[0])
            .map(|g| g.eval(gates))
            .unwrap_or(false);
        let in2 = gates
            .get(&self.inputs[1])
            .map(|g| g.eval(gates))
            .unwrap_or(false);
        match self.op {
            Op::And => in1 && in2,
            Op::Or => in1 || in2,
            Op::Xor => in1 ^ in2,
            Op::Const => self.inputs[0].is_empty(),
        }
    }
    fn deps(&self, gates: &HashMap<String, Self>) -> Vec<String> {
        let mut in1 = gates
            .get(&self.inputs[0])
            .map(|g| g.deps(gates))
            .unwrap_or(Vec::new());
        let mut in2 = gates
            .get(&self.inputs[1])
            .map(|g| g.deps(gates))
            .unwrap_or(Vec::new());
        match self.op {
            Op::Const => {
                in1.push(self.output.clone());
            }
            _ => in1.extend(in2),
        }
        in1.sort();
        in1
    }
}

impl FromStr for Gate {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split_ascii_whitespace();
        let in1 = vals.next().unwrap().to_string();
        let op = vals.next().unwrap().into();
        let in2 = vals.next().unwrap().to_string();
        vals.next();
        let output = vals.next().unwrap().to_string();

        Ok(Self {
            op,
            inputs: [in1, in2],
            output,
        })
    }

    type Err = ();
}

fn check_deps(out: String, deps: &[String]) {
    let out_num = out[1..].parse::<u64>().unwrap();
    let mut dep_x = Vec::new();
    let mut dep_y = Vec::new();
    for d in deps {
        let dval = d[1..].parse::<u64>().unwrap();
        if d.starts_with('x') {
            dep_x.push(dval);
        } else {
            dep_y.push(dval);
        }
    }
    dep_x.sort();
    dep_y.sort();
    if dep_x != dep_y {
        println!("xy differ {}", out);
    }
    println!("out:{}, {:?}", out, dep_x);
}

pub fn f(input: AocInput) -> AocResult {
    let mut gates = HashMap::new();
    let mut lines_iter = input.lines();
    while let Some(Ok(l)) = lines_iter.next() {
        if l.is_empty() {
            break;
        }
        let (k, v) = l.split_once(':').unwrap();
        let v = if v.trim().parse::<u8>().unwrap() == 1 {
            true
        } else {
            false
        };
        let gate = Gate::const_val(v, k.to_string());
        gates.insert(gate.output.clone(), gate);
    }

    let mut outputs = Vec::new();
    for l in lines_iter {
        let l = l.unwrap();
        let gate = Gate::from_str(&l).unwrap();
        let out = gate.output.clone();
        if out.starts_with('z') {
            outputs.push(out.clone());
        }
        gates.insert(out, gate);
    }
    outputs.sort();

    let mut res1 = 0;

    for o in outputs.into_iter().rev() {
        let out_gate = gates.get(&o).unwrap();
        let v = out_gate.eval(&gates) as u64;
        //println!("{}: {:?}",o,out_gate.deps(&gates).len());
        check_deps(o, &out_gate.deps(&gates));
        res1 <<= 1;
        res1 |= v;
    }

    res1.into()
}