use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self, dir: Dir) -> Vec<(Self, Dir)> {
        vec![
            (Self::new(self.row, self.col + 1), Dir::East),
            (Self::new(self.row + 1, self.col), Dir::South),
            (Self::new(self.row, self.col - 1), Dir::West),
            (Self::new(self.row - 1, self.col), Dir::North),
        ]
        .into_iter()
        .filter(|(_, d)| *d != dir.opposite())
        .collect()
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    curr: Point,
    dir: Dir,
    cost: usize,
    path: Vec<Point>,
}

impl State {
    fn new(curr: Point, dir: Dir, cost: usize, path: Vec<Point>) -> Self {
        Self {
            curr,
            dir,
            cost,
            path,
        }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::default();
        let mut start = Point::default();
        let mut end = Point::default();

        for (i, line) in input.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let curr = Point::new(i, j);
                match ch {
                    'S' => start = curr,
                    'E' => end = curr,
                    '#' => {
                        walls.insert(curr);
                    }
                    _ => {}
                }
            }
        }

        Self { walls, start, end }
    }

    fn shortest_paths(&self) -> (usize, HashSet<Point>) {
        let mut best = usize::MAX;
        let mut unique_points = HashSet::new();

        let mut visited: HashMap<(Point, Dir), usize> = HashMap::new();
        let mut queue = BinaryHeap::from([State::new(self.start, Dir::East, 0, vec![self.start])]);

        while let Some(State {
            curr,
            dir,
            cost,
            path,
        }) = queue.pop()
        {
            if let Some(&prev_cost) = visited.get(&(curr, dir)) {
                if cost > prev_cost {
                    continue;
                }
            } else {
                visited.insert((curr, dir), cost);
            }

            if curr == self.end && cost <= best {
                best = cost;
                unique_points.extend(path.iter());
            }

            for (next, next_dir) in curr.neighbors(dir) {
                if !self.walls.contains(&next) {
                    let next_cost = cost + if next_dir != dir { 1001 } else { 1 };
                    let next_path = {
                        let mut path = path.clone();
                        path.push(next);
                        path
                    };
                    let next_state = State::new(next, next_dir, next_cost, next_path);
                    queue.push(next_state);
                }
            }
        }

        (best, unique_points)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let maze = Maze::new(&input);
    let (cost, unique) = maze.shortest_paths();
    println!("shortest path: {}", cost);
    println!("unique tiles: {}", unique.len())
}
