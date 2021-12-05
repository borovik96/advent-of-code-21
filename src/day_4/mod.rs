use crate::read_input;

#[derive(Clone)]
struct Cell {
    value: i32,
    is_marked: bool,
}

impl Cell {
    fn new(val: i32) -> Cell {
        Cell {
            value: val,
            is_marked: false,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_marked {
            write!(f, "({})", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Clone)]
struct Board {
    data: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Board {
        Board {
            data: Vec::<Vec<Cell>>::new(),
        }
    }

    fn is_win(&self) -> bool {
        let mut column_count = [0; 5];

        for row in &self.data {
            let mut row_count = 0;

            for (i, cell) in row.into_iter().enumerate() {
                if cell.is_marked {
                    row_count += 1;
                    column_count[i] += 1;

                    if row_count == row.len() || column_count[i] == row.len() {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn mark_num(&mut self, val: i32) {
        for row in &mut self.data {
            for cell in row {
                if cell.value == val {
                    cell.is_marked = true
                }
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_i, row) in (&self.data).into_iter().enumerate() {
            for (i, cell) in row.into_iter().enumerate() {
                f.write_fmt(format_args!("{}\t", cell)).unwrap();

                if i != row.len() - 1 {
                    f.write_str(" ").unwrap();
                }
            }

            if row_i != self.data.len() - 1 {
                f.write_str("\n").unwrap();
            }
        }

        write!(f, "\n")
    }
}

fn get_input(path: &str) -> (Vec<i32>, Vec<Board>) {
    let data: Vec<String> = read_input::read_input(path)
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let numbers = data[0]
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    let mut board = Board::new();

    for row in data[2..].into_iter() {
        if row.len() == 0 {
            continue;
        }

        let row: Vec<Cell> = row
            .trim()
            .split(' ')
            .filter_map(|n| {
                if n == "" {
                    return None;
                }

                Some(Cell::new(n.parse().unwrap()))
            })
            .collect();

        board.data.push(row);

        if board.data.len() == 5 {
            boards.push(board);
            board = Board::new()
        }
    }

    (numbers, boards)
}

fn get_score(n: i32, board: &Board) -> i32 {
    let mut marked_sum = 0;

    for row in &board.data {
        for cell in row {
            if !cell.is_marked {
                marked_sum += cell.value;
            }
        }
    }

    n * marked_sum
}

fn find_first_winner(nums: &Vec<i32>, boards: &mut Vec<Board>) -> (i32, Board) {
    for num in nums {
        for board in boards.into_iter() {
            board.mark_num(*num);

            if board.is_win() {
                return (*num, board.clone());
            }
        }
    }

    panic!("No winned board")
}

fn find_last_winner(nums: &Vec<i32>, boards: &mut Vec<Board>) -> (i32, Board) {
    let mut win_boards_count = 0;
    let boards_len = boards.len();

    for num in nums {
        for board in boards.into_iter() {
            if board.is_win() {
                continue;
            }

            board.mark_num(*num);

            if board.is_win() {
                win_boards_count += 1;

                if win_boards_count == boards_len {
                    return (*num, board.clone());
                }
            }
        }
    }

    panic!("No win board")
}

pub fn get_result_1() -> i32 {
    let mut input = get_input("day_4/input-1.txt");

    for (i, board) in input.1.iter().enumerate() {
        println!("#{}\n{}\n", i, &board)
    }

    let (num, winner) = find_first_winner(&input.0, &mut input.1);

    println!("{}", &winner);

    get_score(num, &winner)
}

pub fn get_result_2() -> i32 {
    let mut input = get_input("day_4/input-2.txt");

    for (i, board) in input.1.iter().enumerate() {
        println!("#{}\n{}\n", i, &board)
    }

    let (num, winner) = find_last_winner(&input.0, &mut input.1);

    println!("{}", &winner);

    get_score(num, &winner)
}
