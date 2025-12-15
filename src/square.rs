use crate::piece::Piece;
use anyhow::Context;
use std::error::Error;
use std::fmt::{Display, Formatter};

const RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
const FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(Clone, PartialEq)]
pub struct Square {
    pub piece: Option<Piece>,
    rank: String,
    file: String,
}

impl Square {
    pub fn from(rank: String, file: String, piece: Option<Piece>) -> Self {
        Self { rank, file, piece }
    }

    pub fn from_uci_notation(notation: &String) -> Result<Self, Box<dyn Error>> {
        let mut chars = notation.chars();

        let file = chars
            .next()
            .context(format!("Unable to parse file from: {notation}"))?
            .to_string();
        let rank = chars
            .next()
            .context(format!("Unable to parse rank from: {notation}"))?
            .to_string();

        Ok(Square {
            rank,
            file,
            piece: None,
        })
    }

    pub fn rank_index(&self) -> Result<usize, Box<dyn Error>> {
        RANKS
            .iter()
            .position(|item| *item == self.rank.as_str())
            .ok_or_else(|| Box::from(format!("Found an invalid rank: {}", self.rank)))
    }

    pub fn file_index(&self) -> Result<usize, Box<dyn Error>> {
        FILES
            .iter()
            .position(|item| *item == self.file.as_str())
            .ok_or_else(|| Box::from(format!("Found an invalid file: {}", self.file)))
    }

    pub fn index_to_rank(index: usize) -> Result<String, Box<dyn Error>> {
        RANKS
            .get(index)
            .map(|rank| rank.to_string())
            .ok_or_else(|| Box::from(format!("Found invalid index for file: {index}")))
    }

    pub fn index_to_file(index: usize) -> Result<String, Box<dyn Error>> {
        FILES
            .get(index)
            .map(|file| file.to_string())
            .ok_or_else(|| Box::from(format!("Found invalid index for file: {index}")))
    }

    pub fn chess_dot_com_index_char_to_index(c: char) -> Result<usize, Box<dyn Error>> {
        let index = (c
            .to_digit(10)
            .context(format!("Could not convert char index '{c}' to a digit"))?
            - 1) as usize;
        Ok(index)
    }

    pub fn uci_notation(&self) -> String {
        format!("{}{}", self.file, self.rank)
    }

    pub fn get_offset_from_board_center(
        &self,
        board_width: f64,
        is_board_flipped: bool,
    ) -> Result<(i64, i64), Box<dyn Error>> {
        let square_width = board_width / 8.0;
        let square_center_offset = square_width / 2.0;

        let rank_index_from_center = match self.rank_index()? {
            7 => -4,
            6 => -3,
            5 => -2,
            4 => -1,
            3 => 0,
            2 => 1,
            1 => 2,
            0 => 3,
            bad_rank => {
                return Err(Box::from(format!(
                    "Failed to find center offset with rank: {bad_rank}"
                )));
            }
        };

        let file_index_from_center = match self.file_index()? {
            0 => -4,
            1 => -3,
            2 => -2,
            3 => -1,
            4 => 0,
            5 => 1,
            6 => 2,
            7 => 3,
            bad_file => {
                return Err(Box::from(format!(
                    "Failed to find center offset with file: {bad_file}"
                )));
            }
        };

        let rank_offset_from_center =
            (rank_index_from_center as f64 * square_width + square_center_offset) as i64;
        let file_offset_from_center =
            (file_index_from_center as f64 * square_width + square_center_offset) as i64;

        match is_board_flipped {
            true => Ok((-rank_offset_from_center, -file_offset_from_center)),
            false => Ok((rank_offset_from_center, file_offset_from_center)),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uci_notation())
    }
}
