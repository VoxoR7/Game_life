#[derive(Copy, Clone, PartialEq)]
pub enum CellState {Dead, Alive}

pub struct Board {
    board: Vec<Vec<CellState>>,
    second_board: Vec<Vec<CellState>>,
    turn: u32,
}

impl Board {
    pub fn is_alive(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 {
            return false;
        }

        let urow: usize;
        let ucol: usize;

        /* SAFETY : row and col cannot be negative number,
            we check it right before */
        unsafe {
            urow = row.try_into().unwrap_unchecked();
            ucol = col.try_into().unwrap_unchecked();
        }

        if let Some(vec_row) = self.board.get(urow) {
            if let Some(cell) = vec_row.get(ucol) {
                return CellState::Alive == *cell;
            }
        }

        false
    }

    /// Return the number of cell alive in the 8 surrounding cell
    fn get_cell_alive_arround(&self, row: isize, col: isize) -> u8 {
        let mut surrounding_alive = 0;

        if self.is_alive(row - 1, col - 1) {
            surrounding_alive += 1;
        }
        if self.is_alive(row - 1, col) {
            surrounding_alive += 1;
        }
        if self.is_alive(row - 1, col + 1) {
            surrounding_alive += 1;
        }
        if self.is_alive(row, col - 1) {
            surrounding_alive += 1;
        }
        if self.is_alive(row, col + 1) {
            surrounding_alive += 1;
        }
        if self.is_alive(row + 1, col - 1) {
            surrounding_alive += 1;
        }
        if self.is_alive(row + 1, col) {
            surrounding_alive += 1;
        }
        if self.is_alive(row + 1, col + 1) {
            surrounding_alive += 1;
        }

        surrounding_alive
    }

    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    /// calculate the next turn
    pub fn next_turn(&mut self) {
        for (row, outer_elem) in self.board.iter().enumerate() {
            for (col, _) in outer_elem.iter().enumerate() {
                let surrounding_alive = self.get_cell_alive_arround(row as isize, col as isize);

                if self.board[row][col] == CellState::Alive {
                    if surrounding_alive < 2 {
                        self.second_board[row][col] = CellState::Dead;
                    } else if surrounding_alive > 3 {
                        self.second_board[row][col] = CellState::Dead;
                    } else {
                        self.second_board[row][col] = CellState::Alive;
                    }
                } else {
                    if surrounding_alive == 3 {
                        self.second_board[row][col] = CellState::Alive;
                    } else {
                        self.second_board[row][col] = CellState::Dead;
                    }
                }
            }
        }

        for (row, outer_elem) in self.board.iter_mut().enumerate() {
            for (col, inner_elem) in outer_elem.iter_mut().enumerate() {
                *inner_elem = self.second_board[row][col];
            }
        }

        self.turn += 1;
    }

    /// print to the screen the board
    pub fn print(&self) {
        println!("----- Turn : {} -----", self.turn);

        for outer_elem in self.board.iter() {
            for inner_elem in outer_elem.iter() {
                match inner_elem {
                    CellState::Dead => print!("   "),
                    CellState::Alive => print!(" O "),
                }
            }
            print!("\n");
        }
    }

    /// set a particular cell to a state
    pub fn set_cell(&mut self, row: usize, col: usize, state: CellState) {
        if let Some(vec_row) = self.board.get_mut(row) {
            if let Some(cell) = vec_row.get_mut(col) {
                *cell = state;
            }
        }
    }

    /// Return a board constitued of only dead cell
    /// with size col and row
    pub fn new(row: usize, col: usize) -> Option<Board> {
        if row > isize::MAX.try_into().unwrap() || col > isize::MAX.try_into().unwrap() {
            return None
        }

        let mut vec:Vec<Vec<CellState>> = Vec::with_capacity(row);
        for i in 0..row {
            vec.push(Vec::with_capacity(col));
            for _ in 0..col {
                vec[i].push(CellState::Dead);
            }
        }

        let vec2 = vec.clone();

        dbg!("a board of size ^{} by <{} as been created !", vec.len(), vec[0].len());

        Some(Board { 
            board: vec,
            second_board: vec2,
            turn: 0,
        })
    }
}
