use crate::pieces::*;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Occupied(Color, Piece),
}
struct Board {
    tiles: [[Tile; 8]; 8],
}

impl Board {
    fn new() -> Self {
        // Create Start configuration
        let mut tiles = [[Tile::Empty; 8]; 8];
        let white_pawns = [Tile::Occupied(Color::White, Piece::Pawn); 8];
        let black_pawns = [Tile::Occupied(Color::Black, Piece::Pawn); 8];
        let white_back_rank = [
            Tile::Occupied(Color::White, Piece::Rook),
            Tile::Occupied(Color::White, Piece::Bishop),
            Tile::Occupied(Color::White, Piece::Knight),
            Tile::Occupied(Color::White, Piece::Queen),
            Tile::Occupied(Color::White, Piece::King),
            Tile::Occupied(Color::White, Piece::Knight),
            Tile::Occupied(Color::White, Piece::Bishop),
            Tile::Occupied(Color::White, Piece::Rook),
        ];
        let black_back_rank = [
            Tile::Occupied(Color::Black, Piece::Rook),
            Tile::Occupied(Color::Black, Piece::Bishop),
            Tile::Occupied(Color::Black, Piece::Knight),
            Tile::Occupied(Color::Black, Piece::Queen),
            Tile::Occupied(Color::Black, Piece::King),
            Tile::Occupied(Color::Black, Piece::Knight),
            Tile::Occupied(Color::Black, Piece::Bishop),
            Tile::Occupied(Color::Black, Piece::Rook),
        ];
        (tiles[0], tiles[1], tiles[7], tiles[6]) =
            (black_back_rank, black_pawns, white_back_rank, white_pawns);
        Board { tiles }
    }
}
