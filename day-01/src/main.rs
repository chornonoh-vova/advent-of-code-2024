use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn read_lines<P>(filename: P) -> Result<impl Iterator<Item = Result<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_pairs(lines: impl Iterator<Item = Result<String>>) -> impl Iterator<Item = (i32, i32)> {
    lines.flatten().map(|line| {
        let mut loc = line.split_whitespace();
        let left = loc.next().unwrap().parse::<i32>().unwrap();
        let right = loc.next().unwrap().parse::<i32>().unwrap();
        (left, right)
    })
}

fn calc_distance(filename: &str) {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    get_pairs(read_lines(filename).unwrap()).for_each(|(l, r)| {
        left.push(Reverse(l));
        right.push(Reverse(r));
    });

    println!("{0}", distance(left, right));
}

fn distance(mut left: BinaryHeap<Reverse<i32>>, mut right: BinaryHeap<Reverse<i32>>) -> i32 {
    let mut res = 0;
    while !left.is_empty() {
        let l = left.pop().unwrap().0;
        let r = right.pop().unwrap().0;
        res += (l - r).abs();
    }
    res
}

fn calc_similarity(filename: &str) {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    get_pairs(read_lines(filename).unwrap()).for_each(|(l, r)| {
        left.push(l);

        right.insert(r, *right.get(&r).unwrap_or(&0) + 1);
    });

    println!("{0}", similarity(left, right));
}

fn similarity(left: Vec<i32>, right: HashMap<i32, i32>) -> i32 {
    let mut res = 0;
    for loc in left {
        let r: i32 = *right.get(&loc).unwrap_or(&0);
        res += loc * r;
    }
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let operation = &args[1];
    let filename = &args[2];

    match operation.as_str() {
        "distance" => calc_distance(filename.as_str()),
        "similarity" => calc_similarity(filename.as_str()),
        _ => panic!("Unknown operation"),
    }
}
