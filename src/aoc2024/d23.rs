use std::collections::{HashMap, HashSet};

use crate::{AocInput, AocResult};

fn add_connection(c1: &str, c2: &str, connections: &mut HashMap<String, HashSet<String>>) {
    let conn = connections.entry(c1.to_string()).or_insert(HashSet::new());
    conn.insert(c2.to_owned());
}

pub fn f(input: AocInput) -> AocResult {
    let mut connections = HashMap::new();
    for l in input.lines() {
        let l = l.unwrap();
        let (c1, c2) = l.split_once('-').unwrap();
        add_connection(c1, c2, &mut connections);
        add_connection(c2, c1, &mut connections);
    }
    let mut cliques = HashSet::new();
    for (pc1, other) in &connections {
        for pc2 in other {
            for third in connections.get(pc2).unwrap().intersection(other) {
                let mut clique = [pc1.to_owned(), pc2.to_owned(), third.to_owned()];
                clique.sort();
                cliques.insert(clique);
            }
        }
    }

    let mut res1 = 0;
    for c in cliques {
        for p in c {
            if p.starts_with('t') {
                res1 += 1;
                break;
            }
        }
    }
    res1.into()
}
