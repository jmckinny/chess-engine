#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Piece{
    color: Color,
    piece: PieceType
}
#[derive(Debug,Clone, Copy, PartialEq)]
pub enum Color{
    Black,
    White
}
#[derive(Debug,Clone, Copy,PartialEq)]
pub enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

