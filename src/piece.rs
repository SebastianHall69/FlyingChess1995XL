use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, PartialEq)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn from(color: PieceColor, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }

    pub fn chess_dot_com_char_to_piece_color(c: char) -> Result<PieceColor, Box<dyn Error>> {
        match c {
            'w' => Ok(PieceColor::White),
            'b' => Ok(PieceColor::Black),
            _ => Err(Box::from(format!(
                "Could not parse char '{c}' into a valid color"
            ))),
        }
    }

    pub fn chess_dot_com_char_to_piece_type(c: char) -> Result<PieceType, Box<dyn Error>> {
        match c {
            'p' => Ok(PieceType::Pawn),
            'b' => Ok(PieceType::Bishop),
            'n' => Ok(PieceType::Knight),
            'r' => Ok(PieceType::Rook),
            'q' => Ok(PieceType::Queen),
            'k' => Ok(PieceType::King),
            _ => Err(Box::from(format!(
                "Could not parse char '{c}' into a valid piece type"
            ))),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Bishop => "b",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };
        write!(f, "{}", display_str)
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            PieceColor::White => "w",
            PieceColor::Black => "b",
        };
        write!(f, "{}", display_str)
    }
}
