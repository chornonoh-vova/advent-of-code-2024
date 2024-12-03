fn read_instructions<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn find_num_end(instructions: &str, start: usize) -> usize {
    instructions[start..]
        .find(|c: char| !c.is_digit(10))
        .map_or(instructions.len(), |rel_pos| start + rel_pos)
}

fn get_multiplications(instructions: &str) -> Vec<(i32, i32)> {
    let mut multiplications: Vec<(i32, i32)> = vec![];
    let mut multiply = true;

    let mut i = 0;

    while i < instructions.len() {
        if instructions[i..].starts_with("do()") {
            multiply = true;
            i += "do()".len()
        } else if instructions[i..].starts_with("don't()") {
            multiply = false;
            i += "don't()".len()
        } else if instructions[i..].starts_with("mul(") {
            let n1_start = i + "mul(".len();
            let n1_end = find_num_end(instructions, n1_start);
            if n1_end >= instructions.len() || instructions[n1_end..].chars().next() != Some(',') {
                i = n1_end;
                continue;
            }

            let n2_start = n1_end + 1;
            let n2_end = find_num_end(instructions, n2_start);
            if n2_end >= instructions.len() || instructions[n2_end..].chars().next() != Some(')') {
                i = n2_end;
                continue;
            }

            let n1 = instructions[n1_start..n1_end].parse::<i32>().unwrap_or(0);
            let n2 = instructions[n2_start..n2_end].parse::<i32>().unwrap_or(0);

            if multiply {
                multiplications.push((n1, n2));
            }
            i = n2_end + 1;
        } else {
            i += 1;
        }
    }

    multiplications
}

fn calc_result(multiplications: Vec<(i32, i32)>) -> i32 {
    multiplications.iter().map(|(n1, n2)| n1 * n2).sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];

    let instructions =
        read_instructions(filename).expect("File exists, contains instructions to execute");

    println!("instructions: {0}", instructions);

    let multiplications = get_multiplications(instructions.as_str());

    println!("multiplications: {:?}", multiplications);

    let result = calc_result(multiplications);

    println!("result: {0}", result);
}
