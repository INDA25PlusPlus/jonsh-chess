use crate::board::{Board, Tile};
use crate::pieces::{Color, Piece};

impl Board {
    pub fn move_piece_basic(
        mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Self {
        // rudimentary movement function 🥳
        let curr_piece = self.tiles[from_y][from_x]; // get selected piece info
        self.tiles[to_y][to_x] = curr_piece; // move piece to new place
        self.tiles[from_y][from_x] = Tile::Empty; // delete last remaining instance of the piece
        return self; //BOOOOOOOOOOOOOOMMMM, EZ win, WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW 😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎😎
    }

    pub fn move_piece(mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        let selected_piece = self.tiles[from_y][from_x];

        if let Tile::Occupied(_, Piece::Pawn) = selected_piece // rudimentary pawn movement
            && from_x == to_x
        {
            if let Tile::Occupied(Color::Black, _) = selected_piece {
                if from_y == 1 {
                    if to_y - from_y == 1 || to_y - from_y == 2 {
                        self.tiles[to_y][to_x] = selected_piece;
                        self.tiles[from_y][from_x] = Tile::Empty;
                    }
                } else {
                    if to_y - from_y == 1 {
                        self.tiles[to_y][to_x] = selected_piece;
                        self.tiles[from_y][from_x] = Tile::Empty;
                    }
                }
            } else {
                if from_y == 6 {
                    if from_y - to_y == 1 || from_y - to_y == 2 {
                        self.tiles[to_y][to_x] = selected_piece;
                        self.tiles[from_y][from_x] = Tile::Empty;
                    }
                } else {
                    if from_y - to_y == 1 {
                        self.tiles[to_y][to_x] = selected_piece;
                        self.tiles[from_y][from_x] = Tile::Empty;
                    }
                }
            }
        } else if let Tile::Occupied(_, Piece::Knight) = selected_piece {
            if (from_x as isize - to_x as isize).abs() == 1
                && (from_y as isize - to_y as isize).abs() == 2
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if (from_x as isize - to_x as isize).abs() == 2
                && (from_y as isize - to_y as isize).abs() == 1
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Bishop) = selected_piece {
            if (from_x as isize - to_x as isize).abs() == (from_y as isize - to_y as isize).abs() {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Rook) = selected_piece {
            if (from_x as isize - to_x as isize) != 0 && (from_y as isize - to_y as isize) == 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if (from_x as isize - to_x as isize) == 0
                && (from_y as isize - to_y as isize) != 0
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Queen) = selected_piece {
            if (from_x as isize - to_x as isize) != 0 && (from_y as isize - to_y as isize) == 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if (from_x as isize - to_x as isize) == 0
                && (from_y as isize - to_y as isize) != 0
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if (from_x as isize - to_x as isize).abs()
                == (from_y as isize - to_y as isize).abs()
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::King) = selected_piece {
            if (from_x as isize - to_x as isize).abs() <= 1
                && (from_y as isize - to_y as isize).abs() <= 1
            {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else {
            self.tiles[to_y][to_x] = selected_piece;
            self.tiles[from_y][from_x] = Tile::Empty;
        }
        return self;
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;
    #[test]
    fn print_piece() {
        let board = Board::new();
        let (from_x, from_y) = input();
        if let Tile::Occupied(_, piece) = board.tiles[from_y][from_x] {
            println!("{:?}", piece);
        }
    }
    #[test]
    fn test_move_piece_basic() {
        let board = Board::new();
        let (from_x, from_y) = input();
        let (to_x, to_y) = input();
        let board = board.move_piece_basic(from_x, from_y, to_x, to_y);
        Board::print_board(board);
    }
    #[test]
    fn test_input() {
        let (x, y) = input();
        println!("{:?}", (x, y));
    }
    #[test]
    fn test_move_piece() {
        let board = Board::new();
        let (from_x, from_y) = input();
        let (to_x, to_y) = input();
        let board = board.move_piece(from_x, from_y, to_x, to_y);
        Board::print_board(board);
    }
    #[test]
    fn testing() {
        println!("♔");
    }
}
