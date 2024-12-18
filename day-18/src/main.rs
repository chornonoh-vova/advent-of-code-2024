use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    curr: Point,
    dist: usize,
}

impl State {
    fn new(curr: Point, dist: usize) -> Self {
        Self { curr, dist }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Memory {
    corrupted: Vec<Vec<bool>>,
    size: i32,
}

impl Memory {
    fn new(bytes: &[Point], size: i32) -> Self {
        let mut corrupted = vec![vec![false; size as usize]; size as usize];
        for point in bytes {
            corrupted[point.row as usize][point.col as usize] = true;
        }
        Self { corrupted, size }
    }

    fn update_corrupted(&mut self, bytes: &[Point]) {
        for row in self.corrupted.iter_mut() {
            for point in row.iter_mut() {
                *point = false;
            }
        }

        for point in bytes {
            self.corrupted[point.row as usize][point.col as usize] = true;
        }
    }

    fn is_corrupted(&self, point: &Point) -> bool {
        self.corrupted[point.row as usize][point.col as usize]
    }

    fn shortest_path(&self) -> i32 {
        let mut best_dist: HashMap<(i32, i32), usize> = HashMap::new();
        let start = (0, 0);
        let end = (self.size - 1, self.size - 1);

        let mut heap = BinaryHeap::from([State::new(Point::new(start.0, start.1), 0)]);
        best_dist.insert(start, 0);

        while let Some(State { curr, dist }) = heap.pop() {
            if (curr.row, curr.col) == end {
                return dist as i32;
            }

            for next in self.neighbors(&curr) {
                let next_dist = dist + 1;

                let next_pos = (next.row, next.col);
                if next_dist < *best_dist.get(&next_pos).unwrap_or(&usize::MAX)
                    && !self.is_corrupted(&next)
                {
                    best_dist.insert(next_pos, next_dist);
                    heap.push(State::new(next, next_dist));
                }
            }
        }

        -1
    }

    fn neighbors(&self, curr: &Point) -> Vec<Point> {
        DIRECTIONS
            .iter()
            .map(|(dy, dx)| Point::new(curr.row + dy, curr.col + dx))
            .filter(|p| p.row >= 0 && p.col >= 0 && p.row < self.size && p.col < self.size)
            .collect()
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = (0..self.size)
            .map(|row| {
                (0..self.size)
                    .map(|col| {
                        if self.is_corrupted(&Point::new(row, col)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();
        f.write_str(map.join("\n").as_str())
    }
}

fn byte_positions(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let (col_str, row_str) = line.split_once(",")?;
            let row = row_str.parse::<i32>().ok()?;
            let col = col_str.parse::<i32>().ok()?;
            Some(Point::new(row, col))
        })
        .collect()
}

fn shortest_path(input: &str, bytes: usize, size: i32) {
    let byte_positions = byte_positions(input);

    let memory = Memory::new(&byte_positions[0..bytes], size);
    println!("shortest path: {}", memory.shortest_path());
}

fn first_unreachable_byte(input: &str, bytes: usize, size: i32) {
    let byte_positions = byte_positions(input);

    let mut bad = bytes;
    let mut good = byte_positions.len();
    let mut memory = Memory::new(&byte_positions[0..bad], size);

    while good - bad > 1 {
        let middle = (good + bad) / 2;
        memory.update_corrupted(&byte_positions[0..middle]);
        let shortest_path = memory.shortest_path();

        if shortest_path == -1 {
            good = middle;
        } else {
            bad = middle;
        }
    }

    println!(
        "first unreachable byte: {},{}",
        byte_positions[bad].col, byte_positions[bad].row
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let size = &args[2].parse::<i32>().expect("Failed to get memory size");
    let bytes = &args[3]
        .parse::<usize>()
        .expect("Failed to get number of falling bytes");

    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    shortest_path(&input, *bytes, *size);
    first_unreachable_byte(&input, *bytes, *size);
}
