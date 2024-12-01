use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io::BufRead,
};

fn read_location_ids_heap() -> (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>) {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut loc = line.split_whitespace();

        let left_loc = loc.next().unwrap().parse::<i32>().unwrap();
        left.push(Reverse(left_loc));

        let right_loc = loc.next().unwrap().parse::<i32>().unwrap();
        right.push(Reverse(right_loc));
    }

    (left, right)
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

fn read_location_ids_map() -> (Vec<i32>, HashMap<i32, i32>) {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut loc = line.split_whitespace();

        let left_loc = loc.next().unwrap().parse::<i32>().unwrap();
        left.push(left_loc);

        let right_loc = loc.next().unwrap().parse::<i32>().unwrap();

        match right.get(&right_loc) {
            Some(r) => right.insert(right_loc, *r + 1),
            None => right.insert(right_loc, 1),
        };
    }

    (left, right)
}

fn similarity(left: Vec<i32>, right: HashMap<i32, i32>) -> i32 {
    let mut res = 0;
    for loc in left {
        let r: i32 = *right.get(&loc).unwrap_or_else(|| &0);
        res += loc * r;
    }
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let operation = &args[1];

    match operation.as_str() {
        "distance" => {
            let (left, right) = read_location_ids_heap();
            println!("{0}", distance(left, right));
        }
        "similarity" => {
            let (left, right) = read_location_ids_map();
            println!("{0}", similarity(left, right))
        }
        _ => panic!("Unknown operation"),
    }
}
