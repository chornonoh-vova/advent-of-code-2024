use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn get_computer_links(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn count_t_computers(input: &str) -> usize {
    let computer_links = get_computer_links(input);

    let mut neighbors = HashSet::new();
    let mut vs = HashSet::new();

    for (a, b) in &computer_links {
        vs.insert(a);
        vs.insert(b);
        neighbors.insert((a, b));
        neighbors.insert((b, a));
    }

    let mut cnt = 0;

    for (a, b) in &computer_links {
        for v in &vs {
            if !a.starts_with("t") && !b.starts_with("t") && !v.starts_with("t") {
                continue;
            }
            if neighbors.contains(&(*v, a)) && neighbors.contains(&(b, *v)) {
                cnt += 1;
            }
        }
    }

    cnt / 3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    label: &'a str,
}

fn bron_kerbosch<'a>(
    mut p: HashSet<Vertex<'a>>,
    r: HashSet<Vertex<'a>>,
    mut x: HashSet<Vertex<'a>>,
    n: &HashMap<Vertex<'a>, HashSet<Vertex<'a>>>,
) -> Vec<HashSet<Vertex<'a>>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut res = Vec::new();

    while let Some(v) = pop(&mut p) {
        let mut nr = r.clone();
        nr.insert(v);

        let np = p.clone().intersection(&n[&v]).copied().collect();
        let nx = x.clone().intersection(&n[&v]).copied().collect();

        res.extend(bron_kerbosch(np, nr, nx, n));

        x.insert(v);
    }

    res
}

fn pop<T>(s: &mut HashSet<T>) -> Option<T>
where
    T: Hash + Eq + Copy,
{
    let e = s.iter().next().copied()?;
    s.take(&e)
}

fn lan_party_password(input: &str) -> String {
    let computer_links = get_computer_links(input);

    let mut neighbors = HashMap::new();
    let mut vs = HashSet::new();

    for (a, b) in &computer_links {
        let v1 = Vertex { label: a };
        let v2 = Vertex { label: b };
        vs.insert(v1);
        vs.insert(v2);
        neighbors
            .entry(v1)
            .and_modify(|vs: &mut HashSet<Vertex>| {
                vs.insert(v2);
            })
            .or_insert(HashSet::from([v2]));
        neighbors
            .entry(v2)
            .and_modify(|vs: &mut HashSet<Vertex>| {
                vs.insert(v1);
            })
            .or_insert(HashSet::from([v1]));
    }

    bron_kerbosch(vs, HashSet::new(), HashSet::new(), &neighbors)
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()))
        .map(|vs| {
            let mut labels: Vec<String> = vs.iter().map(|v| v.label.to_string()).collect();
            labels.sort();
            labels.join(",")
        })
        .unwrap_or(String::new())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    println!("count of t computers: {}", count_t_computers(&input));
    println!("lan party password: {}", lan_party_password(&input));
}
