use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Board {
    numbers: [usize; 25],
    seen: [bool; 25],
    won: bool,
}

impl Board {
    fn is_won(&self) -> bool {
        // check rows
        for i in 0..5 {
            if self.seen[5 * i]
                && self.seen[5 * i + 1]
                && self.seen[5 * i + 2]
                && self.seen[5 * i + 3]
                && self.seen[5 * i + 4]
            {
                return true;
            }
        }
        // check cols
        for i in 0..5 {
            if self.seen[i]
                && self.seen[5 + i]
                && self.seen[2 * 5 + i]
                && self.seen[3 * 5 + i]
                && self.seen[4 * 5 + i]
            {
                return true;
            }
        }

        false
    }
}

fn parse_drawn(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect_vec()
}

fn parse_games(input: &str) -> Vec<Board> {
    let mut games = Vec::new();

    for board_str in input.split("\n\n").skip(1) {
        let mut numbers = [0; 25];
        for (row_index, row) in board_str.lines().enumerate() {
            for (col_index, col) in row.split_whitespace().enumerate() {
                numbers[row_index * 5 + col_index] = col.parse().unwrap();
            }
        }
        games.push(Board {
            numbers,
            seen: [false; 25],
            won: false,
        })
    }

    games
}

pub fn p1(input: &str) -> String {
    let drawn = parse_drawn(input);
    let mut boards = parse_games(input);

    for number in drawn {
        for board in boards.iter_mut() {
            if let Some(index) = board.numbers.iter().position(|&num| num == number) {
                board.seen[index] = true;
                if board.is_won() {
                    return (board
                        .numbers
                        .iter()
                        .zip(board.seen.iter())
                        .filter(|&(_, seen)| !*seen)
                        .map(|(&num, _)| num)
                        .sum::<usize>()
                        * number)
                        .to_string();
                }
            }
        }
    }
    unreachable!()
}

pub fn p2(input: &str) -> String {
    let drawn = parse_drawn(input);
    let mut boards = parse_games(input);
    let mut last_board=boards[0];
    let mut last_number=0;

    for number in drawn {
        for board in boards.iter_mut() {
            if let Some(index) = board.numbers.iter().position(|&num| num == number) {
                board.seen[index] = true;
                if board.is_won() && !board.won {
                    board.won = true;
                    last_board = *board;
                    last_number = number;
                }
            }
        }
    }
    (last_board
        .numbers
        .iter()
        .zip(last_board.seen.iter())
        .filter(|&(_, seen)| !*seen)
        .map(|(&num, _)| num)
        .sum::<usize>()
        * last_number)
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../inputs/d04_example.txt")), "4512");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../inputs/d04.txt")), "38913");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../inputs/d04_example.txt")), "1924");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../inputs/d04.txt")), " ");
}
