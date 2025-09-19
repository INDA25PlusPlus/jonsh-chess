pub mod board;
pub mod movement;
pub mod pieces;

use jonsh_chess::input;

use crate::board::Board;

fn main() {
    let mut board = Board::new();
    let (mut is_checkmate, mut is_white, mut is_stalemate) = (false, false, false);
    while (!is_checkmate) && (!is_stalemate) {
        let (fx, fy) = input();
        println!("Valid Moves: {:?}", board.valid_moves(fx, fy));
        println!("Legal Moves: {:?}", board.legal_moves(fx, fy));
        let (tx, ty) = input();
        board.move_piece(fx, fy, tx, ty);
        println!("Check: {:?}", board.is_check());
        board.print_board();
        (is_checkmate, is_white, is_stalemate) = board.game_end();
        println!("En passant: {:?}", board.en_passant_target);
    }
    if is_checkmate {
        if is_white {
            println!("WHITE IS CHECKMATE")
        } else {
            println!("BLACK IS CHECKMATE")
        }
    }
    if is_stalemate {
        println!("STALEMATE")
    }
}
