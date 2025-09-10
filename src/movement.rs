use crate::board::{Board, Tile};
use crate::pieces::{Color, Piece};
use std::io;

impl Board {
    pub fn move_piece(mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        // rudimentary movement function 🥳
        let curr_piece = self.tiles[from_x][from_y]; // get selected piece info
        self.tiles[to_x][to_y] = curr_piece; // move piece to new place
        self.tiles[from_x][from_y] = Tile::Empty; // delete last remaining instance of the piece
        return self; //BOOOOOOOOOOOOOOMMMM, EZ win, WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW 😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;
    #[test]
    fn print_piece() {
        let mut board = Board::new();
        let (from_x, from_y) = input();
        if let Tile::Occupied(_, piece) = board.tiles[from_x][from_y] {
            println!("{:?}", piece);
        }
    }
    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        let (from_x, from_y) = input();
        let (to_x, to_y) = input();
        let board = board.move_piece(from_x, from_y, to_x, to_y);
        Board::print_board(board);
    }
    #[test]
    fn test_input() {
        let (x, y) = input();
        println!("{:?}", (x, y));
    }
}
