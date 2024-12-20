use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

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
}

impl Default for Point {
    fn default() -> Self {
        Self { row: -1, col: -1 }
    }
}

#[derive(Debug)]
struct RaceTrack {
    walls: HashSet<Point>,
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

    fn find_path(&self) -> (BTreeMap<usize, Point>, HashMap<Point, usize>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();

        queue.push_back((self.start, 0));
        visited.insert(self.start);

        while let Some((curr, dist)) = queue.pop_front() {
            if curr == self.end {
                let mut path = BTreeMap::new();
                let mut path_points = HashMap::new();
                let mut trace = Some(curr);
                let mut step = dist;
                while let Some(p) = trace {
                    path.insert(step, p);
                    path_points.insert(p, step);
                    trace = prev.get(&p).cloned();
                    if trace.is_some() {
                        step -= 1;
                    }
                }
                return (path, path_points);
            }

            for neighbor in self.neighbors(&curr) {
                if visited.insert(neighbor) && !self.walls.contains(&neighbor) {
                    prev.insert(neighbor, curr);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        panic!("No path found from start to end");
    }

    fn find_cheats(
        &self,
        path: &BTreeMap<usize, Point>,
        path_points: &HashMap<Point, usize>,
    ) -> BTreeMap<usize, Vec<(Point, Point)>> {
        let mut cheats = BTreeMap::new();

        for (&dist, point) in path {
            for (cheat_start, cheat_end, cheat_len) in self.neighbor_cheats(point) {
                if let Some(&next_dist) = path_points.get(&cheat_end) {
                    if next_dist > dist {
                        cheats
                            .entry(next_dist - dist - cheat_len)
                            .and_modify(|e: &mut Vec<(Point, Point)>| {
                                e.push((cheat_start, cheat_end))
                            })
                            .or_insert_with(|| vec![(cheat_start, cheat_end)]);
                    }
                }
            }
        }

        cheats
    }

    fn neighbors(&self, p: &Point) -> Vec<Point> {
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|d| Point {
                row: p.row + d.0,
                col: p.col + d.1,
            })
            .filter(|p| p.row >= 0 && p.col >= 0 && p.row < self.size && p.col < self.size)
            .collect()
    }

    fn neighbor_cheats(&self, p: &Point) -> Vec<(Point, Point, usize)> {
        [
            ((-1, 0), (-2, 0)),
            ((0, 1), (0, 2)),
            ((1, 0), (2, 0)),
            ((0, -1), (0, -2)),
        ]
        .iter()
        .map(|(d1, d2)| {
            (
                Point {
                    row: p.row + d1.0,
                    col: p.col + d1.1,
                },
                Point {
                    row: p.row + d2.0,
                    col: p.col + d2.1,
                },
                2,
            )
        })
        .filter(|(p1, p2, _)| self.walls.contains(p1) && !self.walls.contains(p2))
        .collect()
    }
}

fn print_cheats(cheats: &BTreeMap<usize, Vec<(Point, Point)>>, threshold: usize) {
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
    let threshold = &args[2].parse::<usize>().expect("Failed to parse threshold");
    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let racetrack = RaceTrack::new(&input);
    let (path, path_points) = racetrack.find_path();
    let cheats = racetrack.find_cheats(&path, &path_points);
    print_cheats(&cheats, *threshold);
}
