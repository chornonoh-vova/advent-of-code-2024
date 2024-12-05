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

type PageOrdering = HashMap<i32, HashSet<i32>>;
type PageUpdates = Vec<Vec<i32>>;

fn get_page_ordering(input: &str) -> Result<PageOrdering, String> {
    let mut ordering: PageOrdering = HashMap::new();

    let ordering_str = input
        .split_once("\n\n")
        .ok_or("Input must contain page ordering information")?
        .0;

    for line in ordering_str.lines() {
        let (page1, page2) = line
            .split_once("|")
            .ok_or("Line must contain a | separator")?;

        let page1 = page1.parse::<i32>().map_err(|_| "Invalid page number")?;
        let page2 = page2.parse::<i32>().map_err(|_| "Invalid page number")?;

        ordering.entry(page2).or_default().insert(page1);
    }

    Ok(ordering)
}

fn get_page_updates(input: &str) -> Result<PageUpdates, String> {
    let page_updates_str = input
        .split_once("\n\n")
        .ok_or("Input must contain page updates information")?
        .1;

    let mut page_updates = vec![];

    for line in page_updates_str.lines() {
        let page_update = line
            .split(",")
            .filter_map(|num_str| num_str.parse::<i32>().ok())
            .collect();

        page_updates.push(page_update);
    }

    Ok(page_updates)
}

fn is_valid_page_update(page_update: &[i32], page_ordering: &PageOrdering) -> bool {
    let mut prev = HashSet::new();

    for page in page_update {
        if !prev.is_empty() {
            let is_valid = page_ordering
                .get(page)
                .map(|v| v.is_superset(&prev))
                .unwrap_or(false);
            if !is_valid {
                return false;
            }
        }
        prev.insert(*page);
    }

    true
}

fn sort_invalid_page_update(page_update: &mut [i32], page_ordering: &PageOrdering) {
    page_update.sort_by(|a, b| {
        if page_ordering.get(b).map(|o| o.contains(a)).unwrap_or(false) {
            return Ordering::Less;
        }
        if page_ordering.get(a).map(|o| o.contains(b)).unwrap_or(false) {
            return Ordering::Greater;
        }

        Ordering::Equal
    });
}

fn bucket_page_updates(
    page_updates: &[Vec<i32>],
    page_ordering: &PageOrdering,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut valid = vec![];
    let mut invalid = vec![];

    for page_update in page_updates.iter().cloned() {
        if is_valid_page_update(&page_update, page_ordering) {
            valid.push(page_update);
        } else {
            invalid.push(page_update);
        }
    }

    (valid, invalid)
}

fn get_middles_sum(page_updates: &[Vec<i32>]) -> i32 {
    page_updates
        .iter()
        .map(|page_update| page_update.get(page_update.len() / 2).unwrap_or(&0))
        .sum()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename).map_err(|e| e.to_string())?;

    let page_ordering = get_page_ordering(&input)?;
    println!("page ordering: {:?}", page_ordering);

    let page_updates = get_page_updates(&input)?;
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
    println!("invalid middles sum: {}", invalid_middles_sum);

    Ok(())
}
