use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position(i32, i32);

fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn get_lab_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_start_position(lab_map: &[Vec<char>]) -> Position {
    for (i, row) in lab_map.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return Position(i as i32, j as i32);
            }
        }
    }
    panic!("No starting position");
}

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // Up
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
];

fn get_next(pos: &Position, dir_idx: usize) -> Position {
    let (dx, dy) = DIRECTIONS[dir_idx];
    Position(pos.0 + dx, pos.1 + dy)
}

fn turn(dir_idx: usize) -> usize {
    (dir_idx + 1) % 4 // Clockwise rotation
}

fn count_guard(lab_map: &[Vec<char>]) -> (usize, usize) {
    let rows = lab_map.len() as i32;
    let cols = lab_map[0].len() as i32;

    let mut visited: HashSet<Position> = HashSet::new();
    let mut obstructions: HashSet<Position> = HashSet::new();
    let mut loop_cache: HashMap<(Position, usize), bool> = HashMap::new();
    let mut curr_pos = find_start_position(lab_map);
    let mut curr_dir = 0; // Using integer direction index (0: Up, 1: Right, etc.)

    loop {
        visited.insert(curr_pos);

        let next = get_next(&curr_pos, curr_dir);

        if next.0 < 0 || next.1 < 0 || next.0 >= rows || next.1 >= cols {
            break; // Out of bounds
        }

        let next_ch = lab_map[next.0 as usize][next.1 as usize];
        if next_ch == '#' {
            curr_dir = turn(curr_dir);
            continue;
        }

        if !obstructions.contains(&next) {
            let cache_key = (curr_pos, curr_dir);
            loop_cache
                .entry(cache_key)
                .or_insert_with(|| check_for_loop(&curr_pos, curr_dir, lab_map, rows, cols));

            if *loop_cache.get(&cache_key).unwrap() {
                obstructions.insert(next);
            }
        }

        curr_pos = next;
    }

    (visited.len(), obstructions.len())
}

fn check_for_loop(
    start_pos: &Position,
    start_dir: usize,
    lab_map: &[Vec<char>],
    rows: i32,
    cols: i32,
) -> bool {
    let mut visited = HashSet::new();
    let mut curr_pos = *start_pos;
    let mut curr_dir = turn(start_dir);

    loop {
        visited.insert(curr_pos);

        let next = get_next(&curr_pos, curr_dir);
        if next.0 < 0 || next.1 < 0 || next.0 >= rows || next.1 >= cols {
            return false;
        }

        let next_ch = lab_map[next.0 as usize][next.1 as usize];
        if next_ch == '#' {
            curr_dir = turn(curr_dir);
            continue;
        }

        if next == *start_pos {
            return true; // Loop detected
        }

        curr_pos = next;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename).expect("Failed to read file");

    println!("{}", input);

    let lab_map = get_lab_map(&input);

    let (guard_positions, obstructions) = count_guard(&lab_map);
    println!(
        "guard positions: {}\nobstructions: {}",
        guard_positions, obstructions
    );
}
