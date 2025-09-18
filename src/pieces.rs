#[derive(Clone, Copy, Debug)]
pub enum Piece {
    GhostPawn,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Copy, Debug)]
pub enum PromoPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}
