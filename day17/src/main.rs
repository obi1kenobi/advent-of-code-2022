use std::{env, fs, collections::VecDeque};

#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reversed_args: Vec<_> = args.iter().map(|x| x.as_str()).rev().collect();

    reversed_args
        .pop()
        .expect("Expected the executable name to be the first argument, but was missing");

    let part = reversed_args.pop().expect("part number");
    let input_file = reversed_args.pop().expect("input file");
    let content = fs::read_to_string(input_file).unwrap();

    let input_data = content.trim_end().chars().map(|c| {
        match c {
            '<' => Move::Left,
            '>' => Move::Right,
            c => unreachable!("move {c}"),
        }
    }).collect_vec();

    match part {
        "1" => {
            let result = solve_part1(&input_data);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(&input_data);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

const CHAMBER_WIDTH: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Row,
    Plus,
    Angle,
    Column,
    Square,
}

impl Piece {
    #[rustfmt::skip]
    const SHAPES: [[[bool; 4]; 4]; 5] = [
        // Vertically flipped, so that the [0] row of each shape is down.
        [
            // Row
            [true; 4],
            [false; 4],
            [false; 4],
            [false; 4],
        ],
        [
            // Plus
            [false, true, false, false],
            [true,  true,  true, false],
            [false, true, false, false],
            [false; 4],
        ],
        [
            // Angle
            [true,  true,  true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false; 4],
        ],
        [
            // Column
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ],
        [
            // Square
            [true, true, false, false],
            [true, true, false, false],
            [false; 4],
            [false; 4],
        ],
    ];

    const ALL_PIECES: [Piece; 5] = [
        Piece::Row,
        Piece::Plus,
        Piece::Angle,
        Piece::Column,
        Piece::Square,
    ];

    const fn shape(&self) -> [[bool; 4]; 4] {
        let idx = match self {
            Piece::Row => 0,
            Piece::Plus => 1,
            Piece::Angle => 2,
            Piece::Column => 3,
            Piece::Square => 4,
        };

        Self::SHAPES[idx]
    }

    const fn width(&self) -> usize {
        match self {
            Piece::Row => 4,
            Piece::Plus => 3,
            Piece::Angle => 3,
            Piece::Column => 1,
            Piece::Square => 2,
        }
    }

    const fn height(&self) -> usize {
        match self {
            Piece::Row => 1,
            Piece::Plus => 3,
            Piece::Angle => 3,
            Piece::Column => 4,
            Piece::Square => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    occupied: VecDeque<[bool; CHAMBER_WIDTH]>,
    spawn_height: usize,
    spawn_column: usize,
    truncated_rows: usize,
}

impl Board {
    const EMPTY_ROW: [bool; CHAMBER_WIDTH] = [false; CHAMBER_WIDTH];
    const FULL_ROW: [bool; CHAMBER_WIDTH] = [true; CHAMBER_WIDTH];

    fn new(spawn_height: usize, spawn_column: usize) -> Self {
        Self {
            occupied: VecDeque::new(),
            spawn_height,
            spawn_column,
            truncated_rows: 0,
        }
    }

    fn check_walls_for_move(mv: Move, piece: Piece, column: usize) -> usize {
        match mv {
            Move::Left => column.saturating_sub(1),
            Move::Right => if column + piece.width() < CHAMBER_WIDTH {
                column + 1
            } else {
                column
            }
        }
    }

    fn show_tower(&self) -> String {
        let mut result = String::with_capacity((CHAMBER_WIDTH + 1) * self.occupied.len() + 1);
        for row in self.occupied.iter().rev() {
            for slot in row {
                if *slot {
                    result += "#";
                } else {
                    result += ".";
                }
            }
            result += "\n";
        }
        result
    }

    fn height(&self) -> usize {
        self.occupied.len() + self.truncated_rows
    }

    fn get_tile(&self, position: (usize, usize)) -> bool {
        let (row, column) = position;
        self.occupied[row - self.truncated_rows][column]
    }

    fn get_tile_mut(&mut self, position: (usize, usize)) -> &mut bool {
        let (row, column) = position;
        self.occupied.get_mut(row - self.truncated_rows).unwrap().get_mut(column).unwrap()
    }

    fn get_row(&mut self, row: usize) -> &[bool; CHAMBER_WIDTH] {
        self.occupied.get(row - self.truncated_rows).unwrap()
    }

    fn does_piece_fit(&self, piece: Piece, position: (usize, usize)) -> bool {
        let (row, column) = position;

        for i in 0..piece.height() {
            for j in 0..piece.width() {
                if piece.shape()[i][j] && self.get_tile((row + i, column + j)) {
                    return false;
                }
            }
        }

        true
    }

    fn drop_piece(&mut self, piece: Piece, moves_iter: &mut impl Iterator<Item = (usize, Move)>) {
        let mut column = self.spawn_column;

        // Compute the starting moves that happen before the piece reaches the tower.
        let starting_moves = moves_iter.by_ref().take(self.spawn_height + 1).collect_vec();
        println!("{piece:?} {}", starting_moves.first().unwrap().0);

        println!("{}", self.show_tower());

        for (_, mv) in starting_moves {
            column = Self::check_walls_for_move(mv, piece, column);
        }
        let mut row = self.occupied.len() + self.truncated_rows;

        // Add rows to the board to fit the new piece.
        self.occupied.extend([Self::EMPTY_ROW].repeat(piece.height()));
        assert!(self.does_piece_fit(piece, (row, column)), "piece didn't initially fit: {piece:?} {:?}\n{}", (row, column), self.show_tower());

        loop {
            let next_row = if row > self.truncated_rows {
                row - 1
            } else {
                row
            };
            if next_row == row || !self.does_piece_fit(piece, (next_row, column)) {
                // The piece can't drop down any more, and comes to a rest.
                break;
            }
            row = next_row;

            let next_column = Self::check_walls_for_move(moves_iter.next().unwrap().1, piece, column);
            if self.does_piece_fit(piece, (row, next_column)) {
                // The piece can move sideways to the new position.
                column = next_column;
            }
        }

        let mut truncate_from_row = None;

        // Add the new piece at its resting location.
        for i in 0..piece.height() {
            for j in 0..piece.width() {
                if piece.shape()[i][j] {
                    *self.get_tile_mut((row + i, column + j)) = true;
                    if self.get_row(row + i) == &Self::FULL_ROW {
                        truncate_from_row = Some(row + i);
                    }
                }
            }
        }

        // Truncate any rows that are now unreachable due to being full or below a full row.
        if let Some(truncated_row) = truncate_from_row {
            for _ in 0..=(truncated_row - self.truncated_rows) {
                self.occupied.pop_front().unwrap();
            }
            self.truncated_rows = truncated_row + 1;  // because rows are zero-indexed
        }

        // Remove completely-empty rows from the top of the tower.
        while self.occupied.back().unwrap() == &Self::EMPTY_ROW {
            self.occupied.pop_back().unwrap();
        }
    }
}

fn solve_part1(moves: &[Move]) -> usize {
    let spawn_height = 3;
    let spawn_column = 2;

    let mut board = Board::new(spawn_height, spawn_column);
    let mut move_cycle = moves.iter().copied().enumerate().cycle();
    let piece_cycle = Piece::ALL_PIECES.iter().copied().cycle();

    for piece in piece_cycle.take(2022) {
        board.drop_piece(piece, &mut move_cycle);
    }

    board.height()
}

fn solve_part2(moves: &[Move]) -> usize {
    let spawn_height = 3;
    let spawn_column = 2;

    let mut board = Board::new(spawn_height, spawn_column);
    let mut move_cycle = moves.iter().copied().enumerate().cycle();
    let piece_cycle = Piece::ALL_PIECES.iter().copied().cycle();

    for piece in piece_cycle.take(10000) {
        board.drop_piece(piece, &mut move_cycle);
    }

    // println!("{}", board.show_tower());

    board.height()
}
