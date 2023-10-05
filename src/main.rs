use std::{thread, time};

mod board;
mod display;

const DEFAULT_TIME_MS: u64 = 1000;

fn main() {
    let mut main_board = board::Board::new(20, 30).expect("Board too large, can't create board !");
    let mut tdisplay = display::new(display::DisplayType::TERM);
    let mut sdisplay = display::new(display::DisplayType::SDL);

    main_board.set_cell(0, 1, board::CellState::Alive);

    main_board.set_cell(4, 10, board::CellState::Alive);
    main_board.set_cell(5, 10, board::CellState::Alive);
    main_board.set_cell(6, 10, board::CellState::Alive);
    main_board.set_cell(7, 10, board::CellState::Alive);
    main_board.set_cell(7, 11, board::CellState::Alive);

    tdisplay.print(&main_board);
    sdisplay.print(&main_board);

    let mut ctrl = sdisplay.control();

    while ctrl.is_none() {
        ctrl = sdisplay.control();
    }

    let mut command = ctrl.unwrap();

    while command != display::DisplayControl::QUIT {
        main_board.next_turn();
        tdisplay.print(&main_board);
        sdisplay.print(&main_board);

        if command == display::DisplayControl::CONTINUE {
            thread::sleep(time::Duration::from_millis(DEFAULT_TIME_MS));

            ctrl = sdisplay.control();
            if ctrl.is_some() {
                command = ctrl.unwrap();
            }
        } else {
            ctrl = sdisplay.control();
            while ctrl.is_none() {
                ctrl = sdisplay.control();
            }

            command = ctrl.unwrap();
        }
    }
}
