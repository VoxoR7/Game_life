use std::{thread, time};

mod board;
mod display;

const DEFAULT_LOOP: u32 = 100;
const DEFAULT_TIME_MS: u64 = 1000;

fn main() {
    let mut main_board = board::Board::new(20, 30).expect("Board too large, can't create board !");
    let display = display::new();

    main_board.set_cell(4, 10, board::CellState::Alive);
    main_board.set_cell(5, 10, board::CellState::Alive);
    main_board.set_cell(6, 10, board::CellState::Alive);
    main_board.set_cell(7, 10, board::CellState::Alive);
    main_board.set_cell(7, 11, board::CellState::Alive);

    display.print(&main_board);

    while main_board.get_turn() < DEFAULT_LOOP {
        thread::sleep(time::Duration::from_millis(DEFAULT_TIME_MS));
        main_board.next_turn();
        display.print(&main_board);
    }
}
