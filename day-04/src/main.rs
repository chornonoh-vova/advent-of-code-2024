fn read_word_search<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn get_search_matrix(word_search: &str) -> Vec<Vec<char>> {
    word_search
        .split("\n")
        .map(|w| w.chars().collect())
        .collect()
}

const XMAS: &[char] = &['X', 'M', 'A', 'S'];

const TOP: &[(i32, i32)] = &[(0, 0), (0, -1), (0, -2), (0, -3)];
const TOP_RIGHT: &[(i32, i32)] = &[(0, 0), (1, -1), (2, -2), (3, -3)];
const RIGHT: &[(i32, i32)] = &[(0, 0), (1, 0), (2, 0), (3, 0)];
const BOTTOM_RIGHT: &[(i32, i32)] = &[(0, 0), (1, 1), (2, 2), (3, 3)];
const BOTTOM: &[(i32, i32)] = &[(0, 0), (0, 1), (0, 2), (0, 3)];
const BOTTOM_LEFT: &[(i32, i32)] = &[(0, 0), (-1, 1), (-2, 2), (-3, 3)];
const LEFT: &[(i32, i32)] = &[(0, 0), (-1, 0), (-2, 0), (-3, 0)];
const TOP_LEFT: &[(i32, i32)] = &[(0, 0), (-1, -1), (-2, -2), (-3, -3)];
const DIRECTIONS: &[&[(i32, i32)]] = &[
    TOP,
    TOP_RIGHT,
    RIGHT,
    BOTTOM_RIGHT,
    BOTTOM,
    BOTTOM_LEFT,
    LEFT,
    TOP_LEFT,
];

fn count_xmas(search_matrix: &[Vec<char>]) -> usize {
    let mut cnt = 0;

    let rows = search_matrix.len() as i32;
    let cols = search_matrix[0].len() as i32;

    let trace_xmas = |path: &[(i32, i32)], start_i: usize, start_j: usize| -> bool {
        for (k, &(di, dj)) in path.iter().enumerate() {
            let new_i = start_i as i32 + di;
            let new_j = start_j as i32 + dj;
            if new_i < 0
                || new_j < 0
                || new_i >= rows
                || new_j >= cols
                || search_matrix[new_i as usize][new_j as usize] != XMAS[k]
            {
                return false;
            }
        }

        true
    };

    for (i, row) in search_matrix.iter().enumerate() {
        for j in 0..row.len() {
            for direction in DIRECTIONS {
                if trace_xmas(direction, i, j) {
                    cnt += 1
                }
            }
        }
    }

    cnt
}

const PATTERN_1: &[char] = &['M', 'S', 'M', 'S'];
const PATTERN_2: &[char] = &['M', 'M', 'S', 'S'];
const PATTERN_3: &[char] = &['S', 'S', 'M', 'M'];
const PATTERN_4: &[char] = &['S', 'M', 'S', 'M'];
const PATTERNS: &[&[char]] = &[PATTERN_1, PATTERN_2, PATTERN_3, PATTERN_4];
const IDX_PATTERN: &[(i32, i32)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];

fn count_x_mas(search_matrix: &[Vec<char>]) -> usize {
    let mut cnt = 0;

    let rows = search_matrix.len() as i32;
    let cols = search_matrix[0].len() as i32;

    let match_x_mas = |pattern: &[char], center_i: usize, center_j: usize| -> bool {
        for (k, &(di, dj)) in IDX_PATTERN.iter().enumerate() {
            let new_i = center_i as i32 + di;
            let new_j = center_j as i32 + dj;
            if new_i < 0
                || new_j < 0
                || new_i >= rows
                || new_j >= cols
                || search_matrix[new_i as usize][new_j as usize] != pattern[k]
            {
                return false;
            }
        }

        true
    };

    for (i, row) in search_matrix.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem != 'A' {
                continue;
            }

            for pattern in PATTERNS {
                if match_x_mas(pattern, i, j) {
                    cnt += 1;
                }
            }
        }
    }

    cnt
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let word_search = read_word_search(filename).expect("File to exist and contain word search");

    let search_matrix = get_search_matrix(&word_search);

    let xmas = count_xmas(&search_matrix);
    println!("xmas: {}", xmas);

    let x_mas = count_x_mas(&search_matrix);
    println!("x-mas: {}", x_mas);
}
