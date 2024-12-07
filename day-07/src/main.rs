use std::collections::HashMap;

#[derive(Debug)]
enum CalibrationError {
    ParseError(String),
    UnsupportedOperatorError,
}

impl std::fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "ParseError: {}", e),
            Self::UnsupportedOperatorError => write!(f, "UnsupportedOperatorError"),
        }
    }
}

impl std::error::Error for CalibrationError {}

struct CalibrationEquation {
    test_value: u64,
    nums: Vec<u64>,
}

impl CalibrationEquation {
    fn calculate(&self, operators: &str) -> Result<u64, CalibrationError> {
        assert_eq!(operators.len(), self.nums.len() - 1);

        let mut res = self.nums[0];

        for (i, op) in operators.chars().enumerate() {
            let next = self.nums[i + 1];
            res = match op {
                '+' => res + next,
                '*' => res * next,
                '|' => res * 10_u64.pow((next as f64).log10().ceil() as u32) + next,
                _ => return Err(CalibrationError::UnsupportedOperatorError),
            };
        }

        Ok(res)
    }
}

impl std::str::FromStr for CalibrationEquation {
    type Err = CalibrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value_str, nums_str) = s
            .split_once(": ")
            .ok_or(CalibrationError::ParseError("No : delimiter".to_string()))?;

        let test_value = test_value_str.parse::<u64>().map_err(|_| {
            CalibrationError::ParseError(format!("Failed to parse test_value {}", test_value_str))
        })?;

        let nums: Vec<u64> = nums_str
            .split_whitespace()
            .map(|num_str| {
                num_str.parse::<u64>().map_err(|_| {
                    CalibrationError::ParseError(format!("Invalid number: {}", num_str))
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { test_value, nums })
    }
}

fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

struct OperatorsCache {
    cache: HashMap<usize, Vec<String>>,
}

const VALID_OPERATORS: [char; 3] = ['+', '|', '*'];

impl OperatorsCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, length: usize) -> &[String] {
        self.cache.entry(length).or_insert_with(|| {
            let mut operators = Vec::with_capacity(VALID_OPERATORS.len().pow(length as u32));
            let mut prefix = String::from("");
            generate_operators(&mut operators, &mut prefix, length);

            operators
        })
    }
}

fn generate_operators(operators: &mut Vec<String>, prefix: &mut String, length: usize) {
    if prefix.len() == length {
        operators.push(prefix.clone());
        return;
    }

    for op in VALID_OPERATORS {
        prefix.push(op);
        generate_operators(operators, prefix, length);
        prefix.pop();
    }
}

fn total_calibration(input: &str) -> Result<u64, CalibrationError> {
    let mut op_cache = OperatorsCache::new();

    input
        .lines()
        .map(|line| {
            let equation = line.parse::<CalibrationEquation>()?;

            for operators in op_cache.get(equation.nums.len() - 1) {
                if equation.test_value == equation.calculate(operators)? {
                    return Ok(equation.test_value);
                }
            }

            Ok(0)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = read_input(filename)?;

    let result = total_calibration(&input)?;
    println!("total calibration result: {}", result);

    Ok(())
}
