type Heights = [u8; 5];

fn parse_lock(shape: &str) -> Heights {
    parse_heights(shape.lines())
}

fn parse_key(shape: &str) -> Heights {
    parse_heights(shape.lines().rev())
}

fn parse_heights<'a>(iter: impl Iterator<Item = &'a str>) -> Heights {
    iter.skip(1).fold([0; 5], |mut heights, line| {
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch == '#' {
                heights[i] += 1;
            }
        });
        heights
    })
}

fn parse_input(input: &str) -> (Vec<Heights>, Vec<Heights>) {
    input
        .split("\n\n")
        .fold((Vec::new(), Vec::new()), |(mut locks, mut keys), shape| {
            if shape.starts_with("#####") {
                locks.push(parse_lock(shape));
            } else if shape.starts_with(".....") {
                keys.push(parse_key(shape));
            }
            (locks, keys)
        })
}

fn print_locks_and_keys(locks: &[Heights], keys: &[Heights]) {
    println!("Locks:");
    locks.iter().for_each(|lock| println!("{:?}", lock));

    println!("Keys:");
    keys.iter().for_each(|key| println!("{:?}", key));
}

fn is_compatible(lock: &Heights, key: &Heights) -> bool {
    lock.iter().zip(key).all(|(l, k)| l + k <= 5)
}

fn unique_pairs(locks: &[Heights], keys: &[Heights]) -> usize {
    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| is_compatible(lock, key))
        .count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    let (locks, keys) = parse_input(input.as_str());
    print_locks_and_keys(&locks, &keys);
    println!("unique pairs: {}", unique_pairs(&locks, &keys));
}
