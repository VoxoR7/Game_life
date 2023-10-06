use std::{thread, time};

mod board;
mod display;

const DEFAULT_TIME_MS: u128 = 50;
const DEFAULT_REFRESH_MS: u64 = 33;

fn main() {
    let mut main_board = board::Board::new(80, 120).expect("Board too large, can't create board !");
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
    let mut command = ctrl.map_or(display::DisplayControl::STOP, |ctrl| ctrl);

    'inner_while: while command != display::DisplayControl::QUIT {
        match command {
            display::DisplayControl::CONTINUE => {
                let now = time::Instant::now();

                while now.elapsed().as_millis() < DEFAULT_TIME_MS {
                    thread::sleep(time::Duration::from_millis(DEFAULT_REFRESH_MS));
                    if let Some(command) = sdisplay.control() {
                        if command != display::DisplayControl::CONTINUE {
                            continue 'inner_while;
                        }
                    }
                }

                main_board.next_turn();
                tdisplay.print(&main_board);
                sdisplay.print(&main_board);
            },
            display::DisplayControl::STOP => {
                ctrl = None;
                while ctrl.is_none() {
                    thread::sleep(time::Duration::from_millis(DEFAULT_REFRESH_MS));
                    ctrl = sdisplay.control();
                    if let Some((y, x)) = sdisplay.cell_control(&main_board) {
                        main_board.reverse_cell(y as usize, x as usize);
                        tdisplay.print(&main_board);
                        sdisplay.print(&main_board);
                    }
                }
                command = ctrl.unwrap();
            },
            display::DisplayControl::STEP => {
                main_board.next_turn();
                tdisplay.print(&main_board);
                sdisplay.print(&main_board);
                command = display::DisplayControl::STOP
            },
            display::DisplayControl::QUIT => break,
        }
    }
}
