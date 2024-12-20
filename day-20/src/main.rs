use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

type PointSet = HashSet<Point>;
type CheatMap = BTreeMap<i32, Vec<(Point, Point)>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as i32,
            col: col as i32,
        }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { row: -1, col: -1 }
    }
}

#[derive(Debug)]
struct RaceTrack {
    walls: PointSet,
    start: Point,
    end: Point,
    size: i32,
}

impl RaceTrack {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = Point::default();
        let mut end = Point::default();
        let mut size = -1;

        for (i, line) in input.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let curr = Point::new(i, j);
                match ch {
                    '#' => {
                        walls.insert(curr);
                    }
                    'S' => start = curr,
                    'E' => end = curr,
                    _ => {}
                }
            }
            size = i as i32 + 1;
        }

        Self {
            walls,
            start,
            end,
            size,
        }
    }

    fn find_path(&self) -> (Vec<Point>, HashMap<Point, i32>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();

        queue.push_back((self.start, 0));
        visited.insert(self.start);

        while let Some((curr, dist)) = queue.pop_front() {
            if curr == self.end {
                let mut path_points = HashMap::new();
                let mut path = Vec::new();
                let mut trace = Some(curr);
                let mut step = dist;
                while let Some(p) = trace {
                    path_points.insert(p, step);
                    path.push(p);
                    trace = prev.get(&p).cloned();
                    if trace.is_some() {
                        step -= 1;
                    }
                }
                path.reverse();
                return (path, path_points);
            }

            for neighbor in self.neighbors(&curr) {
                if visited.insert(neighbor) && !self.is_wall(&neighbor) {
                    prev.insert(neighbor, curr);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        panic!("No path found from start to end");
    }

    fn find_cheats(
        &self,
        path: &[Point],
        path_points: &HashMap<Point, i32>,
        max_len: i32,
    ) -> CheatMap {
        let mut cheats = BTreeMap::new();

        for (dist, &point) in path.iter().enumerate() {
            for next_point in self.manhattan_circle(&point, max_len) {
                if let Some(&next_dist) = path_points.get(&next_point) {
                    let saved = next_dist - dist as i32 - point.manhattan_distance(&next_point);
                    if saved > 0 {
                        cheats
                            .entry(saved)
                            .or_insert_with(Vec::new)
                            .push((point, next_point));
                    }
                }
            }
        }

        cheats
    }

    fn neighbors(&self, p: &Point) -> PointSet {
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|&d| *p + d)
            .filter(|p| self.is_in_bounds(p))
            .collect()
    }

    fn is_in_bounds(&self, p: &Point) -> bool {
        p.row >= 0 && p.col >= 0 && p.row < self.size && p.col < self.size
    }

    fn is_wall(&self, p: &Point) -> bool {
        self.walls.contains(p)
    }

    fn manhattan_circle(&self, start: &Point, diameter: i32) -> PointSet {
        let mut points = HashSet::new();

        for drow in -diameter..=diameter {
            let remaining = diameter - drow.abs();
            for dcol in -remaining..=remaining {
                let point = *start + (drow, dcol);
                if self.is_in_bounds(&point) && !self.is_wall(&point) {
                    points.insert(point);
                }
            }
        }

        points
    }
}

fn print_cheats(cheats: &CheatMap, threshold: i32) {
    let mut above_threshold = 0;
    println!("Threshold: {}", threshold);
    for cheat in cheats {
        let cnt = cheat.1.len();
        println!("There are {} cheats that save {} picoseconds", cnt, cheat.0);
        if *cheat.0 >= threshold {
            above_threshold += cnt;
        }
    }
    println!("Above threshold: {}", above_threshold);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let max_cheat_len = &args[2]
        .parse::<i32>()
        .expect("Failed to parse max cheat length");
    let threshold = &args[3].parse::<i32>().expect("Failed to parse threshold");
    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let racetrack = RaceTrack::new(&input);
    let (path, path_points) = racetrack.find_path();
    let cheats = racetrack.find_cheats(&path, &path_points, *max_cheat_len);
    print_cheats(&cheats, *threshold);
}
