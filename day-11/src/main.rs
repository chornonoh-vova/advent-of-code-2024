use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io;

fn read_input<P>(filename: P) -> io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    read_to_string(filename)
}

fn parse_stones(input: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();

    for stone_str in input.split_whitespace() {
        let stone = stone_str.parse::<u64>().unwrap();
        stones.entry(stone).and_modify(|e| *e += 1).or_insert(1);
    }

    stones
}

fn split_number(n: u64) -> (u64, u64) {
    let num_digits = ((n as f64).log10().floor() as usize + 1) / 2;
    let divisor = 10_u64.pow(num_digits as u32);
    (n / divisor, n % divisor)
}

fn transform_stones(stones: &mut HashMap<u64, u64>) {
    let mut new_stones = HashMap::with_capacity(stones.len() * 2);
    for (&stone, &n) in stones.iter() {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += n;
        } else {
            let num_digits = (stone as f64).log10().floor() as usize + 1;
            if num_digits % 2 == 0 {
                let (l, r) = split_number(stone);
                *new_stones.entry(l).or_insert(0) += n;
                *new_stones.entry(r).or_insert(0) += n;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += n;
            }
        }
    }

    *stones = new_stones
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    let input = read_input(filename).expect("Failed to read input file");
    let mut stones = parse_stones(&input);

    for _ in 0..75 {
        transform_stones(&mut stones);
    }

    println!("Length: {}", stones.iter().fold(0, |acc, (_, n)| acc + *n));
}
