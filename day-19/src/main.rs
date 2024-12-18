use std::collections::HashMap;

struct Combinator {
    patterns: Vec<String>,
    possible: HashMap<String, usize>,
}

impl Combinator {
    fn new(patterns: Vec<String>) -> Self {
        let possible = HashMap::new();
        Self { patterns, possible }
    }

    fn count_possible(&mut self, design: &str) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&possible) = self.possible.get(design) {
            return possible;
        }

        let patterns = self.patterns.clone();

        let mut count_possible = 0;
        for pat in &patterns {
            if design.starts_with(pat) {
                let remaining = &design[pat.len()..];
                count_possible += self.count_possible(remaining);
            }
        }

        self.possible.insert(design.to_string(), count_possible);
        count_possible
    }
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, designs) = input.split_once("\n\n").expect("Failed to find delimiter");

    let patterns = patterns.split(", ").map(|p| p.to_string()).collect();
    let designs = designs.lines().map(|d| d.to_string()).collect();

    (patterns, designs)
}

fn count_possible(designs: &[String], comb: &mut Combinator) -> usize {
    designs
        .iter()
        .filter(|design| comb.count_possible(design) != 0)
        .count()
}

fn count_possible_ways(designs: &[String], comb: &mut Combinator) -> usize {
    designs
        .iter()
        .map(|design| comb.count_possible(design))
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    let (patterns, designs) = parse_input(&input);
    let mut comb = Combinator::new(patterns);

    println!("possible count: {}", count_possible(&designs, &mut comb));
    println!(
        "possible ways count: {}",
        count_possible_ways(&designs, &mut comb)
    );
}
