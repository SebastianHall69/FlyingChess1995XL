use crate::chess_game::square::Square;

pub struct Board {
    position: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            position: Vec::new(),
        }
    }

    pub fn update_position(&mut self, new_position: Vec<Vec<Square>>) {
        self.position = new_position;
    }

    pub fn calculate_move_made_in_long_algebraic_notation(
        &self,
        new_position: &Vec<Vec<Square>>,
    ) -> Option<String> {
        let mut changes: Vec<(usize, usize)> = Vec::new();

        for rank in 0..self.position.len() {
            for file in 0..self.position[rank].len() {
                if self.position[rank][file] != new_position[rank][file] {
                    changes.push((rank, file));
                }
            }
        }

        self.get_move_notation(&changes, new_position)
    }

    fn get_move_notation(
        &self,
        changes: &Vec<(usize, usize)>,
        new_position: &Vec<Vec<Square>>,
    ) -> Option<String> {
        match changes.len() {
            2 => Some(self.get_standard_move_notation(&changes, new_position)),
            3 => Some(self.get_en_passant_notation(&changes, new_position)),
            4 => Some(self.get_castle_notation(&changes, new_position)),
            _ => None,
        }
    }

    fn get_standard_move_notation(
        &self,
        changes: &Vec<(usize, usize)>,
        new_position: &Vec<Vec<Square>>,
    ) -> String {
        let (rank1, file1) = changes[0];
        let (rank2, file2) = changes[1];

        // In new position, empty square = starting square, piece = ending square
        let (start_rank, start_file, end_rank, end_file) =
            if new_position[rank1][file1].piece().is_none() {
                (rank1, file1, rank2, file2)
            } else {
                (rank2, file2, rank1, file1)
            };

        let start_square = &self.position[start_rank][start_file];
        let end_square = &new_position[end_rank][end_file];
        let promotion_type = get_promotion_type(start_square, end_square);

        build_notation(start_square, end_square, promotion_type)
    }

    fn get_en_passant_notation(
        &self,
        changes: &Vec<(usize, usize)>,
        new_position: &Vec<Vec<Square>>,
    ) -> String {
        // Pawn ending square is the only one with a piece still on it
        let (end_rank, end_file) = changes
            .iter()
            .find(|(rank, file)| new_position[*rank][*file].piece().is_some())
            .unwrap()
            .clone();

        // Pawn starting square will be the only one on a different file from the end square
        let (start_rank, start_file) = changes
            .iter()
            .find(|(_rank, file)| *file != end_file)
            .unwrap()
            .clone();

        let start_square = &self.position[start_rank][start_file];
        let end_square = &new_position[end_rank][end_file];
        let promotion_type = get_promotion_type(start_square, end_square);

        build_notation(start_square, end_square, promotion_type)
    }

    fn get_castle_notation(
        &self,
        changes: &Vec<(usize, usize)>,
        new_position: &Vec<Vec<Square>>,
    ) -> String {
        let king_file: usize = 4;
        let white_rank: usize = 0;
        let black_rank: usize = 7;

        let start_rank = if changes.contains(&(white_rank, king_file)) {
            white_rank
        } else {
            black_rank
        };

        // file > king in changes = king side castle. file < king in changes = queen side castle
        let end_file = if changes.contains(&(start_rank, king_file + 2)) {
            king_file + 2
        } else {
            king_file - 2
        };

        let start_square = &self.position[start_rank][king_file];
        let end_square = &new_position[start_rank][end_file];

        build_notation(start_square, end_square, None)
    }
}

fn get_promotion_type(start_square: &Square, end_square: &Square) -> Option<String> {
    match start_square.piece_type() == end_square.piece_type() {
        true => None,
        false => Some(end_square.piece_type()),
    }
}

fn build_notation(
    start_square: &Square,
    end_square: &Square,
    promotion_type: Option<String>,
) -> String {
    let promotion_type = promotion_type.unwrap_or(String::new());
    format!(
        "{}{}{}",
        start_square.algebraic_notation(),
        end_square.algebraic_notation(),
        promotion_type
    )
}
