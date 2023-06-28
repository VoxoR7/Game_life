fn main() {
    game_life::launch();
}

mod game_life {
    const CELL_NUMBER_HEIGHT: usize = 16;
    const CELL_NUMBER_WIDTH: usize = 16;
    
    const DEFAULT_LOOP: u64 = 100;
    const DEFAULT_TIME_MS: u64 = 100;
    
    #[derive(Copy, Clone, PartialEq)]
    enum CellState {DEAD, ALIVE}
    
    fn print_board(board: [[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT]) {
        /* print upper boundaries */
        for _ in 0..CELL_NUMBER_WIDTH {
            print!("-");
        }
        println!("");
    
        /* print board */
        for i in board {
            for j in i {
                match j {
                    CellState::DEAD => print!(" "),
                    CellState::ALIVE => print!("O"),
                }
            }
            println!("");
        }
    
        /* print lower boundaries */
        for _ in 0..CELL_NUMBER_WIDTH {
            print!("-");
        }
        println!("");
    }
    
    fn calculate_cell_alive_arround(board: [[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT], row: usize, col: usize) -> u8 {
        debug_assert!(row < CELL_NUMBER_WIDTH.try_into().unwrap(), "a row greater than the maximum has been passed !");
        debug_assert!(col < CELL_NUMBER_HEIGHT.try_into().unwrap(), "a row greater than the maximum has been passed !");
    
        let mut cell_alive = 0;
    
        if row == 0 {
            cell_alive = if board[row + 1][col] == CellState::ALIVE { 1 } else { 0 };
    
            if col == 0 {
                cell_alive += if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col + 1] == CellState::ALIVE { 1 } else { 0 };
            } else if col == CELL_NUMBER_WIDTH - 1 {
                cell_alive += if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col - 1] == CellState::ALIVE { 1 } else { 0 };
            } else {
                cell_alive += if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col - 1] == CellState::ALIVE { 1 } else { 0 };
            }
        } else if row == CELL_NUMBER_HEIGHT - 1 {
            cell_alive = if board[row - 1][col] == CellState::ALIVE { 1 } else { 0 };
    
            if col == 0 {
                cell_alive += if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row - 1][col + 1] == CellState::ALIVE { 1 } else { 0 };
            } else if col == CELL_NUMBER_WIDTH - 1 {
                cell_alive += if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row - 1][col - 1] == CellState::ALIVE { 1 } else { 0 };
            } else {
                cell_alive += if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row - 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row - 1][col - 1] == CellState::ALIVE { 1 } else { 0 };
            }
        } else {
            if col == 0 {
                cell_alive += if board[row - 1][col] == CellState::ALIVE { 1 } else { 0 } 
                            + if board[row - 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col] == CellState::ALIVE { 1 } else { 0 };
            } else if col == CELL_NUMBER_WIDTH - 1 {
                cell_alive += if board[row - 1][col] == CellState::ALIVE { 1 } else { 0 } 
                            + if board[row - 1][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col] == CellState::ALIVE { 1 } else { 0 };
            } else {
                cell_alive += if board[row - 1][col] == CellState::ALIVE { 1 } else { 0 } 
                            + if board[row - 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col + 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row - 1][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col - 1] == CellState::ALIVE { 1 } else { 0 }
                            + if board[row + 1][col] == CellState::ALIVE { 1 } else { 0 };
            }
        }
    
        cell_alive
    }
    
    fn calculate_next_turn(new: &[[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT], old: [[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT]) {
    
    }
    
    fn copy_board(dest: &[[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT], src: [[CellState; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT]) {
    
    }

    pub fn launch() {
        let mut current_board = [[CellState::DEAD; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT];
        let mut next_board = [[CellState::DEAD; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT];
        //current_board[20][50] = CellState::ALIVE;
    
        let sleep_time = std::time::Duration::from_millis(DEFAULT_TIME_MS);
    
        print_board(current_board);
        for _ in 0..DEFAULT_LOOP {
            std::thread::sleep(sleep_time);
    
    
        }
    }

    #[test]
    fn test_calculate_cell_alive_arround() {
        let mut current_board = [[CellState::DEAD; CELL_NUMBER_WIDTH]; CELL_NUMBER_HEIGHT];
        assert_eq!(calculate_cell_alive_arround(current_board, 0, 0), 0, "test_calculate_cell_alive_arround failed !");

        current_board[0][1] = CellState::ALIVE;
        assert_eq!(calculate_cell_alive_arround(current_board, 0, 0), 1, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 1, 0), 1, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 1, 1), 1, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 1, 2), 1, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 0, 2), 1, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 0, 3), 0, "test_calculate_cell_alive_arround failed !");
        assert_eq!(calculate_cell_alive_arround(current_board, 3, 0), 0, "test_calculate_cell_alive_arround failed !");
    }
}