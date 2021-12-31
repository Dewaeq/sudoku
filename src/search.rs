use crate::board::Board;

pub struct Search<'a> {
    board: &'a mut Board,
}

impl Search<'_> {
    pub fn new(board: &mut Board) -> Search {
        Search { board }
    }

    fn get_empty_square(&self, square: u8) -> u8 {
        for i in square..81 {
            if self.board.get_square_value(i) == 0 {
                return i;
            }
        }
        255
    }

    pub fn search_board(&mut self, square: u8) -> bool {
        let square = self.get_empty_square(square);
        // Board is completed
        if square == 255 {
            return true;
        }

        for i in 1..10 {
            if self.board.valid_square_value(square, i) {
                self.board.set_square_value(square, i);
                if self.search_board(square) {
                    return true;
                }
            }
            self.board.set_square_value(square, 0);
        }
        return false;
    }
}
