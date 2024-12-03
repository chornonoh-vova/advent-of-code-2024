use std::path::Path;

fn read_instructions<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename)
}

fn get_muls(instructions: String) -> Vec<(i32, i32)> {
    let mul_pattern: Vec<char> = "mul(".chars().collect();
    let do_pattern: Vec<char> = "do()".chars().collect();
    let dont_pattern: Vec<char> = "don't()".chars().collect();

    let mut multiplications: Vec<(i32, i32)> = vec![];
    let chars: Vec<char> = instructions.chars().collect();

    let mut i = 0;

    let mut multiply = true;

    while i < chars.len() {
        if chars[i] == 'd' {
            println!("encountered 'd' at {0}", i);

            let mut j = i;

            while j - i < do_pattern.len() && chars[j] == do_pattern[j - i] {
                j += 1;
            }

            if (j - i) != do_pattern.len() {
                j = i;

                while j - i < dont_pattern.len() && chars[j] == dont_pattern[j - i] {
                    j += 1;
                }

                if (j - i) != dont_pattern.len() {
                    i = j;
                    continue;
                }

                println!("got 'don't()' from {0} to {1}", i, j);
                multiply = false;
                i += 1;
            } else {
                println!("got 'do()' from {0} to {1}", i, j);
                multiply = true;
                i += 1;
            }
        } else if chars[i] == 'm' {
            println!("encountered 'm' at {0}", i);

            let mut j = i;

            while j - i < mul_pattern.len() && chars[j] == mul_pattern[j - i] {
                j += 1;
            }

            if (j - i) != mul_pattern.len() {
                i = j;
                continue;
            }

            println!("got 'mul(' from {0} to {1}", i, j);

            let mut num1: Vec<char> = vec![];

            while chars[j].is_digit(10) {
                num1.push(chars[j]);
                j += 1;
            }

            if num1.len() > 3 || chars[j] != ',' {
                i = j;
                continue;
            }

            let n1 = num1.iter().collect::<String>().parse::<i32>();

            if n1.is_err() {
                i = j;
                continue;
            }

            let n1 = n1.unwrap();

            println!("got 'n1' {0}", n1);

            j += 1;

            let mut num2: Vec<char> = vec![];

            while chars[j].is_digit(10) {
                num2.push(chars[j]);
                j += 1;
            }

            if num2.len() > 3 || chars[j] != ')' {
                i = j;
                continue;
            }

            let n2 = num2.iter().collect::<String>().parse::<i32>();

            if n2.is_err() {
                i = j;
                continue;
            }

            let n2 = n2.unwrap();

            println!("got 'n2' {0}", n2);

            j += 1;

            if multiply {
                multiplications.push((n1, n2));
            }

            i = j;
        } else {
            i += 1;
        }
    }

    multiplications
}

fn calc_result(multiplications: Vec<(i32, i32)>) -> i32 {
    multiplications
        .iter()
        .fold(0, |acc, (n1, n2)| acc + (n1 * n2))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];

    let instructions =
        read_instructions(filename).expect("File exists, contains instructions to execute");

    println!("instructions: {0}", instructions);

    let multiplications = get_muls(instructions);

    println!("multiplications: {:?}", multiplications);

    let result = calc_result(multiplications);

    println!("result: {0}", result);
}
