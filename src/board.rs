use std::result;

use crate::piece::{Color, Piece, PieceType};

type Coordinate = (u8, u8);
#[derive(Debug)]
pub struct Board {
    squares: Vec<Vec<Option<Piece>>>,
    to_move: Color,
    w_castle_kingside: bool,
    w_castle_queenside: bool,
    b_castle_kingside: bool,
    b_castle_queenside: bool,
    en_passant: Option<Coordinate>,
    halfmoves: u32,
    fullmoves: u32,
}

impl Board {
    pub fn from_fen(fen_str: &str) -> Result<Self, &str> {
        let split: Vec<&str> = fen_str.split_whitespace().collect();

        let pieces_str = split
            .get(0)
            .ok_or("FEN parse error: No pieces found in fen string")?;
        let to_move_str = split.get(1).ok_or("FEN parse error: No to_move found")?;
        let castling_str = split.get(2).ok_or("FEN parse error: no castling found")?;
        let en_passant_str = split.get(3).ok_or("FEN parse error: no en passant found")?;
        let fullmove_str = split
            .get(4)
            .ok_or("FEN parse error: no en fullmove found")?;
        let halfmove_str = split
            .get(5)
            .ok_or("FEN parse error: no en halfmove found")?;

        let squares = parse_pieces(pieces_str)?;
        let to_move = match *to_move_str {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err("FEN parse error: to move color invalid"),
        }?;
        let w_castle_kingside = castling_str.contains('K');
        let w_castle_queenside = castling_str.contains('Q');
        let b_castle_kingside = castling_str.contains('k');
        let b_castle_queenside = castling_str.contains('q');
        let en_passant = parse_coordinate(en_passant_str)?;
        let fullmoves = fullmove_str
            .parse::<u32>()
            .or(Err("FEN parse error: fullmove invalid"))?;
        let halfmoves = halfmove_str
            .parse::<u32>()
            .or(Err("FEN parse error: halfmove invalid"))?;

        let result = Board {
            squares,
            to_move,
            w_castle_kingside,
            w_castle_queenside,
            b_castle_kingside,
            b_castle_queenside,
            en_passant,
            halfmoves,
            fullmoves,
        };
        Ok(result)
    }

    pub fn to_fen(&self) -> String {
        [
            self.pieces_to_fen().as_str(),
            if self.to_move == Color::White {
                "w".to_string()
            } else {
                "b".to_string()
            }
            .as_str(),
            self.build_castle_fen().as_str(),
            coordinate_to_notation(self.en_passant).as_str(),
            self.halfmoves.to_string().as_str(),
            self.fullmoves.to_string().as_str(),
        ]
        .join(" ")
    }

    fn build_castle_fen(&self) -> String {
        let mut result = String::new();
        if self.w_castle_kingside {
            result.push('K');
        }
        if self.w_castle_queenside {
            result.push('Q');
        }
        if self.w_castle_kingside {
            result.push('k');
        }
        if self.w_castle_queenside {
            result.push('q');
        }

        result
    }

    fn pieces_to_fen(&self) -> String {
        let mut result = Vec::new();
        for rank in &self.squares {
            let mut s = String::new();
            for col in rank {
                if let Some(p) = col {
                    s.push_str(piece_to_fen(p).as_str());
                } else {
                    s.push('1');
                }
            }
            result.push(s);
        }
        result.join("/")
    }
}

fn piece_to_fen(p: &Piece) -> String {
    let piece_type = match p.get_type() {
        PieceType::Pawn => 'p',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::Rook => 'r',
        PieceType::Queen => 'q',
        PieceType::King => 'k',
    };
    if p.get_color() == Color::White {
        piece_type.to_uppercase().to_string()
    } else {
        piece_type.to_string()
    }
}

fn parse_coordinate(notation_form: &str) -> Result<Option<Coordinate>, &str> {
    if notation_form == "-" {
        Ok(None)
    } else {
        let row = notation_form
            .chars()
            .nth(1)
            .ok_or("Coordinate parse error: invalid row")?;
        let y = row
            .to_digit(10)
            .ok_or("Coordinate parse error: invalid row")? as u8
            - 1;
        let col = notation_form
            .chars()
            .next()
            .ok_or("Coordinate parse error: invalid coordinate")?;
        let x = match col {
            'a' => Ok(0),
            'b' => Ok(1),
            'c' => Ok(2),
            'd' => Ok(3),
            'e' => Ok(4),
            'f' => Ok(5),
            'g' => Ok(6),
            'h' => Ok(7),
            _ => Err("Coordinate parse error: invalid column"),
        }?;
        Ok(Some((x, y)))
    }
}

fn coordinate_to_notation(cord: Option<Coordinate>) -> String {
    if let Some((col, row)) = cord {
        let mut col_str = match col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!("Invalid coordinate number"),
        }
        .to_string();
        col_str.push_str((row + 1).to_string().as_str());
        col_str
    } else {
        "-".to_string()
    }
}

fn parse_pieces(to_parse: &str) -> Result<Vec<Vec<Option<Piece>>>, &str> {
    let mut result = Vec::new();
    let ranks = to_parse.split('/');

    for rank in ranks {
        let cur = parse_rank(rank)?;
        result.push(cur);
    }
    Ok(result)
}

fn parse_rank(rank_str: &str) -> Result<Vec<Option<Piece>>, &str> {
    let mut result: Vec<Option<Piece>> = Vec::new();
    for c in rank_str.chars() {
        match c {
            'P' => result.push(Some(Piece::new(Color::White, PieceType::Pawn))),
            'N' => result.push(Some(Piece::new(Color::White, PieceType::Knight))),
            'B' => result.push(Some(Piece::new(Color::White, PieceType::Bishop))),
            'R' => result.push(Some(Piece::new(Color::White, PieceType::Rook))),
            'Q' => result.push(Some(Piece::new(Color::White, PieceType::Queen))),
            'K' => result.push(Some(Piece::new(Color::White, PieceType::King))),
            'p' => result.push(Some(Piece::new(Color::Black, PieceType::Pawn))),
            'n' => result.push(Some(Piece::new(Color::Black, PieceType::Knight))),
            'b' => result.push(Some(Piece::new(Color::Black, PieceType::Bishop))),
            'r' => result.push(Some(Piece::new(Color::Black, PieceType::Rook))),
            'q' => result.push(Some(Piece::new(Color::Black, PieceType::Queen))),
            'k' => result.push(Some(Piece::new(Color::Black, PieceType::King))),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                let num = c
                    .to_digit(10)
                    .ok_or("FEN parse error: invlaid rank number")?;
                for _ in 0..num {
                    result.push(None)
                }
            }
            _ => return Err("FEN parse error: invlaid rank symbol"),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_fen() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board.to_move, Color::White);
        assert!(board.w_castle_kingside);
        assert!(board.w_castle_queenside);
        assert!(board.b_castle_kingside);
        assert!(board.b_castle_queenside);
        assert_eq!(board.en_passant, None);
        assert_eq!(board.fullmoves, 0);
        assert_eq!(board.halfmoves, 1);
    }
    #[test]
    fn test_bad_fen() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR c KQkq - 0 1");
        let expected = Err::<Board, &str>("FEN parse error: to move color invalid");
        assert_eq!(board.err(), expected.err());
    }
    #[test]
    fn fen_convert() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(
            board.to_fen().as_str(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
