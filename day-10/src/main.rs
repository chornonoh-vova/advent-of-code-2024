fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn parse_map(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn find_trailheads(map: &[Vec<i32>]) -> Vec<(i32, i32)> {
    let mut trailheads = vec![];

    for (i, row) in map.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == 0 {
                trailheads.push((i as i32, j as i32));
            }
        }
    }

    trailheads
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn trace_trailhead(map: &[Vec<i32>], scores: &mut Vec<(i32, i32)>, curr: &(i32, i32), height: i32) {
    if height == 9 {
        scores.push(*curr);
        return;
    }

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    for (di, dj) in DIRECTIONS {
        let next_i = curr.0 + di;
        let next_j = curr.1 + dj;

        if next_i >= 0
            && next_j >= 0
            && next_i < rows
            && next_j < cols
            && map[next_i as usize][next_j as usize] == height + 1
        {
            trace_trailhead(map, scores, &(next_i, next_j), height + 1);
        }
    }
}

fn trailhead_score(map: &[Vec<i32>], start: &(i32, i32)) -> usize {
    let mut scores: Vec<(i32, i32)> = vec![];

    trace_trailhead(map, &mut scores, start, 0);

    scores.len()
}

fn trailhead_scores(map: &[Vec<i32>], trailheads: &[(i32, i32)]) -> Vec<usize> {
    trailheads
        .iter()
        .map(|trailhead| trailhead_score(map, trailhead))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename)?;
    println!("{}", input);

    let map = parse_map(&input);

    let trailheads = find_trailheads(&map);
    println!("trailheads: {:?}", trailheads);
    let scores = trailhead_scores(&map, &trailheads);
    println!("scores: {:?}", scores);
    let sum: usize = scores.iter().sum();
    println!("sum: {}", sum);

    Ok(())
}
