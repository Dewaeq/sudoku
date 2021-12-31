pub mod board;
pub mod search;
use board::Board;
use search::Search;

#[rustfmt::skip]
pub static DEFAULT_BOARD: [u8; 81] = [
    0, 0, 8, 0, 1, 0, 7, 9, 3, 
    0, 0, 4, 2, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 6, 0, 
    1, 0, 0, 4, 0, 2, 0, 0, 0, 
    0, 7, 0, 0, 9, 5, 6, 0, 0, 
    4, 0, 0, 3, 0, 0, 0, 8, 0, 
    5, 1, 0, 0, 0, 0, 8, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 6, 
    0, 0, 2, 0, 0, 0, 4, 5, 9,
];

fn parse_board(board: Option<Board>) -> Board {
    match board {
        Some(b) => b,
        None => {
            println!("Failed to parse board");
            panic!();
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut board: Board;

    if args.len() > 1 {
        let input = args.get(1).unwrap();
        board = parse_board(Board::from_input(input));
    } else {
        board = parse_board(Board::new(DEFAULT_BOARD));
    }

    let mut search = Search::new(&mut board);
    let succes = search.search_board(0);

    if !succes {
        println!("Failed to solve sudoku! Please check input");
    }
    board.print_board();
}
