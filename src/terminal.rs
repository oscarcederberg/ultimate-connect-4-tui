pub mod uc4;
pub mod bots;

use core::panic;
use bots::{random_bot::RandomBot, Bot};
use crossterm::{self, queue};
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use uc4::{
    BoardType::{self, *},
    GameInstance,
    GameState::*,
    MoveResult::*,
    PlayerType::*,
    SlotType::*,
};

fn main() {
    let instance = GameInstance::new();
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
    instance: GameInstance,
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

            let result = match fragments[0].to_ascii_lowercase().as_str() {
                "b" | "bots" => self.bots(fragments),
                "v" | "view" => self.view(fragments),
                "p" | "play" => self.play(fragments),
                "h" | "help" => self.help(),
                "q" | "quit" => break,
                _ => None,
            };

            if matches!(result, None) {
                println!("could not read input: \"{}\"", input.trim());
            }
        }
    }

    fn bots(&mut self, arguments: Vec<&str>) -> Option<()> {
        if arguments.len() != 3 {
            return None;
        }

        let mut bot_1 = match arguments[1] {
            "random" => RandomBot::new(),
            _ => return None,
        };

        let mut bot_2 = match arguments[1] {
            "random" => RandomBot::new(),
            _ => return None,
        };

        let total_games = 1000;
        let mut bot_1_wins = 0;
        let mut bot_2_wins = 0;

        for _ in 0..total_games {
            let mut instance = GameInstance::new();
            let mut state = instance.state();
            let mut end_condition = state == Win(Blue) || state == Win(Red) || state == Tie;

            while !end_condition {
                let bot_move = match bot_1.play(&instance) {
                    Some(bot_move) => bot_move,
                    None => {
                        bot_2_wins += 1;
                        println!("bot 1 failed to make a move, bot 2 wins");
                        break;
                    },
                };

                match instance.play(bot_move.board, bot_move.column) {
                    Some(_) => {},
                    None => {
                        bot_2_wins += 1;
                        println!("bot 1 made an illegal move, bot 2 wins");
                        break;
                    },
                }

                state = instance.state();
                end_condition = state == Win(Blue) || state == Win(Red) || state == Tie;

                if end_condition {
                    break;
                }

                let bot_move = match bot_2.play(&instance) {
                    Some(bot_move) => bot_move,
                    None => {
                        bot_1_wins += 1;
                        println!("bot 2 failed to make a move, bot 1 wins");
                        break;
                    },
                };

                match instance.play(bot_move.board, bot_move.column) {
                    Some(_) => {},
                    None => {
                        bot_1_wins += 1;
                        println!("bot 2 made an illegal move, bot 1 wins");
                        break;
                    },
                }

                state = instance.state();
                end_condition = state == Win(Blue) || state == Win(Red) || state == Tie;
            }

            match state {
                Win(Blue) => {
                    bot_1_wins += 1;
                    println!("bot 1 won!");
                }
                Win(Red) => {
                    bot_2_wins += 1;
                    println!("bot 2 won!");
                },
                Tie => println!("game was a tie!"),
                Turn(_) => {}
            }
        }

        println!("games played: {}", total_games);
        println!("bot 1 wins: {}", bot_1_wins);
        println!("bot 2 wins: {}", bot_2_wins);

        Some(())
    }

    fn view(&mut self, arguments: Vec<&str>) -> Option<()> {
        if arguments.len() < 2 {
            return None;
        }

        match arguments[1] {
            "o" | "omega" => {
                if arguments.len() != 2 {
                    return None;
                }

                self.print_game_state();
                self.print_board(Omega);
            }
            "a" | "alpha" => {
                if arguments.len() != 3 {
                    return None;
                }

                if let Ok(board) = arguments[2].parse::<usize>() {
                    self.print_game_state();
                    self.print_board(Alpha(board));
                } else {
                    return None;
                }
            }
            _ => {
                    return None;
            }
        }

        Some(())
    }

    fn play(&mut self, arguments: Vec<&str>) -> Option<()> {
        if arguments.len() != 3 {
            return None;
        }

        let board_index: usize;
        let column: usize;

        if let Ok(value) = arguments[1].parse::<usize>() {
            board_index = value;
        } else {
            return None;
        }

        if let Ok(value) = arguments[2].parse::<usize>() {
            column = value;
        } else {
            return None;
        }

        let board = Alpha(board_index);
        let result = match self.instance.play(board, column) {
            Some(result) => result,
            None => {
                return None;
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
        Some(())
    }

    fn help(&self) -> Option<()> {
        println!("help:\t\t\tview this");
        println!("bots <a> <b>:\t\tperform bot competition between bot <a> and <b>");
        println!("play <b> <c>:\t\tplay on column <c> of alpha board <b>");
        println!("view {{a <b>/alpha <b>}}:\tview alpha board <b>");
        println!("view {{o/omega}}:\t\tview omega board");
        println!("quit:\t\t\tquit the program");
        Some(())
    }

    fn print_game_state(&mut self) {
        match self.instance.state() {
            Turn(player) => println!("{}'s turn:", if player == Blue { "blue" } else { "red" }),
            Tie => println!("game tied"),
            Win(player) => println!("{} won:", if player == Blue { "blue" } else { "red" }),
        }

        println!();
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
