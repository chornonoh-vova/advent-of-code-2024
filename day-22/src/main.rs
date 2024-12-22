use std::collections::{HashMap, VecDeque};

fn last_digit(num: i64) -> i64 {
    num % 10
}

fn mix(num: i64, secret: i64) -> i64 {
    num ^ secret
}

fn prune(num: i64) -> i64 {
    num % 16777216
}

struct Prices {
    curr: i64,
    curr_price: i64,
    prev_price: i64,
    changes: VecDeque<i32>,
    n: usize,
    iteration: usize,
}

impl Prices {
    fn new(init: i64, n: usize) -> Self {
        Self {
            curr: init,
            curr_price: last_digit(init),
            prev_price: last_digit(init),
            changes: VecDeque::new(),
            n,
            iteration: 0,
        }
    }

    fn next_secret(&self) -> i64 {
        let mut secret = self.curr;

        let result = secret * 64;
        secret = mix(result, secret);
        secret = prune(secret);

        let result = secret / 32;
        secret = mix(result, secret);
        secret = prune(secret);

        let result = secret * 2048;
        secret = mix(result, secret);
        secret = prune(secret);

        secret
    }
}

impl Iterator for Prices {
    type Item = (i64, i64, Vec<i32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration > self.n {
            return None;
        }

        let changes: Vec<i32> = Vec::from(self.changes.clone());
        let result = (self.curr, self.curr_price, changes);

        self.prev_price = self.curr_price;
        self.curr = self.next_secret();
        self.curr_price = last_digit(self.curr);

        self.changes
            .push_back((self.curr_price - self.prev_price) as i32);

        if self.changes.len() > 4 {
            self.changes.pop_front();
        }

        self.iteration += 1;

        Some(result)
    }
}

fn total_secret(input: &str) {
    let total: i64 = input
        .lines()
        .map(|line| {
            let init = line.parse::<i64>().expect("Invalid secret number");
            Prices::new(init, 2000)
        })
        .map(|prices| {
            let mut result = 0;
            for (secret, _, _) in prices {
                result = secret;
            }
            result
        })
        .sum();

    println!("total: {}", total);
}

fn max_bananas(input: &str) {
    let initial_secrets: Vec<i64> = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    let size = initial_secrets.len();

    let mut map: HashMap<String, Vec<Option<i64>>> = HashMap::new();

    initial_secrets
        .iter()
        .map(|&init| Prices::new(init, 2000))
        .enumerate()
        .for_each(|(i, prices)| {
            for (_, price, prev) in prices {
                if prev.len() < 4 {
                    continue;
                }

                let key = format!("{:?}", prev);
                map.entry(key).or_insert_with(|| vec![None; size])[i].get_or_insert(price);
            }
        });

    let max = map
        .values()
        .map(|v| {
            let sum: i64 = v.iter().filter_map(|&p| p).sum();
            sum
        })
        .max()
        .unwrap_or(0);
    println!("max: {}", max);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    total_secret(&input);
    max_bananas(&input);
}
