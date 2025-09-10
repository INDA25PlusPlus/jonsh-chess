use crate::board::{Board, Tile};
use crate::pieces::{Color, Piece};
use std::io;

impl Board {
    //Get Piece
    pub fn get_piece(&self, piece: (usize, usize)) -> Tile {
        let tile = self.tiles[piece.0][piece.1];
        return tile;
    }
    // pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
    //     if self.get_
    // }
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;
    #[test]
    fn print_piece() {
        let mut board = Board::new();
        let (f_row, f_col) = input();
        if let Tile::Occupied(_, piece) = board.tiles[f_row][f_col] {
            println!("{:?}", piece);
        }
    }
    #[test]
    fn test_get_piece() {
        let mut board = Board::new();
        let mut f_row = String::new();
        let mut f_col = String::new();
        io::stdin().read_line(&mut f_row).unwrap();
        let f_row: usize = f_row.trim().parse().unwrap();
        io::stdin().read_line(&mut f_col).unwrap();
        let f_col: usize = f_col.trim().parse().unwrap();
        let tile = board.get_piece((f_row, f_col));
        println!("{:?}", tile);
    }
}
