#[derive(Clone, Copy)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}
#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Occupied(Color, Piece),
}
