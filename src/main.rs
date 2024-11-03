pub mod game;

use crossterm::{self, cursor::*, queue, terminal::*};
use game::{BoardType::{self, *}, PlayerType::*, SlotType::*};
use std::io::*;

fn main() {
    let instance = game::Instance::new();
    let stdin = stdin();
    let stdout = stdout();
    let mut program = Program {instance, stdout};

    queue!(
        program.stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).expect("queue failed");

    program.print_board(game::BoardType::Omega);
    program.stdout.flush().expect("flush failed");

    loop {
        let input = &mut String::new();
        print!("\nUC4> ");
        program.stdout.flush().expect("flush failed");

        queue!(
            program.stdout,
            Clear(ClearType::All),
            MoveTo(0, 0)
        ).expect("queue failed");

        if stdin.read_line(input).is_ok() {
            program.handle_input(input.trim());
        } else {
            println!();
        }
    }
}

struct Program {
    instance: game::Instance,
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
            },
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
            },
            _ => {
                println!("could not read arguments `{:?}`", arguments);
            }
        }
    }

    fn play(&mut self, arguments: Vec<&str>) {
    }

    fn print_board(&mut self, board: BoardType) {
        use crossterm::style::*;

        match board {
            Alpha(alpha) => println!("Board α{}", alpha),
            Omega => println!("Board Ω"),
        }

        let board = self.instance.get_board(board).unwrap();

        for col in 0..game::BOARD_COLS {
            print!(" {} ", col);
        }
        println!();

        for row in 0..game::BOARD_ROWS {
            for col in 0..game::BOARD_COLS {
                let slot = board.get_slot(row, col).unwrap();
                match slot {
                    Empty => queue!(self.stdout, Print("[ ]")).expect("queue failed"),
                    Filled(First) => queue!(self.stdout, Print("["), PrintStyledContent("O".with(Color::Blue)), Print("]")).expect("queue failed"),
                    Filled(Second) => queue!(self.stdout, Print("["), PrintStyledContent("O".with(Color::Red)), Print("]")).expect("queue failed"),
                }
            }
            queue!(self.stdout, Print("\n")).expect("queue failed");
        }
        self.stdout.flush().expect("flush failed");
    }
}

