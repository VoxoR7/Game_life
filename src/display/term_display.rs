use super::displayable;
use crate::board;

pub struct TermDisplay {}

impl displayable::Displayable for TermDisplay {
    fn print(&mut self, board: &board::Board) {
        println!("----- Turn : {} -----", board.get_turn());

        let (row, col) = board.get_size();

        for r in 0..row {
            for c in 0..col {
                match board.get_cell(r, c).unwrap() {
                    board::CellState::Dead => print!("   "),
                    board::CellState::Alive => print!(" O "),
                }
            }
            println!();
        }
    }

    fn control(&mut self) -> Option<crate::display::DisplayControl> {
        println!("Please enter the action to do");
        println!("s step | q quit : ");

        let mut line = String::new();

        loop {
            std::io::stdin().read_line(&mut line).expect("failed to readline");
            let response = line.trim();

            match response {
                "s" => return Some(crate::display::DisplayControl::STEP),
                "q" => return Some(crate::display::DisplayControl::QUIT),
                _ => println!("not a command. Please retype : "),
            }
        }
    }

    fn cell_control(&mut self, _board: &board::Board) -> Option<(u32, u32)> {
        None
    }
}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay {}
    }
}
