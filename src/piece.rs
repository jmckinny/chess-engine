#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    color: Color,
    piece: PieceType,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn new(color: Color, piece: PieceType) -> Self {
        Piece { color, piece }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_type(&self) -> PieceType {
        self.piece
    }
}
