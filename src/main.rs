pub mod uc4;

use core::panic;
use crossterm::{self, queue};
use std::io::*;
use uc4::{
    BoardType::{self, *},
    GameState::*,
    MoveResult::*,
    PlayerType::*,
    SlotType::*,
};

fn main() {
    let instance = uc4::GameInstance::new();
    let stdin = stdin();
    let stdout = stdout();
    let mut program = Program {
        instance,
        stdin,
        stdout,
    };
    program.run();
}

struct Program {
    instance: uc4::GameInstance,
    stdin: Stdin,
    stdout: Stdout,
}

impl Program {
    fn run(&mut self) {
        self.print_game_state();
        self.print_board(uc4::BoardType::Omega);
        self.stdout.flush().expect("flush failed");

        loop {
            let input = &mut String::new();
            print!("\nUC4> ");
            self.stdout.flush().expect("flush failed");

            // queue!(program.stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("queue failed");

            if self.stdin.read_line(input).is_err() {
                println!();
                continue;
            }

            let fragments: Vec<&str> = input.trim().split(" ").collect();

            match fragments[0].to_ascii_lowercase().as_str() {
                "v" | "view" => self.view(fragments),
                "p" | "play" => self.play(fragments),
                "q" | "quit" => break,
                "h" | "help" => self.help(),
                _ => println!("could not read input `{}`", input),
            }
        }
    }

    fn view(&mut self, arguments: Vec<&str>) {
        match arguments[1] {
            "o" | "omega" => {
                if arguments.len() != 2 {
                    println!("could not read arguments `{:?}`", arguments);
                    return;
                }

                self.print_game_state();
                self.print_board(Omega);
            }
            "a" | "alpha" => {
                if arguments.len() != 3 {
                    println!("could not read arguments `{:?}`", arguments);
                    return;
                }

                if let Ok(board) = arguments[2].parse::<usize>() {
                    self.print_game_state();
                    self.print_board(Alpha(board));
                } else {
                    println!("could not read arguments `{:?}`", arguments);
                }
            }
            _ => {
                println!("could not read arguments `{:?}`", arguments);
            }
        }
    }

    fn play(&mut self, arguments: Vec<&str>) {
        if arguments.len() != 3 {
            println!("could not read arguments `{:?}`", arguments);
            return;
        }

        let board_index: usize;
        let column: usize;

        if let Ok(value) = arguments[1].parse::<usize>() {
            board_index = value;
        } else {
            println!("could not read arguments `{:?}`", arguments);
            return;
        }

        if let Ok(value) = arguments[2].parse::<usize>() {
            column = value;
        } else {
            println!("could not read arguments `{:?}`", arguments);
            return;
        }

        let board = Alpha(board_index);
        let result = match self.instance.play(board, column) {
            Some(result) => result,
            None => {
                println!("could not perform play with arguments `{:?}`", arguments);
                return;
            }
        };

        match result {
            Normal(Alpha(index)) => {
                self.print_game_state();
                self.print_board(Alpha(index));
                println!("played on board alpha {}", index);
            }
            Normal(Omega) => {
                self.print_game_state();
                self.print_board(Omega);
                println!("won on board alpha {}", board_index);
            }
            BoardTie(Alpha(index)) => {
                self.print_game_state();
                self.print_board(Alpha(index));
                println!("tied on board alpha {}", index);
            }
            BoardTie(Omega) => {
                self.print_board(Omega);
                println!("won on board alpha {}", board_index);
                println!("game tied!");
            }
            BoardWin(Alpha(_)) => unreachable!(),
            BoardWin(Omega) => {
                self.print_board(Omega);
                println!("won on board alpha {}", board_index);
                if let Win(player) = self.instance.state() {
                    println!(
                        "game won by {}!",
                        if player == Blue { "blue" } else { "red" }
                    );
                } else {
                    unreachable!();
                }
            }
        }
    }

    fn help(&self) {
        println!("help:\t\t\tview this");
        println!("play <b> <c>:\t\tplay on column <c> of alpha board <b>");
        println!("view {{a <b>/alpha <b>}}:\tview alpha board <b>");
        println!("view {{o/omega}}:\t\tview omega board");
        println!("quit:\t\t\tquit the program");
    }

    fn print_game_state(&mut self) {
        match self.instance.state() {
            Turn(player) => println!("{}'s turn:", if player == Blue { "blue" } else { "red" }),
            Tie => println!("game tied"),
            Win(player) => println!("{} won:", if player == Blue { "blue" } else { "red" }),
        }

        println!("");
    }

    fn print_board(&mut self, board: BoardType) {
        use crossterm::style::*;

        let board = match self.instance.board(board) {
            Some(board) => board,
            None => panic!(),
        };

        match board.board_type() {
            Alpha(alpha) => println!(
                "Board α{}{}",
                alpha,
                if board.available() {
                    ""
                } else {
                    " - Unavailable"
                }
            ),
            Omega => println!("Board Ω - Unavailable"),
        }

        for col in 0..uc4::BOARD_COLS {
            print!(" {} ", col);
        }
        println!();

        for row in 0..uc4::BOARD_ROWS {
            for col in 0..uc4::BOARD_COLS {
                let slot = board.slot(row, col).unwrap();
                match slot {
                    Empty => queue!(self.stdout, Print("[ ]")).expect("queue failed"),
                    Filled(Blue) => queue!(
                        self.stdout,
                        Print("["),
                        PrintStyledContent("O".with(Color::Blue)),
                        Print("]")
                    )
                    .expect("queue failed"),
                    Filled(Red) => queue!(
                        self.stdout,
                        Print("["),
                        PrintStyledContent("O".with(Color::Red)),
                        Print("]")
                    )
                    .expect("queue failed"),
                }
            }
            queue!(self.stdout, Print("\n")).expect("queue failed");
        }
        self.stdout.flush().expect("flush failed");
    }
}
