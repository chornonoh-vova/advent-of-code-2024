use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self(x as i32, y as i32)
    }
}

struct BoxPosition {
    l: Position,
    r: Position,
}

#[derive(Debug)]
struct Warehouse {
    robot: Position,
    boxes_l: HashSet<Position>,
    boxes_r: HashSet<Position>,
    walls: HashSet<Position>,
    rows: usize,
    cols: usize,
}

impl Warehouse {
    fn move_robot(&mut self, dir: (i32, i32)) {
        let next_pos = Position(self.robot.0 + dir.0, self.robot.1 + dir.1);

        if self.boxes_l.contains(&next_pos) {
            let box_pos = BoxPosition {
                l: next_pos,
                r: Position(next_pos.0, next_pos.1 + 1),
            };
            if self.can_move_box(&box_pos, dir) {
                self.move_box(&box_pos, dir);
                self.robot = next_pos;
            }
        }

        if self.boxes_r.contains(&next_pos) {
            let box_pos = BoxPosition {
                l: Position(next_pos.0, next_pos.1 - 1),
                r: next_pos,
            };
            if self.can_move_box(&box_pos, dir) {
                self.move_box(&box_pos, dir);
                self.robot = next_pos;
            }
        }

        if !self.boxes_l.contains(&next_pos)
            && !self.boxes_r.contains(&next_pos)
            && !self.walls.contains(&next_pos)
        {
            self.robot = next_pos;
        }
    }

    fn can_move_box(&self, box_pos: &BoxPosition, dir: (i32, i32)) -> bool {
        let next_box_pos = BoxPosition {
            l: Position(box_pos.l.0 + dir.0, box_pos.l.1 + dir.1),
            r: Position(box_pos.r.0 + dir.0, box_pos.r.1 + dir.1),
        };

        if dir == LEFT {
            if self.walls.contains(&next_box_pos.l) {
                return false;
            }

            if self.boxes_r.contains(&next_box_pos.l) {
                return self.can_move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                );
            }

            true
        } else if dir == RIGHT {
            if self.walls.contains(&next_box_pos.r) {
                return false;
            }

            if self.boxes_l.contains(&next_box_pos.r) {
                return self.can_move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            true
        } else {
            if self.walls.contains(&next_box_pos.l) || self.walls.contains(&next_box_pos.r) {
                return false;
            }

            if self.boxes_l.contains(&next_box_pos.l) && self.boxes_r.contains(&next_box_pos.r) {
                return self.can_move_box(&next_box_pos, dir);
            }

            if self.boxes_l.contains(&next_box_pos.r) && !self.boxes_r.contains(&next_box_pos.l) {
                return self.can_move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            if !self.boxes_l.contains(&next_box_pos.r) && self.boxes_r.contains(&next_box_pos.l) {
                return self.can_move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                );
            }

            if self.boxes_l.contains(&next_box_pos.r) && !self.boxes_r.contains(&next_box_pos.l) {
                return self.can_move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            if self.boxes_l.contains(&next_box_pos.r) && self.boxes_r.contains(&next_box_pos.l) {
                return self.can_move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                ) && self.can_move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            true
        }
    }

    fn move_box(&mut self, box_pos: &BoxPosition, dir: (i32, i32)) {
        let next_box_pos = BoxPosition {
            l: Position(box_pos.l.0 + dir.0, box_pos.l.1 + dir.1),
            r: Position(box_pos.r.0 + dir.0, box_pos.r.1 + dir.1),
        };

        if dir == LEFT {
            if self.boxes_r.contains(&next_box_pos.l) {
                self.move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                );
            }
        } else if dir == RIGHT {
            if self.boxes_l.contains(&next_box_pos.r) {
                self.move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }
        } else {
            if self.boxes_l.contains(&next_box_pos.l) && self.boxes_r.contains(&next_box_pos.r) {
                self.move_box(&next_box_pos, dir);
            }

            if self.boxes_l.contains(&next_box_pos.r) && !self.boxes_r.contains(&next_box_pos.l) {
                self.move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            if !self.boxes_l.contains(&next_box_pos.r) && self.boxes_r.contains(&next_box_pos.l) {
                self.move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                );
            }

            if self.boxes_l.contains(&next_box_pos.r) && !self.boxes_r.contains(&next_box_pos.l) {
                self.move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }

            if self.boxes_l.contains(&next_box_pos.r) && self.boxes_r.contains(&next_box_pos.l) {
                self.move_box(
                    &BoxPosition {
                        l: Position(next_box_pos.l.0, next_box_pos.l.1 - 1),
                        r: next_box_pos.l,
                    },
                    dir,
                );
                self.move_box(
                    &BoxPosition {
                        l: next_box_pos.r,
                        r: Position(next_box_pos.r.0, next_box_pos.r.1 + 1),
                    },
                    dir,
                );
            }
        }

        self.boxes_l.remove(&box_pos.l);
        self.boxes_r.remove(&box_pos.r);

        self.boxes_l.insert(next_box_pos.l);
        self.boxes_r.insert(next_box_pos.r);
    }

    fn gps(&self) -> i32 {
        self.boxes_l.iter().map(|p| p.0 * 100 + p.1).sum()
    }
}

impl std::str::FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = Position(0, 0);
        let mut boxes_l = HashSet::new();
        let mut boxes_r = HashSet::new();
        let mut walls = HashSet::new();
        let mut rows = 0;
        let mut cols = 0;

        for (i, line) in s.lines().enumerate() {
            let line = line
                .replace("#", "##")
                .replace("O", "[]")
                .replace(".", "..")
                .replace("@", "@.");
            for (j, ch) in line.chars().enumerate() {
                if ch == '@' {
                    robot = Position::new(i, j);
                } else if ch == '[' {
                    boxes_l.insert(Position::new(i, j));
                } else if ch == ']' {
                    boxes_r.insert(Position::new(i, j));
                } else if ch == '#' {
                    walls.insert(Position::new(i, j));
                }
                cols = j + 1;
            }
            rows = i + 1;
        }

        Ok(Self {
            robot,
            boxes_l,
            boxes_r,
            walls,
            rows,
            cols,
        })
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = (0..self.rows)
            .map(|i| {
                (0..self.cols)
                    .map(|j| {
                        if self.robot == Position::new(i, j) {
                            '@'
                        } else if self.boxes_l.contains(&Position::new(i, j)) {
                            '['
                        } else if self.boxes_r.contains(&Position::new(i, j)) {
                            ']'
                        } else if self.walls.contains(&Position::new(i, j)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect();
        f.write_str(&map.join("\n"))
    }
}

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

fn move_robot(warehouse: &mut Warehouse, moves: &str) {
    for line in moves.lines() {
        for ch in line.chars() {
            println!("{}", ch);
            match ch {
                '^' => warehouse.move_robot(UP),
                'v' => warehouse.move_robot(DOWN),
                '<' => warehouse.move_robot(LEFT),
                '>' => warehouse.move_robot(RIGHT),
                _ => panic!("Invalid move instruction"),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let (map, moves) = input
        .split_once("\n\n")
        .expect("Have map and moves separated");
    let mut warehouse = map
        .parse::<Warehouse>()
        .expect("Failed to parse warehouse map");

    println!("starting:\n{}", warehouse);
    move_robot(&mut warehouse, moves);
    println!("finished:\n{}", warehouse);
    println!("GPS: {}", warehouse.gps());
}
