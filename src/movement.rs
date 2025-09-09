use crate::board::{Board, Tile};
use crate::pieces::{Color, Piece};
use std::io;

impl Board {
    pub fn move_piece(&mut self) {
        let mut moves = String::new();
        io::stdin().read_line(&mut moves).unwrap();
        let moves: Vec<usize> = moves
            .trim()
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        let f_row: usize = moves[0];
        let f_col: usize = moves[1];
        let t_row: usize = moves[2];
        let t_col: usize = moves[3];
        let get_piece = self.tiles[f_row][f_col];
        println!("{:?}", get_piece);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_piece() {
        let mut board = Board::new();
        board.move_piece();
    }
}
