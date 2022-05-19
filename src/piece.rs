use std::fmt;

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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.piece {
            PieceType::Pawn => {
                if self.color == Color::White {
                    "♙"
                } else {
                    "♟"
                }
            }
            PieceType::Knight => {
                if self.color == Color::White {
                    "♘"
                } else {
                    "♞"
                }
            }
            PieceType::Bishop => {
                if self.color == Color::White {
                    "♗"
                } else {
                    "♝"
                }
            }
            PieceType::Rook => {
                if self.color == Color::White {
                    "♖"
                } else {
                    "♜"
                }
            }
            PieceType::Queen => {
                if self.color == Color::White {
                    "♕"
                } else {
                    "♛"
                }
            }
            PieceType::King => {
                if self.color == Color::White {
                    "♔"
                } else {
                    "♚"
                }
            }
        };
        write!(f, "{}", s)
    }
}
