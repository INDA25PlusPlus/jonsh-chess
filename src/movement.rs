use crate::board::{Board, Tile};
use crate::pieces::{Color, Piece};

impl Board {
    pub fn move_piece(&mut self, fx: usize, fy: usize, tx: usize, ty: usize) {
        let selected_piece = self.tiles[fy][fx];
        if let Tile::Occupied(color, _) = self.tiles[fy][fx]
            && color == self.turn
        {
            if self.legal_moves(fx, fy).contains(&(tx, ty)) {
                self.tiles[ty][tx] = selected_piece;
                self.tiles[fy][fx] = Tile::Empty;
                if self.turn == Color::White {
                    self.turn = Color::Black;
                } else {
                    self.turn = Color::White;
                }
                if let Tile::Occupied(color, Piece::King) = selected_piece {
                    if color == Color::White {
                        self.white_king_pos = (tx, ty);
                    } else {
                        self.black_king_pos = (tx, ty);
                    }
                }
            }
        }
    }

    //Get Color of Piece. Return True for white and False for black.
    pub fn get_color(&self, x: usize, y: usize) -> bool {
        if let Tile::Occupied(Color::White, _) = self.tiles[y][x] {
            return true;
        } else if let Tile::Occupied(Color::Black, _) = self.tiles[y][x] {
            return false;
        } else {
            unimplemented!();
        }
    }
    pub fn legal_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        let fake_moves = self.valid_moves(x, y);

        if let Tile::Occupied(color, piece) = self.tiles[y][x] {
            for (nx, ny) in fake_moves {
                let mut new_board = self.clone();
                let selected_piece = new_board.tiles[y][x];
                new_board.tiles[ny][nx] = selected_piece;
                new_board.tiles[y][x] = Tile::Empty;

                if let Piece::King = piece {
                    if color == Color::White {
                        new_board.white_king_pos = (nx, ny);
                    } else {
                        new_board.black_king_pos = (nx, ny);
                    }
                }

                let (is_check, white) = new_board.is_check();

                if is_check
                    && ((color == Color::White && white) || (color == Color::Black && !white))
                {
                    continue;
                } else {
                    moves.push((nx, ny));
                }
            }
        }
        return moves;
    }
    //Get valid moves
    pub fn valid_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let selected_piece = self.tiles[y][x];
        let mut valid_moves: Vec<(usize, usize)> = Vec::new();
        let i: isize;
        //Pawn
        if let Tile::Occupied(color, Piece::Pawn) = selected_piece {
            if color == Color::White {
                i = -1;
            } else {
                i = 1;
            }
            if Board::in_bounds(x as isize, y as isize + i) {
                if let Tile::Empty = self.tiles[(y as isize + i) as usize][x] {
                    valid_moves.push((x, (y as isize + i) as usize));
                    if let Tile::Empty = self.tiles[(y as isize + i * 2) as usize][x]
                        && ((y == 6 && color == Color::White) || (y == 1 && color == Color::Black))
                    {
                        valid_moves.push((x, (y as isize + 2 * i) as usize));
                    }
                }
            }
            for k in [-1, 1] {
                if Board::in_bounds(x as isize + k, y as isize + i) {
                    if let Tile::Occupied(color, _) =
                        self.tiles[(y as isize + i) as usize][(x as isize + k) as usize]
                        && color != selected_piece.color().unwrap()
                    {
                        valid_moves.push(((x as isize + k) as usize, (y as isize + i) as usize));
                    }
                }
            }
        }
        //Knight
        else if let Tile::Occupied(_, Piece::Knight) = selected_piece {
            for i_x in [-1, -2, 1, 2] {
                for i_y in [-1, -2, 1, 2] {
                    let nx = x as isize + i_x;
                    let ny = y as isize + i_y;
                    if Board::in_bounds(nx, ny) {
                        if let Tile::Empty = self.tiles[(ny) as usize][(nx) as usize]
                            && ((i_x.abs() == 1 && i_y.abs() == 2)
                                || (i_x.abs() == 2 && i_y.abs() == 1))
                        {
                            valid_moves.push(((nx) as usize, (ny) as usize));
                        }
                        if let Tile::Occupied(color, _) = self.tiles[(ny) as usize][(nx) as usize]
                            && color != selected_piece.color().unwrap()
                        {
                            valid_moves.push(((nx) as usize, (ny) as usize));
                        }
                    }
                }
            }
        }
        //Bishop
        else if let Tile::Occupied(_, Piece::Bishop) = selected_piece {
            for (dx, dy) in [(1, 1), (1, -1), (-1, -1), (-1, 1)] {
                for i in 1..8 {
                    let nx = x as isize + i * dx;
                    let ny = y as isize + i * dy;

                    if !Board::in_bounds(nx, ny) {
                        break;
                    }

                    if let Tile::Empty = self.tiles[ny as usize][nx as usize] {
                        valid_moves.push((nx as usize, ny as usize));
                    } else if let Tile::Occupied(color, _) = self.tiles[ny as usize][nx as usize]
                        && color != selected_piece.color().unwrap()
                    {
                        valid_moves.push((nx as usize, ny as usize));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
        //Rook
        else if let Tile::Occupied(_, Piece::Rook) = selected_piece {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                for i in 1..8 {
                    let nx = x as isize + i * dx;
                    let ny = y as isize + i * dy;

                    if !Board::in_bounds(nx, ny) {
                        break;
                    }

                    if let Tile::Empty = self.tiles[ny as usize][nx as usize] {
                        valid_moves.push((nx as usize, ny as usize));
                    } else if let Tile::Occupied(color, _) = self.tiles[ny as usize][nx as usize]
                        && color != selected_piece.color().unwrap()
                    {
                        valid_moves.push((nx as usize, ny as usize));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
        //Queen
        else if let Tile::Occupied(_, Piece::Queen) = selected_piece {
            for (dx, dy) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                for i in 1..8 {
                    let nx = x as isize + i * dx;
                    let ny = y as isize + i * dy;

                    if !Board::in_bounds(nx, ny) {
                        break;
                    }

                    if let Tile::Empty = self.tiles[ny as usize][nx as usize] {
                        valid_moves.push((nx as usize, ny as usize));
                    } else if let Tile::Occupied(color, _) = self.tiles[ny as usize][nx as usize]
                        && color != selected_piece.color().unwrap()
                    {
                        valid_moves.push((nx as usize, ny as usize));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
        //King
        else if let Tile::Occupied(_, Piece::King) = selected_piece {
            for (dx, dy) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if Board::in_bounds(nx, ny) {
                    if let Tile::Empty = self.tiles[ny as usize][nx as usize] {
                        valid_moves.push((nx as usize, ny as usize));
                    } else if let Tile::Occupied(color, _) = self.tiles[ny as usize][nx as usize]
                        && color != selected_piece.color().unwrap()
                    {
                        valid_moves.push((nx as usize, ny as usize));
                    } else {
                    }
                }
            }
        }
        return valid_moves;
    }
    //Is king in check
    pub fn is_check(&self) -> (bool, bool) {
        let mut value = (false, false);
        for iy in 0..8 {
            for ix in 0..8 {
                if let Tile::Occupied(color, _) = self.tiles[iy][ix] {
                    let moves = self.valid_moves(ix, iy);
                    for (nx, ny) in moves {
                        if color == Color::Black && (nx, ny) == self.white_king_pos {
                            value = (true, true);
                        } else if color == Color::White && (nx, ny) == self.black_king_pos {
                            value = (true, false);
                        }
                    }
                }
            }
        }
        value
    }
    pub fn game_end(&self) -> (bool, bool, bool) {
        let (is_check, is_white) = self.is_check();
        let (mut is_checkmate, mut is_stalemate) = (false, false);
        if is_check {
            is_checkmate = true;
            if is_white {
                'outer: for iy in 0..8 {
                    for ix in 0..8 {
                        if let Tile::Occupied(Color::White, _) = self.tiles[iy][ix] {
                            if !self.legal_moves(ix, iy).is_empty() {
                                is_checkmate = false;
                                break 'outer;
                            }
                        }
                    }
                }
            } else {
                'outer: for iy in 0..8 {
                    for ix in 0..8 {
                        if let Tile::Occupied(Color::Black, _) = self.tiles[iy][ix] {
                            if !self.legal_moves(ix, iy).is_empty() {
                                is_checkmate = false;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        } else {
            is_stalemate = true;
            if self.turn == Color::White {
                'outer: for iy in 0..8 {
                    for ix in 0..8 {
                        if let Tile::Occupied(Color::White, _) = self.tiles[iy][ix] {
                            if !self.legal_moves(ix, iy).is_empty() {
                                is_stalemate = false;
                                break 'outer;
                            }
                        }
                    }
                }
            } else {
                'outer: for iy in 0..8 {
                    for ix in 0..8 {
                        if let Tile::Occupied(Color::Black, _) = self.tiles[iy][ix] {
                            if !self.legal_moves(ix, iy).is_empty() {
                                is_stalemate = false;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        return (is_checkmate, is_white, is_stalemate);
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
    fn test_input() {
        let (x, y) = input();
        println!("{:?}", (x, y));
    }
    #[test]
    fn test_valid_moves() {
        let mut board = Board::new();
        // board = board.move_piece(4, 1, 4, 3);
        let (x, y) = input();
        board.tiles[4][4] = Tile::Occupied(Color::White, Piece::King);
        board.tiles[5][4] = Tile::Occupied(Color::Black, Piece::Pawn);
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
    #[test]
    fn test_new_moves() {
        let mut board = Board::new();
        board.move_piece(4, 6, 4, 4);
        board.move_piece(4, 1, 4, 3);
        board.move_piece(5, 7, 1, 3);
        board.move_piece(0, 6, 0, 4);
        board.print_board();
    }
    #[test]
    fn test_game_end() {
        let mut board = Board::new();
        board.move_piece(5, 6, 5, 5);
        board.move_piece(4, 1, 4, 3);
        board.move_piece(6, 6, 6, 4);
        board.move_piece(3, 0, 7, 4);
        board.print_board();

        println!("{:?}", board.valid_moves(3, 0));
        println!("{:?}", board.is_check());
        println!("{:?}", board.game_end());
    }
}
