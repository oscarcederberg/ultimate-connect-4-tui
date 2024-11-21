pub mod random_bot;

use crate::uc4::{*, BoardType::*, SlotType::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub board: BoardType,
    pub column: usize,
}

pub trait Bot {
    fn id() -> &'static str;
    fn play(&mut self, instance: &GameInstance) -> Option<Move>;
}

pub fn get_available_moves(instance: &GameInstance) -> Vec<Move> {
    let mut boards = Vec::new();
    for index in 0..ALPHA_BOARDS_NUM {
        let board = instance.board(Alpha(index)).unwrap();
        if board.available() {
            boards.push(board);
        }
    }

    let mut moves = Vec::new();
    for board in boards {
        for column in 0..BOARD_COLS {
            if board.slot(0, column).unwrap() == Empty {
                moves.push(Move{board: board.board_type(), column});
            }
        }
    }
    moves
}