use std::{thread, time};

mod board;

const DEFAULT_LOOP: u32 = 100;
const DEFAULT_TIME_MS: u64 = 500;

fn main() {
    let mut main_board = board::Board::new(20, 30).expect("Board too large, can't create board !");

    main_board.set_cell(4, 10, board::CellState::Alive);
    main_board.set_cell(5, 10, board::CellState::Alive);
    main_board.set_cell(6, 10, board::CellState::Alive);

    main_board.print();

    while main_board.get_turn() < DEFAULT_LOOP {
        thread::sleep(time::Duration::from_millis(DEFAULT_TIME_MS));
        main_board.next_turn();
        main_board.print();
    }
}
