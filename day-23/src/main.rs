use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn get_computer_links(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .collect()
}

fn count_t_computers(input: &str) -> usize {
    let computer_links = get_computer_links(input);

    let mut neighbors: HashSet<(&str, &str)> = HashSet::new();
    let mut vs: HashSet<&str> = HashSet::new();

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
            if neighbors.contains(&(v, a)) && neighbors.contains(&(b, v)) {
                cnt += 1;
            }
        }
    }

    cnt / 3
}

fn bron_kerbosch<'a>(
    mut p: HashSet<&'a str>,
    r: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    n: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut res = Vec::new();

    while let Some(v) = pop(&mut p) {
        let mut nr = r.clone();
        nr.insert(v);

        let np = p.clone().intersection(&n[&v]).cloned().collect();
        let nx = x.clone().intersection(&n[&v]).cloned().collect();

        res.extend(bron_kerbosch(np, nr, nx, n));

        x.insert(v);
    }

    res
}

fn pop<T>(s: &mut HashSet<T>) -> Option<T>
where
    T: Hash + Eq + Clone,
{
    let e = s.iter().next().cloned()?;
    s.take(&e)
}

fn lan_party_password(input: &str) -> String {
    let computer_links = get_computer_links(input);

    let mut neighbors: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut vs: HashSet<&str> = HashSet::new();

    for (a, b) in computer_links.iter() {
        vs.insert(a);
        vs.insert(b);
        neighbors.entry(a).or_default().insert(b);
        neighbors.entry(b).or_default().insert(a);
    }

    let mut computers: Vec<String> = bron_kerbosch(vs, HashSet::new(), HashSet::new(), &neighbors)
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()))
        .map(|vs| vs.iter().map(|v| v.to_string()).collect())
        .unwrap_or_default();

    computers.sort();

    computers.join(",")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    println!("count of t computers: {}", count_t_computers(&input));
    println!("lan party password: {}", lan_party_password(&input));
}
