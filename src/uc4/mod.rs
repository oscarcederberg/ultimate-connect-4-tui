use serde::{Deserialize, Serialize};

use crate::uc4::{
    BoardType::*, GameMoveResult::*, PlayerType::*, SlotType::*, WinConditionLines::*,
};

pub const BOARD_ROWS: usize = 6;
pub const BOARD_COLS: usize = 7;
pub const ALPHA_BOARDS_NUM: usize = BOARD_COLS;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    alpha_boards: [Board; ALPHA_BOARDS_NUM],
    omega_board: Board,
    turn: PlayerType,
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}

impl Instance {
    pub fn new() -> Self {
        let alpha_boards = [
            Board::new(Alpha(1)),
            Board::new(Alpha(2)),
            Board::new(Alpha(3)),
            Board::new(Alpha(4)),
            Board::new(Alpha(5)),
            Board::new(Alpha(6)),
            Board::new(Alpha(7)),
        ];
        let omega_board = Board::new(BoardType::Omega);
        let turn = PlayerType::First;
        Self {
            alpha_boards,
            omega_board,
            turn,
        }
    }

    pub fn get_board(&self, board: BoardType) -> Option<&Board> {
        match board {
            Alpha(alpha) if alpha > 0 && alpha <= ALPHA_BOARDS_NUM => {
                Some(&self.alpha_boards[alpha - 1])
            }
            Omega => Some(&self.omega_board),
            _ => None,
        }
    }

    pub fn play(&mut self, board: BoardType, col: usize) -> Option<GameMoveResult> {
        if let BoardType::Alpha(alpha) = board {
            return match self.alpha_boards[alpha - 1].play(self.turn, col) {
                Some(AlphaWin) => {
                    self.switch_turn();
                    Some(self.play_omega(col))
                }
                Some(result) => {
                    self.switch_turn();
                    Some(result)
                }
                _ => None,
            };
        } else {
            None
        }
    }

    fn play_omega(&mut self, col: usize) -> GameMoveResult {
        self.omega_board.play(self.turn, col).unwrap()
    }

    fn switch_turn(&mut self) {
        self.turn = if self.turn == First { Second } else { First }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    board_type: BoardType,
    slots: [[SlotType; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    pub fn new(board_type: BoardType) -> Self {
        let slots = [[SlotType::Empty; BOARD_COLS]; BOARD_ROWS];
        Self { board_type, slots }
    }

    pub fn get_slot(&self, row: usize, col: usize) -> Option<SlotType> {
        if row >= BOARD_ROWS || col >= BOARD_COLS {
            None
        } else {
            Some(self.slots[row][col])
        }
    }

    fn play(&mut self, player: PlayerType, col: usize) -> Option<GameMoveResult> {
        if col <= 0 || col > BOARD_COLS {
            return None;
        } else if !matches!(self.slots[col - 1][0], Empty) {
            return None;
        }

        let col = col - 1;

        for row in (0..BOARD_ROWS).rev() {
            if self.slots[row][col] == Empty {
                self.slots[row][col] = Filled(player);

                if self.check_win_condition(player, row, col) {
                    self.reset_board();
                    if self.board_type == Omega {
                        return Some(OmegaWin);
                    } else {
                        return Some(AlphaWin);
                    }
                }

                if self.check_tie_condition() {
                    self.reset_board();
                    if self.board_type == Omega {
                        return Some(OmegaTie);
                    } else {
                        return Some(AlphaTie);
                    }
                }

                return Some(Next);
            }
        }

        return None;
    }

    fn check_win_condition(&self, player: PlayerType, row: usize, col: usize) -> bool {
        for line in [Horizontal, Vertical, FirstDiagonal, SecondDiagonal] {
            let mut counter = 0;
            let offsets: Vec<i32> = (-3..4).collect();
            for offset in offsets {
                let mut row_to_check = row as i32;
                let mut col_to_check = col as i32;

                match line {
                    Horizontal => {
                        col_to_check += offset;
                    }
                    Vertical => {
                        row_to_check += offset;
                    }
                    FirstDiagonal => {
                        row_to_check += offset;
                        col_to_check += offset;
                    }
                    SecondDiagonal => {
                        row_to_check -= offset;
                        col_to_check += offset;
                    }
                }

                if row_to_check < 0 || row_to_check >= BOARD_ROWS as i32 {
                    counter = 0;
                    continue;
                }

                if col_to_check < 0 || col_to_check >= BOARD_COLS as i32 {
                    counter = 0;
                    continue;
                }

                let slot = self.slots[row_to_check as usize][col_to_check as usize];
                if slot == Filled(player) {
                    counter += 1;
                    if counter >= 4 {
                        return true;
                    }
                } else {
                    counter = 0;
                }
            }
        }

        return false;
    }

    fn check_tie_condition(&self) -> bool {
        for col in 0..BOARD_COLS {
            if matches!(self.slots[0][col], Empty) {
                return false;
            }
        }

        return true;
    }

    fn reset_board(&mut self) {
        self.slots = [[SlotType::Empty; BOARD_COLS]; BOARD_ROWS]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotType {
    Empty,
    Filled(PlayerType),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerType {
    First,
    Second,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardType {
    Alpha(usize),
    Omega,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMoveResult {
    Next,
    AlphaTie,
    AlphaWin,
    OmegaTie,
    OmegaWin,
}

enum WinConditionLines {
    Horizontal,
    Vertical,
    FirstDiagonal,
    SecondDiagonal,
}
