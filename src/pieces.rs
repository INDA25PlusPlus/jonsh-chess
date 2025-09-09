#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}
