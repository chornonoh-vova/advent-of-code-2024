use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_reports(lines: io::Lines<BufReader<File>>) -> impl Iterator<Item = Vec<i32>> {
    lines.filter_map(|line| {
        line.ok().map(|l| {
            l.split_whitespace()
                .filter_map(|level| level.parse::<i32>().ok())
                .collect()
        })
    })
}

fn is_safe_report(report: &[i32]) -> bool {
    is_decreasing_report(report) || is_increasing_report(report)
}

fn is_safe_increasing(diff: &i32) -> bool {
    (1..=3).contains(diff)
}

fn is_increasing_report(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|w| is_safe_increasing(&(w[1] - w[0])))
}

fn is_safe_decreasing(diff: &i32) -> bool {
    (-3..=-1).contains(diff)
}

fn is_decreasing_report(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|w| is_safe_decreasing(&(w[1] - w[0])))
}

fn is_safe_with_tolerance(report: &[i32]) -> bool {
    if is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_safe_report(&modified) {
            return true;
        }
    }
    false
}

fn count_safe_reports(reports: impl Iterator<Item = Vec<i32>>) -> usize {
    reports.filter(|r| is_safe_with_tolerance(r)).count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    match read_lines(filename) {
        Ok(lines) => {
            let reports = get_reports(lines);
            let safe = count_safe_reports(reports);
            println!("{0}", safe);
        }
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}
