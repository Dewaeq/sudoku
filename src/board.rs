pub struct Board {
    /// Square 0 is the left upper square, `squares[0]`
    pub squares: [u8; 81],
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Square;
impl Board {
    pub fn new(squares: [u8; 81]) -> Option<Board> {
        if !(Board { squares }).valid_board() {
            return None;
        }
        Some(Board { squares })
    }

    pub fn from_input(input: &str) -> Option<Board> {
        let list: Vec<&str> = input.split("").collect();
        assert!(list.len() == 81, "Please provide a full length string");

        let mut data: [u8; 81] = [0; 81];
        for i in 0..81 {
            let value = list.get(i).unwrap();
            let mut message = "Failed to parse {} to u8".to_owned();
            message = message.replace("{}", value);

            data[i] = value.parse::<u8>().expect(&message);
        }
        Board::new(data)
    }

    fn valid_board(&mut self) -> bool {
        let mut succes = true;
        for i in 0..81 {
            let value = self.get_square_value(i);
            if value == 0 {
                continue;
            }
            if !self.valid_square_value(i, value) {
                let (x, y) = Square::get_coord(i);
                println!("Illegal value {} at x:{} y:{}", value, x + 1, y + 1);
                succes = false;
            }
        }
        succes
    }

    pub fn set_square_value(&mut self, index: u8, value: u8) {
        *self.squares.get_mut(index as usize).unwrap() = value;
    }

    fn get_row(&self, y: u8) -> &[u8] {
        let start = y as usize * 9;
        let stop = start + 9;
        &self.squares[start..stop]
    }

    fn get_column(&self, x: u8) -> [u8; 9] {
        let mut output: [u8; 9] = [0; 9];
        for i in 0..9 {
            output[i] = self.squares[x as usize + i * 9]
        }
        output
    }

    pub fn get_square_value(&self, square: u8) -> u8 {
        *self.squares.get(square as usize).unwrap()
    }

    pub fn valid_square_value(&mut self, index: u8, value: u8) -> bool {
        let old_value = self.get_square_value(index);
        self.set_square_value(index, 0);

        let (column, row) = Square::get_coord(index);
        let fits_row = self.square_fits_in_row(row, value);
        let fits_column = self.value_fits_in_column(column, value);
        let fits_cell = self.fits_cell(index, value);
        self.set_square_value(index, old_value);

        fits_row && fits_column && fits_cell
    }

    fn square_fits_in_row(&self, row: u8, value: u8) -> bool {
        !self.get_row(row).contains(&value)
    }

    fn value_fits_in_column(&self, column: u8, value: u8) -> bool {
        !self.get_column(column).contains(&value)
    }

    fn fits_cell(&self, index: u8, value: u8) -> bool {
        let cell_coords = Square::get_cell_squares(index);
        for i in 0..cell_coords.len() {
            let square = *cell_coords.get(i).unwrap();

            if self.get_square_value(square) == value {
                return false;
            }
        }
        true
    }

    pub fn print_board(&self) {
        for i in 0..81 {
            let x = Square::get_coord(i).0;
            let value = self.get_square_value(i);
            if x == 8 {
                println!(" {} ", value);
            } else {
                print!(" {} ", value);
            }
        }
    }
}

impl Square {
    fn get_coord(square: u8) -> (u8, u8) {
        let y = square / 9;
        let x = square % 9;
        (x, y)
    }

    fn square_from_coord(x: u8, y: u8) -> u8 {
        9 * y + x
    }

    fn get_cell_start_square(square: u8) -> u8 {
        let (x, y) = Square::get_coord(square);
        // Coord of left lower corner of cell
        let cell_x = x - (x % 3);
        let cell_y = y - (y % 3);
        Square::square_from_coord(cell_x, cell_y)
    }

    pub fn get_cell_squares(square: u8) -> [u8; 9] {
        let mut output: [u8; 9] = [0; 9];
        let mut start_square = Square::get_cell_start_square(square);
        for i in 0..3 {
            for j in 0..3 {
                output[i * 3 + j] = start_square + (j as u8);
            }
            start_square += 9;
        }
        output
    }
}
