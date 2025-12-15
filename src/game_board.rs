use crate::chess_move::ChessMove;
use crate::piece::{Piece, PieceColor, PieceType};
use crate::square::Square;
use anyhow::Context;
use std::error::Error;
use std::fmt::{Display, Formatter};

const BOARD_SIZE: usize = 8;
const KING_FILE: usize = 4;
const WHITE_KING_RANK: usize = 0;
const BLACK_KING_RANK: usize = 7;

pub struct GameBoard {
    squares: Vec<Vec<Square>>,
}

impl GameBoard {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut squares = Self::get_empty_squares()?;

        squares[0][0].piece = Some(Piece::from(PieceColor::White, PieceType::Rook));
        squares[0][1].piece = Some(Piece::from(PieceColor::White, PieceType::Knight));
        squares[0][2].piece = Some(Piece::from(PieceColor::White, PieceType::Bishop));
        squares[0][3].piece = Some(Piece::from(PieceColor::White, PieceType::Queen));
        squares[0][4].piece = Some(Piece::from(PieceColor::White, PieceType::King));
        squares[0][5].piece = Some(Piece::from(PieceColor::White, PieceType::Bishop));
        squares[0][6].piece = Some(Piece::from(PieceColor::White, PieceType::Knight));
        squares[0][7].piece = Some(Piece::from(PieceColor::White, PieceType::Rook));
        squares[1][0].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][1].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][2].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][3].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][4].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][5].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][6].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));
        squares[1][7].piece = Some(Piece::from(PieceColor::White, PieceType::Pawn));

        squares[6][0].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][1].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][2].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][3].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][4].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][5].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][6].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[6][7].piece = Some(Piece::from(PieceColor::Black, PieceType::Pawn));
        squares[7][0].piece = Some(Piece::from(PieceColor::Black, PieceType::Rook));
        squares[7][1].piece = Some(Piece::from(PieceColor::Black, PieceType::Knight));
        squares[7][2].piece = Some(Piece::from(PieceColor::Black, PieceType::Bishop));
        squares[7][3].piece = Some(Piece::from(PieceColor::Black, PieceType::Queen));
        squares[7][4].piece = Some(Piece::from(PieceColor::Black, PieceType::King));
        squares[7][5].piece = Some(Piece::from(PieceColor::Black, PieceType::Bishop));
        squares[7][6].piece = Some(Piece::from(PieceColor::Black, PieceType::Knight));
        squares[7][7].piece = Some(Piece::from(PieceColor::Black, PieceType::Rook));

        Ok(Self { squares })
    }

    pub fn from(square_info_list: &Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut squares = Self::get_empty_squares()?;

        // square_info = "piece <color><type> square-<file><rank>"
        for square_info in square_info_list {
            let split_square_info: Vec<&str> = square_info.split_whitespace().collect();

            let mut piece_chars = split_square_info
                .iter()
                .find(|item| item.len() == 2) // <color><type> in square_info
                .context(format!(
                    "Unable to find piece type / piece color info from piece info: {square_info}"
                ))?
                .chars();

            let color_char = piece_chars
                .next()
                .context("Did not find a char for piece color")?;
            let color = Piece::chess_dot_com_char_to_piece_color(color_char)?;

            let piece_type_char = piece_chars
                .next()
                .context("Did not find a char for piece type")?;
            let piece_type = Piece::chess_dot_com_char_to_piece_type(piece_type_char)?;

            let mut coordinate_chars = split_square_info
                .iter()
                .find(|item| item.starts_with("square-"))
                .context(format!(
                    "Unable to find piece coordinates info from piece info: {square_info}"
                ))?
                .trim_start_matches("square-")
                .chars();

            let file_index_char = coordinate_chars
                .next()
                .context("Did not find char for file")?;
            let file_index = Square::chess_dot_com_index_char_to_index(file_index_char)?;

            let rank_index_char = coordinate_chars
                .next()
                .context("Did not find char for rank")?;
            let rank_index = Square::chess_dot_com_index_char_to_index(rank_index_char)?;

            squares[rank_index][file_index].piece = Some(Piece::from(color, piece_type));
        }

        Ok(Self { squares })
    }

    pub fn set_state(&mut self, new_board: &Self) {
        for rank in 0..BOARD_SIZE {
            for file in 0..BOARD_SIZE {
                self.squares[rank][file].piece = new_board.squares[rank][file].piece.clone();
            }
        }
    }

    pub fn get_move_from_difference(
        &self,
        new_board: &GameBoard,
    ) -> Result<Option<ChessMove>, Box<dyn Error>> {
        let mut changed_positions: Vec<(usize, usize)> = Vec::new();
        let mut changed_squares: Vec<String> = Vec::new();

        for rank in 0..BOARD_SIZE {
            for file in 0..BOARD_SIZE {
                if self.squares[rank][file] != new_board.squares[rank][file] {
                    changed_positions.push((rank, file));
                    changed_squares.push(self.squares[rank][file].uci_notation());
                }
            }
        }

        println!("CHANGED RANK / FILE : {:?}", changed_positions);
        println!("CHANGED SQUARES: {:?}", changed_squares);

        match changed_positions.len() {
            0 => Ok(None),
            2 => self
                .get_standard_move(&changed_positions, new_board)
                .map(|chess_move| Some(chess_move)),
            3 => self
                .get_en_passant_move(&changed_positions, new_board)
                .map(|chess_move| Some(chess_move)),
            4 => self
                .get_castle_move(&changed_positions, new_board)
                .map(|chess_move| Some(chess_move)),
            _ => Err(Box::from(format!(
                "Error: {} changes made to chess board 😰. Expected 0 or 2 - 4",
                changed_positions.len()
            ))),
        }
    }

    fn get_standard_move(
        &self,
        changed_positions: &Vec<(usize, usize)>,
        new_board: &GameBoard,
    ) -> Result<ChessMove, Box<dyn Error>> {
        if changed_positions.len() != 2 {
            return Err(Box::from(format!(
                "Expected exactly 2 changed positions for standard move. Found: {:?}",
                changed_positions
            )));
        }

        let (rank1, file1) = changed_positions[0];
        let (rank2, file2) = changed_positions[1];

        // For any standard move, empty square = starting square, square with piece = ending square
        let (start_rank, start_file, end_rank, end_file) =
            if new_board.squares[rank1][file1].piece.is_none() {
                (rank1, file1, rank2, file2)
            } else {
                (rank2, file2, rank1, file1)
            };

        let start_square = self.squares[start_rank][start_file].clone();
        let end_square = new_board.squares[end_rank][end_file].clone();

        Ok(ChessMove::from(start_square, end_square))
    }

    fn get_en_passant_move(
        &self,
        changed_positions: &Vec<(usize, usize)>,
        new_board: &GameBoard,
    ) -> Result<ChessMove, Box<dyn Error>> {
        if changed_positions.len() != 3 {
            return Err(Box::from(format!(
                "Expected exactly 3 changed positions for en passant move. Found: {:?}",
                changed_positions
            )));
        }

        // Pawn ending square is the only square with a piece on it
        let (end_rank, end_file) = changed_positions
            .iter()
            .find(|(rank, file)| new_board.squares[*rank][*file].piece.is_some())
            .context(format!(
                "Could not find pawn ending position for en passant move with {:?}",
                changed_positions
            ))?
            .clone();

        // Pawn starting square is the only square on a different file from end position
        let (start_rank, start_file) = changed_positions
            .iter()
            .find(|(_rank, file)| *file != end_file)
            .context(format!(
                "Could not find pawn starting position for en passant move with {:?}",
                changed_positions
            ))?
            .clone();

        let start_square = self.squares[start_rank][start_file].clone();
        let end_square = new_board.squares[end_rank][end_file].clone();

        Ok(ChessMove::from(start_square, end_square))
    }

    fn get_castle_move(
        &self,
        changed_positions: &Vec<(usize, usize)>,
        new_board: &GameBoard,
    ) -> Result<ChessMove, Box<dyn Error>> {
        if changed_positions.len() != 4 {
            return Err(Box::from(format!(
                "Expected exactly 4 changed positions for castle move. Found: {:?}",
                changed_positions
            )));
        }

        // Castle always starts on the king file, just need to find which rank
        let start_rank = if changed_positions.contains(&(WHITE_KING_RANK, KING_FILE)) {
            WHITE_KING_RANK
        } else {
            BLACK_KING_RANK
        };

        // If king side castle, end file > king file, else is a queen side castle with end file < king file
        let end_file = if changed_positions.contains(&(start_rank, KING_FILE + 2)) {
            KING_FILE + 2
        } else {
            KING_FILE - 2
        };

        let start_square = self.squares[start_rank][KING_FILE].clone();
        let end_square = new_board.squares[start_rank][end_file].clone();

        Ok(ChessMove::from(start_square, end_square))
    }

    fn get_empty_squares() -> Result<Vec<Vec<Square>>, Box<dyn Error>> {
        let mut squares: Vec<Vec<Square>> = Vec::with_capacity(BOARD_SIZE);

        for rank_index in 0..BOARD_SIZE {
            squares.push(Vec::with_capacity(BOARD_SIZE));
            for file_index in 0..BOARD_SIZE {
                let rank = Square::index_to_rank(rank_index)?;
                let file = Square::index_to_file(file_index)?;
                squares[rank_index].push(Square::from(rank, file, None));
            }
        }

        Ok(squares)
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for rank_index in 0..BOARD_SIZE {
            for file_index in 0..BOARD_SIZE {
                let square = &self.squares[rank_index][file_index];
                output += match &square.piece {
                    None => String::from("   "),
                    Some(piece) => format!("{}{} ", piece.color, piece.piece_type),
                }
                .as_str();
            }
            output += "\n";
        }

        write!(f, "{}", output)
    }
}
