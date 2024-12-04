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

fn count_xmas(search_matrix: &Vec<Vec<char>>) -> usize {
    let mut cnt = 0;

    let trace_xmas = |path: &[(i32, i32)], start_i: &i32, start_j: &i32| -> bool {
        let mut k = 0;

        let mut i = *start_i;
        let mut j = *start_j;

        while i >= 0
            && j >= 0
            && i < search_matrix.len() as i32
            && j < search_matrix[i as usize].len() as i32
            && k < XMAS.len()
            && search_matrix[i as usize][j as usize] == XMAS[k]
        {
            k += 1;
            if k != XMAS.len() {
                i = *start_i + path[k].0;
                j = *start_j + path[k].1;
            }
        }

        k == XMAS.len()
    };

    for i in 0..(search_matrix.len() as i32) {
        for j in 0..(search_matrix[i as usize].len() as i32) {
            if trace_xmas(TOP, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(TOP_RIGHT, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(RIGHT, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(BOTTOM_RIGHT, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(BOTTOM, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(BOTTOM_LEFT, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(LEFT, &i, &j) {
                cnt += 1;
            }
            if trace_xmas(TOP_LEFT, &i, &j) {
                cnt += 1;
            }
        }
    }

    cnt
}

const PATTERN_1: &[char] = &['M', 'S', 'M', 'S'];
const PATTERN_2: &[char] = &['M', 'M', 'S', 'S'];
const PATTERN_3: &[char] = &['S', 'S', 'M', 'M'];
const PATTERN_4: &[char] = &['S', 'M', 'S', 'M'];
const IDX_PATTERN: &[(i32, i32)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];

fn count_x_mas(search_matrix: &Vec<Vec<char>>) -> usize {
    let mut cnt = 0;

    let match_x_mas = |pattern: &[char], center_i: &i32, center_j: &i32| -> bool {
        let mut k = 0;

        let mut i = *center_i + IDX_PATTERN[0].0;
        let mut j = *center_j + IDX_PATTERN[0].1;

        while i >= 0
            && j >= 0
            && i < search_matrix.len() as i32
            && j < search_matrix[i as usize].len() as i32
            && k < pattern.len()
            && search_matrix[i as usize][j as usize] == pattern[k]
        {
            k += 1;
            if k != pattern.len() {
                i = *center_i + IDX_PATTERN[k].0;
                j = *center_j + IDX_PATTERN[k].1;
            }
        }

        k == pattern.len()
    };

    for i in 0..(search_matrix.len() as i32) {
        for j in 0..(search_matrix[i as usize].len() as i32) {
            if search_matrix[i as usize][j as usize] != 'A' {
                continue;
            }

            if match_x_mas(PATTERN_1, &i, &j) {
                cnt += 1;
            }

            if match_x_mas(PATTERN_2, &i, &j) {
                cnt += 1;
            }

            if match_x_mas(PATTERN_3, &i, &j) {
                cnt += 1;
            }

            if match_x_mas(PATTERN_4, &i, &j) {
                cnt += 1;
            }
        }
    }

    cnt
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let word_search = read_word_search(&filename).expect("File to exist and contain word search");
    let search_matrix = get_search_matrix(&word_search);
    let xmas = count_xmas(&search_matrix);
    println!("xmas: {}", xmas);
    let x_mas = count_x_mas(&search_matrix);
    println!("x-mas: {}", x_mas);
}
