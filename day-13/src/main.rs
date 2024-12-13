#[derive(Debug)]
struct Button {
    dx: i64,
    dy: i64,
}

impl std::str::FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(", ").ok_or("Missing button delimiter")?;

        let dx = x_str
            .strip_prefix("X")
            .ok_or("Missing dx prefix")?
            .parse::<i64>()
            .map_err(|_| "Invalid dx")?;

        let dy = y_str
            .strip_prefix("Y")
            .ok_or("Missing dy prefix")?
            .parse::<i64>()
            .map_err(|_| "Invalid dy")?;

        Ok(Self { dx, dy })
    }
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Prize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(", ").ok_or("Missing prize delimiter")?;

        let x = x_str
            .strip_prefix("X=")
            .ok_or("Missing x prefix")?
            .parse::<i64>()
            .map_err(|_| "Invalid x")?;

        let y = y_str
            .strip_prefix("Y=")
            .ok_or("Missing y prefix")?
            .parse::<i64>()
            .map_err(|_| "Invalid y")?;

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl std::str::FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() != 3 {
            return Err("Invalid length".to_string());
        }

        let button_a = lines[0]
            .strip_prefix("Button A: ")
            .ok_or("Missing button a prefix")?
            .parse::<Button>()?;

        let button_b = lines[1]
            .strip_prefix("Button B: ")
            .ok_or("Missing button b prefix")?
            .parse::<Button>()?;

        let prize = lines[2]
            .strip_prefix("Prize: ")
            .ok_or("Missing prize prefix")?
            .parse::<Prize>()?;

        Ok(Self {
            button_a,
            button_b,
            prize,
        })
    }
}

fn parse_machines(input: &str) -> Result<Vec<Machine>, String> {
    input.split("\n\n").map(|s| s.parse::<Machine>()).collect()
}

const BUTTON_A_COST: i64 = 3;
const BUTTON_B_COST: i64 = 1;

fn price_to_win(machine: &Machine, offset: Option<i64>) -> Option<i64> {
    let a_x = machine.button_a.dx;
    let a_y = machine.button_a.dy;

    let b_x = machine.button_b.dx;
    let b_y = machine.button_b.dy;

    let p_x = machine.prize.x + offset.unwrap_or(0);
    let p_y = machine.prize.y + offset.unwrap_or(0);

    let a = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x);
    let b = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);

    if (a_x * a + b_x * b, a_y * a + b_y * b) == (p_x, p_y) {
        Some(a * BUTTON_A_COST + b * BUTTON_B_COST)
    } else {
        None
    }
}

fn total_price_to_win(machines: &[Machine], offset: Option<i64>) -> i64 {
    machines
        .iter()
        .filter_map(|m| price_to_win(m, offset))
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let machines = parse_machines(&input).expect("Failed to parse machines");

    println!(
        "part 1 total price: {}",
        total_price_to_win(&machines, None)
    );
    println!(
        "part 2 total price: {}",
        total_price_to_win(&machines, Some(10000000000000))
    );
}
