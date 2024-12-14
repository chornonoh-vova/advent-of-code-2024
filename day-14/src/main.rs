#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn advance(&mut self, rows: i32, cols: i32) {
        let next = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        self.position.0 = if next.0 >= cols {
            next.0 - cols
        } else if next.0 < 0 {
            cols + next.0
        } else {
            next.0
        };

        self.position.1 = if next.1 >= rows {
            next.1 - rows
        } else if next.1 < 0 {
            rows + next.1
        } else {
            next.1
        };
    }
}

impl std::str::FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_str, v_str) = s.split_once(" ").ok_or("Missing separator")?;

        let (p_x_str, p_y_str) = p_str
            .strip_prefix("p=")
            .ok_or("Missing position prefix")?
            .split_once(",")
            .ok_or("Missing position separator")?;
        let p_x = p_x_str
            .parse::<i32>()
            .map_err(|_| "Invalid x position".to_string())?;
        let p_y = p_y_str
            .parse::<i32>()
            .map_err(|_| "Invalid y position".to_string())?;

        let (v_x_str, v_y_str) = v_str
            .strip_prefix("v=")
            .ok_or("Missing velocity prefix")?
            .split_once(",")
            .ok_or("Missing velocity separator")?;
        let v_x = v_x_str
            .parse::<i32>()
            .map_err(|_| "Invalid x velocity".to_string())?;
        let v_y = v_y_str
            .parse::<i32>()
            .map_err(|_| "Invalid y velocity".to_string())?;

        Ok(Self {
            position: (p_x, p_y),
            velocity: (v_x, v_y),
        })
    }
}

fn parse_robots(input: &str) -> Result<Vec<Robot>, String> {
    input.lines().map(|l| l.parse::<Robot>()).collect()
}

fn advance_robots(robots: &mut Vec<Robot>, rows: i32, cols: i32) {
    for robot in robots.iter_mut() {
        robot.advance(rows, cols);
    }
}

fn get_robot_positions(robots: &[Robot], rows: i32, cols: i32) -> String {
    let mut matrix: Vec<String> = (0..rows).map(|_| " ".repeat(cols as usize)).collect();
    for robot in robots {
        matrix[robot.position.1 as usize].replace_range(
            (robot.position.0 as usize)..(robot.position.0 as usize + 1),
            "#",
        );
    }
    matrix.join("\n")
}

const PATTERN: [&str; 33] = [
    "###############################",
    "#                             #",
    "#                             #",
    "#                             #",
    "#                             #",
    "#              #              #",
    "#             ###             #",
    "#            #####            #",
    "#           #######           #",
    "#          #########          #",
    "#            #####            #",
    "#           #######           #",
    "#          #########          #",
    "#         ###########         #",
    "#        #############        #",
    "#          #########          #",
    "#         ###########         #",
    "#        #############        #",
    "#       ###############       #",
    "#      #################      #",
    "#        #############        #",
    "#       ###############       #",
    "#      #################      #",
    "#     ###################     #",
    "#    #####################    #",
    "#             ###             #",
    "#             ###             #",
    "#             ###             #",
    "#                             #",
    "#                             #",
    "#                             #",
    "#                             #",
    "###############################",
];

fn has_easter_egg(matrix: &str) -> bool {
    let mut contains = false;
    let mut j = 0;

    for line in matrix.lines() {
        if j == PATTERN.len() {
            return true;
        }

        contains = if line.contains(PATTERN[j]) {
            j += 1;
            true
        } else {
            false
        };
    }

    contains
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let cols = &args[2].parse::<i32>().expect("Failed to parse cols");
    let rows = &args[3].parse::<i32>().expect("Failed to parse rows");

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut robots = parse_robots(&input).expect("Failed to parse robots");

    let mut cnt = 0;

    loop {
        advance_robots(&mut robots, *rows, *cols);
        cnt += 1;
        let matrix = get_robot_positions(&robots, *rows, *cols);
        if has_easter_egg(&matrix) {
            println!("{}", matrix);
            println!("cnt: {}", cnt);
            break;
        }

        if cnt > 20000 {
            eprintln!("Couldn't find easter egg");
            break;
        }
    }
}
