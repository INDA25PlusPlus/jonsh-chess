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
            if from_x.abs_diff(to_x) == 1 && from_y.abs_diff(to_y) == 2 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if from_x.abs_diff(to_x) == 2 && from_y.abs_diff(to_y) == 1 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Bishop) = selected_piece {
            if from_x.abs_diff(to_x) == from_y.abs_diff(to_y) {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Rook) = selected_piece {
            if from_x.abs_diff(to_x) != 0 && from_y.abs_diff(to_y) == 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if from_x.abs_diff(to_x) == 0 && from_y.abs_diff(to_y) != 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::Queen) = selected_piece {
            if from_x.abs_diff(to_x) != 0 && from_y.abs_diff(to_y) == 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if from_x.abs_diff(to_x) == 0 && from_y.abs_diff(to_y) != 0 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            } else if from_x.abs_diff(to_x) == from_y.abs_diff(to_y) {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else if let Tile::Occupied(_, Piece::King) = selected_piece {
            if from_x.abs_diff(to_x) <= 1 && from_y.abs_diff(to_y) <= 1 {
                self.tiles[to_y][to_x] = selected_piece;
                self.tiles[from_y][from_x] = Tile::Empty;
            }
        } else {
            self.tiles[to_y][to_x] = selected_piece;
            self.tiles[from_y][from_x] = Tile::Empty;
        }
        return self;
    }
    pub fn valid_moves(self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let selected_piece = self.tiles[y][x];
        let mut valid_moves: Vec<(usize, usize)> = Vec::new();

        //Pawn
        if let Tile::Occupied(_, Piece::Pawn) = selected_piece {
            //##################################################################################
            //Black
            if let Tile::Occupied(Color::Black, _) = selected_piece {
                if let Tile::Empty = self.tiles[y + 1][x] {
                    if let Tile::Empty = self.tiles[y + 2][x]
                        && y == 1
                    {
                        valid_moves = vec![(y + 1, x), (y + 2, x)];
                    } else {
                        valid_moves = vec![(y + 1, x)];
                    }
                }
            //##################################################################################
            //White
            } else if let Tile::Occupied(Color::White, _) = selected_piece {
                if let Tile::Empty = self.tiles[y - 1][x] {
                    if let Tile::Empty = self.tiles[y - 2][x]
                        && y == 6
                    {
                        valid_moves = vec![(y - 1, x), (y - 2, x)];
                    } else {
                        valid_moves.push((y - 1, x));
                    }
                }
            }
        }
        //##################################################################################
        //Knight
        else if let Tile::Occupied(_, Piece::Knight) = selected_piece {
            for i_1 in -2isize..=2isize {
                for i_2 in -2isize..=2isize {
                    if y as isize + i_2 >= 0
                        && y as isize + i_2 < 8
                        && x as isize + i_1 >= 0
                        && x as isize + i_1 < 8
                    {
                        if let Tile::Empty =
                            self.tiles[(y as isize + i_2) as usize][(x as isize + i_1) as usize]
                            && ((i_1.abs() == 1 && i_2.abs() == 2)
                                || (i_1.abs() == 2 && i_2.abs() == 1))
                        {
                            valid_moves
                                .push(((y as isize + i_2) as usize, (x as isize + i_1) as usize))
                        }
                    }
                }
            }
        }
        //##################################################################################
        //Bishop
        else if let Tile::Occupied(_, Piece::Bishop) = selected_piece {
            //Diagonal Down-Right
            for i_1 in 1isize..=(7 - y).min(7 - x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize + i_1) as usize][(x as isize + i_1) as usize]
                {
                    valid_moves.push(((y as isize + i_1) as usize, (x as isize + i_1) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Down-Left
            for i_2 in 1isize..=(7 - y).min(x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize + i_2) as usize][(x as isize - i_2) as usize]
                {
                    valid_moves.push(((y as isize + i_2) as usize, (x as isize - i_2) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Upp-Right
            for i_3 in 1isize..=(y).min(7 - x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize - i_3) as usize][(x as isize + i_3) as usize]
                {
                    valid_moves.push(((y as isize - i_3) as usize, (x as isize + i_3) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Upp-Left
            for i_4 in 1isize..=y.min(x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize - i_4) as usize][(x as isize - i_4) as usize]
                {
                    valid_moves.push(((y as isize - i_4) as usize, (x as isize - i_4) as usize));
                } else {
                    break;
                }
            }
        }
        //##################################################################################
        //Rook
        else if let Tile::Occupied(_, Piece::Rook) = selected_piece {
            //Right
            for i_1 in 1isize..=7 - x as isize {
                if let Tile::Empty = self.tiles[y][(x as isize + i_1) as usize] {
                    valid_moves.push((y, (x as isize + i_1) as usize));
                } else {
                    break;
                }
            }
            //Left
            for i_2 in 1isize..=x as isize {
                if let Tile::Empty = self.tiles[y][(x as isize - i_2) as usize] {
                    valid_moves.push((y, (x as isize - i_2) as usize));
                } else {
                    break;
                }
            }
            //Down
            for i_3 in 1isize..=7 - y as isize {
                if let Tile::Empty = self.tiles[(y as isize + i_3) as usize][x] {
                    valid_moves.push(((y as isize + i_3) as usize, x));
                } else {
                    break;
                }
            }
            //Upp
            for i_4 in 1isize..=y as isize {
                if let Tile::Empty = self.tiles[(y as isize - i_4) as usize][x] {
                    valid_moves.push(((y as isize - i_4) as usize, x));
                } else {
                    break;
                }
            }
        }
        //##################################################################################
        //Queen
        else if let Tile::Occupied(_, Piece::Queen) = selected_piece {
            //Diagonal Down-Right
            for i_1 in 1isize..=(7 - y).min(7 - x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize + i_1) as usize][(x as isize + i_1) as usize]
                {
                    valid_moves.push(((y as isize + i_1) as usize, (x as isize + i_1) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Down-Left
            for i_2 in 1isize..=(7 - y).min(x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize + i_2) as usize][(x as isize - i_2) as usize]
                {
                    valid_moves.push(((y as isize + i_2) as usize, (x as isize - i_2) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Upp-Right
            for i_3 in 1isize..=(y).min(7 - x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize - i_3) as usize][(x as isize + i_3) as usize]
                {
                    valid_moves.push(((y as isize - i_3) as usize, (x as isize + i_3) as usize));
                } else {
                    break;
                }
            }
            //Diagonal Upp-Left
            for i_4 in 1isize..=y.min(x) as isize {
                if let Tile::Empty =
                    self.tiles[(y as isize - i_4) as usize][(x as isize - i_4) as usize]
                {
                    valid_moves.push(((y as isize - i_4) as usize, (x as isize - i_4) as usize));
                } else {
                    break;
                }
            }
            //Right
            for i_1 in 1isize..=7 - x as isize {
                if let Tile::Empty = self.tiles[y][(x as isize + i_1) as usize] {
                    valid_moves.push((y, (x as isize + i_1) as usize));
                } else {
                    break;
                }
            }
            //Left
            for i_2 in 1isize..=x as isize {
                if let Tile::Empty = self.tiles[y][(x as isize - i_2) as usize] {
                    valid_moves.push((y, (x as isize - i_2) as usize));
                } else {
                    break;
                }
            }
            //Down
            for i_3 in 1isize..=7 - y as isize {
                if let Tile::Empty = self.tiles[(y as isize + i_3) as usize][x] {
                    valid_moves.push(((y as isize + i_3) as usize, x));
                } else {
                    break;
                }
            }
            //Upp
            for i_4 in 1isize..=y as isize {
                if let Tile::Empty = self.tiles[(y as isize - i_4) as usize][x] {
                    valid_moves.push(((y as isize - i_4) as usize, x));
                } else {
                    break;
                }
            }
        }
        //##################################################################################
        //King
        else if let Tile::Occupied(_, Piece::King) = selected_piece {
            for i_1 in [1.min(7 - y as isize), (-1).max(0 - y as isize), 0 as isize] {
                for i_2 in [1.min(7 - x as isize), (-1).max(0 - x as isize), 0 as isize] {
                    if let Tile::Empty =
                        self.tiles[(y as isize + i_1) as usize][(x as isize + i_2) as usize]
                    {
                        valid_moves
                            .push(((y as isize + i_1) as usize, (x as isize + i_2) as usize));
                    }
                }
            }
        }
        return valid_moves;
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::input;

    use super::*;
    #[test]
    fn test_move_time() {
        let board = Board::new();
        let (from_x, from_y, to_x, to_y): (usize, usize, usize, usize) = (1usize, 7, 2, 5);
        let board = board.move_piece(from_x, from_y, to_x, to_y);
        Board::print_board(board);
    }

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
    fn test_valid_moves() {
        let board = Board::new();
        let (x, y) = input();
        let board = board.move_piece(5, 6, 5, 4);
        println!("{:?}", board.valid_moves(x, y));
        Board::print_board(board);
    }
    #[test]
    fn time_test_valid_moves() {
        let board = Board::new();
        for i_1 in 0..8 {
            for i_2 in 0..8 {
                board.valid_moves(i_1, i_2);
            }
        }
    }
}
