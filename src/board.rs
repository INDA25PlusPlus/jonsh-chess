use crate::pieces::*;

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Occupied(Color, Piece),
}
#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub tiles: [[Tile; 8]; 8],
    pub black_king_pos: (usize, usize),
    pub white_king_pos: (usize, usize),
    pub turn: Color,
    pub en_passant_target: Option<(usize, usize)>,
    pub white_castle: bool,
    pub black_castel: bool,
}

impl Tile {
    pub fn color(&self) -> Option<Color> {
        if let Tile::Occupied(color, _) = self {
            Some(*color)
        } else {
            None
        }
    }
}

impl Board {
    pub fn new() -> Self {
        // Create Start configuration
        let mut tiles = [[Tile::Empty; 8]; 8];
        let white_pawns = [Tile::Occupied(Color::White, Piece::Pawn); 8];
        let black_pawns = [Tile::Occupied(Color::Black, Piece::Pawn); 8];
        let white_back_rank = [
            Tile::Occupied(Color::White, Piece::Rook),
            Tile::Occupied(Color::White, Piece::Knight),
            Tile::Occupied(Color::White, Piece::Bishop),
            Tile::Occupied(Color::White, Piece::Queen),
            Tile::Occupied(Color::White, Piece::King),
            Tile::Occupied(Color::White, Piece::Bishop),
            Tile::Occupied(Color::White, Piece::Knight),
            Tile::Occupied(Color::White, Piece::Rook),
        ];
        let black_back_rank = [
            Tile::Occupied(Color::Black, Piece::Rook),
            Tile::Occupied(Color::Black, Piece::Knight),
            Tile::Occupied(Color::Black, Piece::Bishop),
            Tile::Occupied(Color::Black, Piece::Queen),
            Tile::Occupied(Color::Black, Piece::King),
            Tile::Occupied(Color::Black, Piece::Bishop),
            Tile::Occupied(Color::Black, Piece::Knight),
            Tile::Occupied(Color::Black, Piece::Rook),
        ];
        (tiles[0], tiles[1], tiles[7], tiles[6]) =
            (black_back_rank, black_pawns, white_back_rank, white_pawns);
        Board {
            tiles,
            black_king_pos: (4, 0),
            white_king_pos: (4, 7),
            turn: Color::White,
            en_passant_target: None,
            white_castle: true,
            black_castel: true,
        }
    }
    pub fn print_board(self) {
        for i in 0..8 {
            println!("{:?}", self.tiles[i]);
        }
    }
    pub fn in_bounds(x: isize, y: isize) -> bool {
        (0..8).contains(&x) && (0..8).contains(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_tiles() {
        let board = Board::new();
        for i_1 in 0..8 {
            for i_2 in 0..8 {
                println!("{:?}", board.tiles[i_1][i_2]);
            }
        }
    }
}
