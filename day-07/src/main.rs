use std::str::FromStr;

struct CalibrationEquation {
    pub test_value: usize,
    pub nums: Vec<usize>,
}

impl FromStr for CalibrationEquation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value_str, nums_str) = s.split_once(": ").ok_or("No : delimiter")?;

        let test_value = test_value_str
            .parse::<usize>()
            .map_err(|_| "Failed to parse test_value")?;

        let nums = nums_str
            .split(" ")
            .filter_map(|num_str| num_str.parse::<usize>().ok())
            .collect();

        Ok(Self { test_value, nums })
    }
}

impl CalibrationEquation {
    fn calculate(&self, operators: &[char]) -> usize {
        println!("calculating {:?} {:?}", self.nums, operators);
        let mut result = self.nums[0];

        for i in 1..self.nums.len() {
            match operators[i - 1] {
                '+' => result += self.nums[i],
                '*' => result *= self.nums[i],
                '|' => {
                    result = (result.to_string() + self.nums[i].to_string().as_str())
                        .parse::<usize>()
                        .unwrap()
                }
                _ => panic!("Unsupported operator"),
            }
        }

        println!("result: {}", result);

        result
    }
}

fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

fn generate_operators(operators: &mut Vec<Vec<char>>, prefix: Vec<char>, length: usize) {
    if prefix.len() == length {
        operators.push(prefix);
        return;
    }

    let mut left = prefix.clone();
    left.push('+');
    generate_operators(operators, left, length);

    let mut middle = prefix.clone();
    middle.push('|');
    generate_operators(operators, middle, length);

    let mut right = prefix;
    right.push('*');
    generate_operators(operators, right, length);
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename).map_err(|_| "Failed to read file")?;

    let mut res = 0;

    for line in input.lines() {
        let equation = line.parse::<CalibrationEquation>()?;
        let mut operators: Vec<Vec<char>> = Vec::new();
        generate_operators(&mut operators, Vec::new(), equation.nums.len() - 1);

        for op in operators {
            if equation.calculate(&op) == equation.test_value {
                res += equation.test_value;
                break;
            }
        }
    }

    println!("total calibration result: {}", res);

    Ok(())
}
