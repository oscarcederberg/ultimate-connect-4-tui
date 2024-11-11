use serde::{Deserialize, Serialize};

use crate::uc4::{
    BoardType::*, MoveResult::*, GameState::*, PlayerType::*, SlotType::*, WinConditionLines::*,
};

pub const BOARD_ROWS: usize = 6;
pub const BOARD_COLS: usize = 7;
pub const ALPHA_BOARDS_NUM: usize = BOARD_COLS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotType {
    Empty,
    Filled(PlayerType),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerType {
    Blue,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardType {
    Alpha(usize),
    Omega,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Turn(PlayerType),
    Tie,
    Win(PlayerType),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveResult {
    Normal(BoardType),
    BoardTie(BoardType),
    BoardWin(BoardType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInstance {
    alpha_boards: [Board; ALPHA_BOARDS_NUM],
    omega_board: Board,
    state: GameState,
}

impl Default for GameInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl GameInstance {
    pub fn new() -> Self {
        let alpha_boards = [
            Board::new(Alpha(0)),
            Board::new(Alpha(1)),
            Board::new(Alpha(2)),
            Board::new(Alpha(3)),
            Board::new(Alpha(4)),
            Board::new(Alpha(5)),
            Board::new(Alpha(6)),
        ];
        let omega_board = Board::new(BoardType::Omega);
        let state = GameState::Turn(Blue);

        Self {
            alpha_boards,
            omega_board,
            state,
        }
    }

    pub fn board(&self, board: BoardType) -> Option<&Board> {
        match board {
            Alpha(alpha) => self.alpha_boards.get(alpha),
            Omega => Some(&self.omega_board),
        }
    }

    pub fn play(&mut self, board: BoardType, col: usize) -> Option<MoveResult> {
        let turn = match self.state {
            Turn(turn) => turn,
            _ => return None,
        };

        let index = match board {
            Alpha(index) if index < ALPHA_BOARDS_NUM => index,
            _ => return None,
        };

        let mut result = match self.alpha_boards[index].play(turn, col) {
            Some(result) => result,
            _ => return None,
        };

        match result {
            Normal(Alpha(_)) | BoardTie(Alpha(_)) => {
                self.switch_turn();
            },
            BoardWin(Alpha(index)) => {
                result = self.play_omega(index);
                match result {
                    Normal(Omega) => self.switch_turn(),
                    BoardWin(Omega) => self.state = Win(turn),
                    BoardTie(Omega) => self.state = Tie,
                    Normal(Alpha(_)) | BoardTie(Alpha(_)) | BoardWin(Alpha(_)) => unreachable!(),
                }
            },
            Normal(Omega) | BoardTie(Omega) | BoardWin(Omega) => unreachable!(),
        };

        self.calculate_available_alpha_boards(result, col);

        Some(result)
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    fn calculate_available_alpha_boards(&mut self, result: MoveResult, col: usize) {
        assert!(col < ALPHA_BOARDS_NUM);

        for board in self.alpha_boards.iter_mut() {
            board.available = false;
        }

        match result {
            Normal(Alpha(_)) if self.omega_board.slot(0, col).unwrap() == Empty => {
                self.alpha_boards[col].available = true;
            },
            Normal(Alpha(_)) | BoardTie(Omega) | BoardWin(Omega)  => {
                assert!(!matches!(self.state, Tie) || matches!(self.state, Win(_)));
                for board in self.alpha_boards.iter_mut() {
                    board.available = false;
                }
            },
            Normal(Omega) | BoardTie(Alpha(_)) | BoardWin(Alpha(_)) => {
                for (index, board) in self.alpha_boards.iter_mut().enumerate() {
                    board.available = match self.omega_board.slot(0, index).unwrap() {
                        Empty => true,
                        Filled(_) => false,
                    }
                }

                assert!(self.alpha_boards.iter().any(|b| b.available), "no available alpha boards");
            },
        }
    }

    fn play_omega(&mut self, col: usize) -> MoveResult {
        let turn = match self.state {
            Turn(turn) => turn,
            _ => unreachable!(),
        };

        self.omega_board.available = true;
        let result = self.omega_board.play(turn, col).unwrap();
        self.omega_board.available = false;
        return result;
    }

    fn switch_turn(&mut self) {
        if let Turn(turn) = self.state {
            self.state = Turn(if turn == Blue { Red } else { Blue })
        }
    }
}

enum WinConditionLines {
    Horizontal,
    Vertical,
    FirstDiagonal,
    SecondDiagonal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    available: bool,
    board_type: BoardType,
    slots: [[SlotType; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    pub fn new(board_type: BoardType) -> Self {
        let available = if matches!(board_type, Omega) {
            false
        } else {
            true
        };
        let slots = [[SlotType::Empty; BOARD_COLS]; BOARD_ROWS];

        Self {
            available,
            board_type,
            slots,
        }
    }

    pub fn available(&self) -> bool {
        self.available
    }

    pub fn board_type(&self) -> BoardType {
        self.board_type
    }

    pub fn slot(&self, row: usize, col: usize) -> Option<SlotType> {
        if row >= BOARD_ROWS || col >= BOARD_COLS {
            None
        } else {
            Some(self.slots[row][col])
        }
    }

    fn check_tie_condition(&self) -> bool {
        for col in 0..BOARD_COLS {
            if matches!(self.slots[0][col], Empty) {
                return false;
            }
        }

        return true;
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

    fn play(&mut self, player: PlayerType, col: usize) -> Option<MoveResult> {
        if col >= BOARD_COLS {
            return None;
        } else if !self.available {
            return None;
        }

        for row in (0..BOARD_ROWS).rev() {
            if self.slots[row][col] == Empty {
                self.slots[row][col] = Filled(player);

                return if self.check_win_condition(player, row, col) {
                    self.reset();
                    Some(BoardWin(self.board_type))
                } else if self.check_tie_condition() {
                    self.reset();
                    Some(BoardTie(self.board_type))
                } else {
                    Some(Normal(self.board_type))
                };
            }
        }

        None
    }


    fn reset(&mut self) {
        self.slots = [[SlotType::Empty; BOARD_COLS]; BOARD_ROWS]
    }
}
