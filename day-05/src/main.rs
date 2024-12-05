use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn get_page_ordering(input: &str) -> HashMap<i32, HashSet<i32>> {
    let mut ordering: HashMap<i32, HashSet<i32>> = HashMap::new();

    let ordering_str = input
        .split_once("\n\n")
        .expect("Input must contain page ordering information")
        .0;

    ordering_str.split("\n").for_each(|line| {
        let (page1, page2) = line
            .split_once("|")
            .map(|(page1, page2)| (page1.parse::<i32>(), page2.parse::<i32>()))
            .expect("Line to contain 2 pages");

        if page1.is_ok() && page2.is_ok() {
            let page1 = page1.unwrap();
            let page2 = page2.unwrap();

            if !ordering.contains_key(&page2) {
                ordering.insert(page2, HashSet::new());
            }

            ordering.get_mut(&page2).unwrap().insert(page1);
        }
    });

    ordering
}

fn get_page_updates(input: &str) -> Vec<Vec<i32>> {
    let page_updates_str = input
        .split_once("\n\n")
        .expect("Input must contain page updates information")
        .1;

    page_updates_str
        .split("\n")
        .map(|line| {
            line.split(",")
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn is_valid_page_update(
    page_update: &Vec<i32>,
    page_ordering: &HashMap<i32, HashSet<i32>>,
) -> bool {
    let mut prev = HashSet::new();

    for page in page_update {
        if !prev.is_empty() {
            let empty = HashSet::new();
            let valid = page_ordering.get(page).unwrap_or(&empty);
            if !valid.is_superset(&prev) {
                return false;
            }
        }
        prev.insert(*page);
    }

    true
}

fn sort_invalid_page_update(
    page_update: &mut Vec<i32>,
    page_ordering: &HashMap<i32, HashSet<i32>>,
) {
    page_update.sort_by(|a, b| {
        let empty = HashSet::new();
        if page_ordering.get(&b).unwrap_or(&empty).contains(a) {
            return Ordering::Less;
        }
        if page_ordering.get(&a).unwrap_or(&empty).contains(b) {
            return Ordering::Greater;
        }

        Ordering::Equal
    });
}

fn bucket_page_updates(
    page_updates: &Vec<Vec<i32>>,
    page_ordering: &HashMap<i32, HashSet<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut valid = vec![];
    let mut invalid = vec![];

    for page_update in page_updates {
        if is_valid_page_update(page_update, page_ordering) {
            valid.push(page_update.clone());
        } else {
            invalid.push(page_update.clone());
        }
    }

    (valid, invalid)
}

fn get_middles_sum(page_updates: &Vec<Vec<i32>>) -> i32 {
    page_updates
        .into_iter()
        .map(|page_update| page_update[page_update.len() / 2])
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename).expect("File to contain input");

    let page_ordering = get_page_ordering(&input);
    println!("page ordering: {:?}", page_ordering);

    let page_updates = get_page_updates(&input);
    println!("page_updates: {:?}", page_updates);

    let (valid_page_updates, mut invalid_page_updates) =
        bucket_page_updates(&page_updates, &page_ordering);
    println!("valid page updates: {:?}", valid_page_updates);

    invalid_page_updates.iter_mut().for_each(|page_update| {
        sort_invalid_page_update(page_update, &page_ordering);
    });

    println!("invalid page updates: {:?}", invalid_page_updates);

    let valid_middles_sum = get_middles_sum(&valid_page_updates);
    println!("valid middles sum: {}", valid_middles_sum);

    let invalid_middles_sum = get_middles_sum(&invalid_page_updates);
    println!("invalid middles sum: {}", invalid_middles_sum)
}
