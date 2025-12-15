use crate::piece::PieceType;
use crate::square::Square;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct ChessMove {
    pub start: Square,
    pub end: Square,
}

impl ChessMove {
    pub fn from(start: Square, end: Square) -> Self {
        Self { start, end }
    }

    pub fn from_uci_notation(notation: &String) -> Result<Self, Box<dyn Error>> {
        // e2e4
        let mut chars = notation.chars();

        let start_notation: String = chars.by_ref().take(2).collect();
        let start = Square::from_uci_notation(&start_notation)?;

        let end_notation: String = chars.by_ref().take(2).collect();
        let end = Square::from_uci_notation(&end_notation)?;

        Ok(Self { start, end })
    }

    pub fn uci_notation(&self) -> String {
        let promotion_uci_notation = self
            .get_promotion_type()
            .map_or(String::new(), |item| item.to_string());
        format!(
            "{}{}{}",
            self.start.uci_notation(),
            self.end.uci_notation(),
            promotion_uci_notation
        )
    }

    fn get_promotion_type(&self) -> Option<PieceType> {
        match self.start.piece.as_ref()?.piece_type == self.end.piece.as_ref()?.piece_type {
            true => None,
            false => Some(self.end.piece.as_ref()?.piece_type),
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uci_notation())
    }
}
