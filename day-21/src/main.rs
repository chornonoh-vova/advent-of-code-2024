use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_move(&self, p: Point, map: &HashMap<Point, u8>) -> Option<Point> {
        let next_p = match self {
            Self::North if p.y > 0 => Some(Point::new(p.x, p.y - 1)),
            Self::East => Some(Point::new(p.x + 1, p.y)),
            Self::South => Some(Point::new(p.x, p.y + 1)),
            Self::West if p.x > 0 => Some(Point::new(p.x - 1, p.y)),
            _ => None,
        };

        if map.contains_key(&next_p?) {
            next_p
        } else {
            None
        }
    }
}

impl From<&Direction> for u8 {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::North => b'^',
            Direction::East => b'>',
            Direction::South => b'v',
            Direction::West => b'<',
        }
    }
}

struct Keypad {
    map: HashMap<Point, u8>,
    positions: HashMap<u8, Point>,
}

impl Keypad {
    fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = (Point, u8)> + Clone,
        HashMap<Point, u8>: FromIterator<(Point, u8)>,
        HashMap<u8, Point>: FromIterator<(u8, Point)>,
    {
        Self {
            map: iter.clone().collect(),
            positions: iter.map(|(k, v)| (v, k)).collect(),
        }
    }

    const DOOR_KEYS: [(Point, u8); 11] = [
        (Point::new(0, 0), b'7'), //     0   1   2
        (Point::new(1, 0), b'8'), //   +---+---+---+
        (Point::new(2, 0), b'9'), // 0 | 7 | 8 | 9 |
        (Point::new(0, 1), b'4'), //   +---+---+---+
        (Point::new(1, 1), b'5'), // 1 | 4 | 5 | 6 |
        (Point::new(2, 1), b'6'), //   +---+---+---+
        (Point::new(0, 2), b'1'), // 2 | 1 | 2 | 3 |
        (Point::new(1, 2), b'2'), //   +---+---+---+
        (Point::new(2, 2), b'3'), // 3     | 0 | A |
        (Point::new(1, 3), b'0'), //       +---+---+
        (Point::new(2, 3), b'A'),
    ];

    const ROBOT_KEYS: [(Point, u8); 5] = [
        (Point::new(1, 0), b'^'), //     0   1   2
        (Point::new(2, 0), b'A'), //       +---+---+
        (Point::new(0, 1), b'<'), // 0     | ^ | A |
        (Point::new(1, 1), b'v'), //   +---+---+---+
        (Point::new(2, 1), b'>'), // 1 | < | v | > |
                                  //   +---+---+---+
    ];
}

struct Solution {
    door: Keypad,
    robot: Keypad,
    targets: Vec<(usize, Vec<u8>)>,
}

type Cache = HashMap<(u8, u8, usize), usize>;

impl Solution {
    fn traverse(&self, keypad: &Keypad, a: u8, b: u8, depth: usize, cache: &mut Cache) -> usize {
        if let Some(&result) = cache.get(&(a, b, depth)) {
            return result;
        }

        let from = keypad.positions[&a];
        let to = keypad.positions[&b];

        if depth == 0 {
            return to.distance(&from) as usize + 1;
        }

        let mut moves = Vec::new();

        if from.x < to.x {
            moves.extend([Direction::East].repeat((to.x - from.x) as usize));
        } else {
            moves.extend([Direction::West].repeat((from.x - to.x) as usize));
        }

        if from.y < to.y {
            moves.extend([Direction::South].repeat((to.y - from.y) as usize));
        } else {
            moves.extend([Direction::North].repeat((from.y - to.y) as usize));
        }

        let result = moves
            .iter()
            .permutations(moves.len())
            .filter_map(|moves| {
                let mut p = from;

                for &d in &moves {
                    p = d.next_move(p, &keypad.map)?;
                }

                Some(
                    [b'A']
                        .into_iter()
                        .chain(moves.into_iter().map(u8::from))
                        .chain([b'A'])
                        .tuple_windows()
                        .map(|(a, b)| self.traverse(&self.robot, a, b, depth - 1, cache))
                        .sum::<usize>(),
                )
            })
            .min()
            .expect("failed to find move set");

        cache.insert((a, b, depth), result);
        result
    }

    fn solve(&self, depth: usize) -> usize {
        let mut cache = Cache::default();

        self.targets
            .iter()
            .map(|(number, seq)| {
                number
                    * [b'A']
                        .iter()
                        .chain(seq.iter())
                        .tuple_windows()
                        .map(|(&a, &b)| self.traverse(&self.door, a, b, depth, &mut cache))
                        .sum::<usize>()
            })
            .sum::<usize>()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    let solution = Solution {
        door: Keypad::new(Keypad::DOOR_KEYS.into_iter()),
        robot: Keypad::new(Keypad::ROBOT_KEYS.into_iter()),
        targets: input
            .lines()
            .map(|line| {
                (
                    line.strip_suffix('A').unwrap().parse::<usize>().unwrap(),
                    line.bytes().collect::<Vec<_>>(),
                )
            })
            .collect(),
    };

    println!("part 1: {}", solution.solve(2));
    println!("part 2: {}", solution.solve(25));
}
