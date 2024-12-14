#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn x(&self) -> usize {
        self.position.0 as usize
    }

    fn y(&self) -> usize {
        self.position.1 as usize
    }

    fn advance(&mut self, rows: usize, cols: usize) {
        let next = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        let rows = rows as i32;
        let cols = cols as i32;

        self.position.0 = (next.0 % cols + cols) % cols;
        self.position.1 = (next.1 % rows + rows) % rows;
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

struct Matrix {
    robots: Vec<Robot>,
    map: Vec<Vec<usize>>,
    visual_map: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(input: &str, rows: usize, cols: usize) -> Result<Self, String> {
        let robots: Vec<Robot> = input
            .lines()
            .map(|l| l.parse::<Robot>())
            .collect::<Result<_, _>>()?;
        let map = Matrix::get_map(&robots, rows, cols);
        let visual_map = Matrix::get_visual_map(&map);
        Ok(Self {
            robots,
            rows,
            cols,
            map,
            visual_map,
        })
    }

    fn get_map(robots: &[Robot], rows: usize, cols: usize) -> Vec<Vec<usize>> {
        let mut map = vec![vec![0; cols]; rows];
        for robot in robots.iter() {
            let old = map[robot.y()][robot.x()];
            map[robot.y()][robot.x()] = old + 1;
        }
        map
    }

    fn get_visual_map(map: &[Vec<usize>]) -> Vec<Vec<char>> {
        map.iter()
            .map(|l| l.iter().map(|n| if *n > 0 { '#' } else { ' ' }).collect())
            .collect()
    }

    fn advance_robots(&mut self) {
        for robot in self.robots.iter_mut() {
            let (old_x, old_y) = (robot.x(), robot.y());
            robot.advance(self.rows, self.cols);
            let (new_x, new_y) = (robot.x(), robot.y());

            self.map[old_y][old_x] -= 1;
            if self.map[old_y][old_x] == 0 {
                self.visual_map[old_y][old_x] = ' ';
            }
            self.map[new_y][new_x] += 1;
            if self.map[new_y][new_x] > 0 {
                self.visual_map[new_y][new_x] = '#';
            }
        }
    }

    fn has_easter_egg(&self) -> bool {
        let mut j = 0;

        for line in &self.visual_map {
            if j == PATTERN.len() {
                return true;
            }

            let line_str: String = line.iter().collect();
            if line_str.contains(PATTERN[j]) {
                j += 1;
            }
        }

        j == PATTERN.len()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = self
            .visual_map
            .iter()
            .map(|line| -> String { line.iter().collect() })
            .collect();
        f.write_str(&map.join("\n"))
    }
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

const MAX_ITERATIONS: usize = 20000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let cols = &args[2].parse::<usize>().expect("Failed to parse cols");
    let rows = &args[3].parse::<usize>().expect("Failed to parse rows");

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut matrix = Matrix::new(&input, *rows, *cols).expect("Failed to parse input");

    let mut cnt = 0;

    loop {
        matrix.advance_robots();
        cnt += 1;
        if matrix.has_easter_egg() {
            println!("{}", matrix);
            println!("cnt: {}", cnt);
            break;
        }

        if cnt > MAX_ITERATIONS {
            eprintln!("Couldn't find easter egg");
            break;
        }
    }
}
