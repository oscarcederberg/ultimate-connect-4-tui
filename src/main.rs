pub mod uc4;

use crossterm::{self, cursor::*, queue, terminal::*};
use std::io::*;
use uc4::{
    BoardType::{self, *},
    PlayerType::*,
    SlotType::*,
};

fn main() {
    let instance = uc4::Instance::new();
    let stdin = stdin();
    let stdout = stdout();
    let mut program = Program { instance, stdout };

    // queue!(program.stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("queue failed");

    program.print_board(uc4::BoardType::Omega);
    program.stdout.flush().expect("flush failed");

    loop {
        let input = &mut String::new();
        print!("\nUC4> ");
        program.stdout.flush().expect("flush failed");

        // queue!(program.stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("queue failed");

        if stdin.read_line(input).is_ok() {
            program.handle_input(input.trim());
        } else {
            println!();
        }
    }
}

struct Program {
    instance: uc4::Instance,
    stdout: Stdout,
}

impl Program {
    fn handle_input(&mut self, input: &str) {
        let fragments: Vec<&str> = input.split(" ").collect();

        match fragments[0].to_ascii_lowercase().as_str() {
            "v" | "view" => {
                self.view(fragments);
            }
            "p" | "play" => {
                self.play(fragments);
            }
            "h" | "help" => {
                println!("help:\t\t\tview this");
                println!("play <b> <c>:\t\tplay on column <c> of alpha board <b>");
                println!("view {{o/omega}}:\t\tview omega board");
                println!("view {{a <b>/alpha <b>}}:\tview alpha board <b>");
            }
            _ => {
                println!("could not read input `{}`", input);
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

                self.print_board(Omega);
            }
            "a" | "alpha" => {
                if arguments.len() != 3 {
                    println!("could not read arguments `{:?}`", arguments);
                    return;
                }

                if let Ok(board) = arguments[2].parse::<usize>() {
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

        let board: usize;
        let column: usize;

        if let Ok(value) = arguments[1].parse::<usize>() {
            board = value;
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

        let result = self.instance.play(Alpha(board), column);
        match result {
            Some(result) => {
                self.print_board(Alpha(board));
                println!("played on board alpha {}", board);
            }
            None => println!("could not perform play with arguments `{:?}`", arguments),
        }
    }

    fn print_board(&mut self, board: BoardType) {
        use crossterm::style::*;

        match board {
            Alpha(alpha) => println!("Board α{}", alpha),
            Omega => println!("Board Ω"),
        }

        let board = self.instance.get_board(board).unwrap();

        for col in 0..uc4::BOARD_COLS {
            print!(" {} ", col);
        }
        println!();

        for row in 0..uc4::BOARD_ROWS {
            for col in 0..uc4::BOARD_COLS {
                let slot = board.get_slot(row, col).unwrap();
                match slot {
                    Empty => queue!(self.stdout, Print("[ ]")).expect("queue failed"),
                    Filled(First) => queue!(
                        self.stdout,
                        Print("["),
                        PrintStyledContent("O".with(Color::Blue)),
                        Print("]")
                    )
                    .expect("queue failed"),
                    Filled(Second) => queue!(
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
