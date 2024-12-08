use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Self(i as i32, j as i32)
    }

    fn inside(&self, rows: i32, cols: i32) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < rows && self.1 < cols
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct CityMap {
    rows: i32,
    cols: i32,
    frequencies: HashMap<char, HashSet<Position>>,
}

impl CityMap {
    fn new(city_map_str: &str) -> Self {
        let mut frequencies = HashMap::new();

        let mut rows = 0;
        let mut cols = 0;

        for (i, row) in city_map_str.lines().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                cols = j;
                if ch == '.' {
                    continue;
                }

                let antenna = Position::new(i, j);

                frequencies
                    .entry(ch)
                    .and_modify(|e: &mut HashSet<Position>| {
                        e.insert(antenna);
                    })
                    .or_insert_with(|| HashSet::from([antenna]));
            }
            rows = i;
        }

        println!("{}\n{}x{}", city_map_str, rows, cols);

        Self {
            rows: (rows + 1) as i32,
            cols: (cols + 1) as i32,
            frequencies,
        }
    }

    fn anti_nodes(&self) -> HashSet<Position> {
        let mut anti_nodes = HashSet::new();

        for (frequency, antennas) in self.frequencies.iter() {
            for antenna1 in antennas.iter() {
                for antenna2 in antennas.difference(&HashSet::from([*antenna1])) {
                    let difference = *antenna1 - *antenna2;
                    let mut possible = *antenna2;
                    while possible.inside(self.rows, self.cols) {
                        println!(
                            "frequency: {}, a1: {:?}, a2: {:?} possible: {:?}",
                            frequency, antenna1, antenna2, possible,
                        );

                        anti_nodes.insert(possible);

                        possible = possible - difference;
                    }
                }
            }
        }

        anti_nodes
    }
}

fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let city_map_str = read_input(filename)?;

    let city_map = CityMap::new(&city_map_str);
    let anti_nodes = city_map.anti_nodes();

    println!("anti nodes: {:?}\ncount: {}", anti_nodes, anti_nodes.len());

    Ok(())
}
