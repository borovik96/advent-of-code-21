use std::fs;

enum Direction {
    Up,
    Down,
    Forward,
}

type Move = (Direction, i32);

struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    aim: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_input(path: &str) -> Vec<Move> {
    let data = fs::read_to_string(path).unwrap();

    data.split('\n')
        .map(|n| {
            let string = Vec::from_iter(n.split(' ').into_iter());

            let direction = string[0];
            let count = string[1];

            let direction = match direction {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                _ => panic!("Unknown direction {}", direction),
            };

            (direction, count.parse::<i32>().unwrap())
        })
        .collect()
}

pub fn get_result_1() -> i32 {
    let input = get_input("./day_2/input-1.txt");

    let dest = calc_destination_1(input);

    dest.x * dest.y
}

pub fn get_result_2() -> i32 {
    let input = get_input("./day_2/input-2.txt");

    let dest = calc_destination_2(input);

    dest.x * dest.y
}

fn calc_destination_1(input: Vec<Move>) -> Point {
    let mut dest = Point { x: 0, y: 0 };
    for (mv, count) in input {
        match mv {
            Direction::Up => {
                dest.y -= count;
            }
            Direction::Down => {
                dest.y += count;
            }
            Direction::Forward => {
                dest.x += count;
            }
        }
    }

    dest
}

fn calc_destination_2(input: Vec<Move>) -> Point3 {
    let mut dest = Point3 { x: 0, y: 0, aim: 0 };
    for (mv, count) in input {
        match mv {
            Direction::Up => {
                dest.aim -= count;
            }
            Direction::Down => {
                dest.aim += count;
            }
            Direction::Forward => {
                dest.x += count;
                dest.y += dest.aim * count;
            }
        }
    }

    dest
}
