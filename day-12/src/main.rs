use std::{
    collections::{HashMap, HashSet},
    env,
    fs::read_to_string,
    io,
};

fn read_input<P>(filename: P) -> io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    read_to_string(filename)
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct GardenMap {
    map: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl GardenMap {
    fn new(garden_map: &str) -> Self {
        let map: Vec<Vec<char>> = garden_map.lines().map(|l| l.chars().collect()).collect();
        let rows = map.len() as i32;
        let cols = map[0].len() as i32;
        Self { map, rows, cols }
    }

    fn get(&self, i: i32, j: i32) -> Option<char> {
        if i >= 0 && j >= 0 && i < self.rows && j < self.cols {
            Some(self.map[i as usize][j as usize])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct PlantRegion {
    id: char,
    points: HashSet<(i32, i32)>,
}

impl PlantRegion {
    fn new(id: char, points: HashSet<(i32, i32)>) -> Self {
        Self { id, points }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self, garden_map: &GardenMap) -> usize {
        let mut perimeter = 0;

        for (i, j) in self.points.iter() {
            for (di, dj) in DIRECTIONS {
                let ni = i + di;
                let nj = j + dj;

                perimeter += match garden_map.get(ni, nj) {
                    Some(ch) => {
                        if ch != self.id {
                            1
                        } else {
                            0
                        }
                    }
                    None => 1,
                };
            }
        }

        perimeter
    }

    fn sides(&self, garden_map: &GardenMap) -> usize {
        let mut rows: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
        let mut cols: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();

        for (i, j) in self.points.iter() {
            rows.entry(*i)
                .and_modify(|e| {
                    e.insert((*i, *j));
                })
                .or_insert_with(|| HashSet::from([(*i, *j)]));
            cols.entry(*j)
                .and_modify(|e| {
                    e.insert((*i, *j));
                })
                .or_insert_with(|| HashSet::from([(*i, *j)]));
        }

        let mut sides = 0;

        for (_, row) in rows {
            let mut top_sides = 0;
            let mut top = false;
            let mut bottom_sides = 0;
            let mut bottom = false;

            let mut row: Vec<(i32, i32)> = row.iter().cloned().collect();

            row.sort_by_key(|e| e.1);

            for (i, j) in row {
                let t = garden_map.get(i - 1, j);

                if t.is_none() || t.unwrap() != self.id {
                    if !top {
                        top_sides += 1;
                        top = true;
                    }
                } else {
                    top = false;
                }

                let b = garden_map.get(i + 1, j);

                if b.is_none() || b.unwrap() != self.id {
                    if !bottom {
                        bottom_sides += 1;
                        bottom = true;
                    }
                } else {
                    bottom = false;
                }

                if let Some(ch) = garden_map.get(i, j + 1) {
                    if ch != self.id {
                        top = false;
                        bottom = false;
                    }
                }
            }

            sides += top_sides;
            sides += bottom_sides;
        }

        for (_, col) in cols {
            let mut left_sides = 0;
            let mut left = false;
            let mut right_sides = 0;
            let mut right = false;

            let mut col: Vec<(i32, i32)> = col.iter().cloned().collect();

            col.sort_by_key(|e| e.0);

            for (i, j) in col {
                let l = garden_map.get(i, j - 1);

                if l.is_none() || l.unwrap() != self.id {
                    if !left {
                        left_sides += 1;
                        left = true;
                    }
                } else {
                    left = false;
                }

                let r = garden_map.get(i, j + 1);

                if r.is_none() || r.unwrap() != self.id {
                    if !right {
                        right_sides += 1;
                        right = true;
                    }
                } else {
                    right = false;
                }

                if let Some(ch) = garden_map.get(i + 1, j) {
                    if ch != self.id {
                        left = false;
                        right = false;
                    }
                }
            }

            sides += left_sides;
            sides += right_sides;
        }

        sides
    }
}

fn trace_region_points(
    garden_map: &GardenMap,
    points: &mut HashSet<(i32, i32)>,
    id: char,
    curr: (i32, i32),
) {
    points.insert(curr);

    for (di, dj) in DIRECTIONS {
        let ni = curr.0 + di;
        let nj = curr.1 + dj;

        if let Some(ch) = garden_map.get(ni, nj) {
            let next = (ni, nj);
            if id == ch && !points.contains(&next) {
                trace_region_points(garden_map, points, id, next);
            }
        }
    }
}

fn trace_region(garden_map: &GardenMap, id: char, start: (i32, i32)) -> PlantRegion {
    let mut points = HashSet::new();

    trace_region_points(garden_map, &mut points, id, start);

    PlantRegion::new(id, points)
}

fn find_regions(garden_map: &GardenMap) -> Vec<PlantRegion> {
    let mut regions = vec![];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in garden_map.map.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if !visited.contains(&(i as i32, j as i32)) {
                let region = trace_region(garden_map, *ch, (i as i32, j as i32));
                region.points.iter().for_each(|p| {
                    visited.insert(*p);
                });
                regions.push(region);
            }
        }
    }

    regions
}

fn total_cost(regions: &[PlantRegion], garden_map: &GardenMap) -> usize {
    regions
        .iter()
        .map(|r| r.area() * r.perimeter(garden_map))
        .sum()
}

fn total_cost_with_discount(regions: &[PlantRegion], garden_map: &GardenMap) -> usize {
    regions.iter().map(|r| r.area() * r.sides(garden_map)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    let input = read_input(filename).expect("Failed to read input file");

    let garden_map = GardenMap::new(&input);
    let regions = find_regions(&garden_map);
    println!("total cost: {}", total_cost(&regions, &garden_map));
    println!(
        "total cost with discount: {}",
        total_cost_with_discount(&regions, &garden_map)
    );
}
