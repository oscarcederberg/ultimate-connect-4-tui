pub const BOARD_ROWS: usize = 6;
pub const BOARD_COLS: usize = 7;
pub const ALPHA_BOARDS_NUM: usize = BOARD_COLS;

#[derive(Copy, Clone)]
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
            Board::new(BoardType::Alpha(1)),
            Board::new(BoardType::Alpha(2)),
            Board::new(BoardType::Alpha(3)),
            Board::new(BoardType::Alpha(4)),
            Board::new(BoardType::Alpha(5)),
            Board::new(BoardType::Alpha(6)),
            Board::new(BoardType::Alpha(7)),
        ];
        let omega_board = Board::new(BoardType::Omega);
        let turn = PlayerType::First;
        Self {alpha_boards, omega_board, turn}
    }

    pub fn get_board(&self, board: BoardType) -> Option<&Board> {
        match board {
            BoardType::Alpha(alpha) if alpha > 0 && alpha <= ALPHA_BOARDS_NUM => Some(&self.alpha_boards[alpha]),
            BoardType::Omega => Some(&self.omega_board),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    board_type: BoardType,
    slots: [[SlotType; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    pub fn new(board_type: BoardType) -> Self {
        let slots = [[SlotType::Empty; BOARD_COLS]; BOARD_ROWS];
        Self {board_type, slots}
    }

    pub fn get_slot(&self, row: usize, col: usize) -> Option<SlotType> {
        if row >= BOARD_ROWS || col >= BOARD_COLS {
            None
        } else {
            Some(self.slots[row][col])
        }
    }

    pub fn play(col:usize) -> GameMoveResult {
        GameMoveResult::Fail
    }
}

#[derive(Copy, Clone)]
pub enum SlotType {
    Empty,
    Filled(PlayerType),
}

#[derive(Copy, Clone)]
pub enum PlayerType {
    First,
    Second,
}

#[derive(Copy, Clone)]
pub enum BoardType {
    Alpha(usize),
    Omega,
}

#[derive(Copy, Clone)]
pub enum GameMoveResult {
    Fail,
    Ok,
    AlphaTie,
    AlphaWin,
    OmegaTie,
    OmegaWin,
}
