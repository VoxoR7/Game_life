use super::display;
use crate::board;

pub struct TermDisplay {}

impl display::Displayable for TermDisplay {
    fn print(&self, board: &board::Board) {
        println!("----- Turn : {} -----", board.get_turn());

        let (row, col) = board.get_size();

        for c in 0..col {
            for r in 0..row {
                match board.get_cell(r, c).unwrap() {
                    board::CellState::Dead => print!("   "),
                    board::CellState::Alive => print!(" O "),
                }
            }
            print!("\n");
        }
    }
}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay {}
    }
}