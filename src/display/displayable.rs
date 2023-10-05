use crate::board;

pub trait Displayable {
    fn print(&mut self, board: &board::Board);
    fn control(&mut self) -> Option<crate::display::DisplayControl>;
    fn cell_control(&mut self, board: &board::Board) -> Option<(u32, u32)>;
}
