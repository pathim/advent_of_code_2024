use std::collections::{HashMap, HashSet};

use crate::{AocInput, AocResult};

fn larger(members: &[String], connections: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    let mut new_members = connections
        .get(members.iter().next().unwrap())
        .unwrap()
        .clone();
    for m in members {
        new_members = new_members
            .intersection(connections.get(m).unwrap())
            .cloned()
            .collect::<HashSet<_>>();
    }
    let mut res = Vec::new();

    for n in new_members {
        let mut new_clique = members.to_vec();
        new_clique.push(n);
        new_clique.sort();
        res.push(new_clique);
    }
    res
}

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

    let mut current_cliques = HashSet::new();
    current_cliques.extend(cliques.iter().map(|x| x.to_vec()));

    let largest = loop {
        let mut next_cliques = HashSet::new();
        for c in current_cliques {
            next_cliques.extend(larger(&c, &connections));
        }
        if next_cliques.len() == 1 {
            break next_cliques.iter().cloned().next().unwrap();
        }
        current_cliques = next_cliques;
    };

    let mut res1 = 0;
    for c in cliques {
        for p in c {
            if p.starts_with('t') {
                res1 += 1;
                break;
            }
        }
    }

    let res2 = largest.join(",");
    (res1, res2).into()
}
