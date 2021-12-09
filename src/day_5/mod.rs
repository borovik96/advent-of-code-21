use std::{num::ParseIntError, str::FromStr, vec};

use crate::read_input;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(" -> ").collect();

        let start_fromstr = coords[0].parse::<Point>()?;
        let end_fromstr = coords[1].parse::<Point>()?;

        Ok(Line {
            start: start_fromstr,
            end: end_fromstr,
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn get_input(path: &str) -> Vec<Line> {
    read_input::read_input(path)
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

type Board = Vec<Vec<u32>>;

fn draw_line(board: &mut Board, line: &Line) {
    let normal_vector = Point {
        x: if line.end.x > line.start.x {
            1
        } else if line.end.x == line.start.x {
            0
        } else {
            -1
        },
        y: if line.end.y > line.start.y {
            1
        } else if line.end.y == line.start.y {
            0
        } else {
            -1
        },
    };

    let mut current_x = line.start.x;
    let mut current_y = line.start.y;

    loop {
        let row = board.get_mut(current_y as usize).unwrap();

        row[current_x as usize] += 1;

        if line.end.x == current_x && line.end.y == current_y {
            break;
        }

        current_x += normal_vector.x;
        current_y += normal_vector.y;
    }
}

fn calc_score(board: &Board) -> u32 {
    let mut sum = 0;

    for column in board {
        for cell in column {
            if cell > &1 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn get_result_1() -> u32 {
    let lines = get_input("day_5/input-1.txt");

    let mut board: Board = vec![vec![0; 1000]; 1000];

    for line in lines {
        if line.start.x == line.end.x || line.start.y == line.end.y {
            draw_line(&mut board, &line)
        }
    }

    calc_score(&board)
}

struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn new(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { data }
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: num::Integer + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_i, row) in (&self.data).into_iter().enumerate() {
            for (i, cell) in row.into_iter().enumerate() {
                f.write_fmt(format_args!(
                    "{}",
                    if (cell).is_zero() {
                        ".".to_string()
                    } else {
                        cell.to_string()
                    }
                ))
                .unwrap();

                // if i != row.len() - 1 {
                //     f.write_str(" ").unwrap();
                // }
            }

            if row_i != self.data.len() - 1 {
                f.write_str("\n").unwrap();
            }
        }

        write!(f, "\n")
    }
}

const BOARD_SIZE: usize = 1000;

pub fn get_result_2() -> u32 {
    let lines = get_input("day_5/input-2.txt");

    let mut board: Board = vec![vec![0; BOARD_SIZE]; BOARD_SIZE];

    for line in lines {
        draw_line(&mut board, &line)
    }

    // println!("{}", Matrix::new(board.clone()));

    calc_score(&board)
}
