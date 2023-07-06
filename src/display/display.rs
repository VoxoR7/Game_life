use crate::board;

pub trait Displayable {
    fn print(&self, board: &board::Board);
}
