#[derive(PartialEq)]
pub struct Square {
    rank_index: usize,
    file_index: usize,
    piece: Option<String>,
}

impl Square {
    pub fn new(rank_index: usize, file_index: usize, piece: Option<String>) -> Self {
        Self {
            rank_index,
            file_index,
            piece,
        }
    }

    pub fn set_piece(&mut self, piece: Option<String>) {
        self.piece = piece;
    }

    pub fn piece(&self) -> &Option<String> {
        &self.piece
    }

    pub fn piece_type(&self) -> String {
        self.piece
            .clone()
            .unwrap()
            .trim_start_matches("w")
            .trim_start_matches("b")
            .to_string()
    }

    pub fn algebraic_notation(&self) -> String {
        let rank = self.get_rank_from_index();
        let file = self.get_file_from_index();
        format!("{}{}", file, rank)
    }

    fn get_file_from_index(&self) -> String {
        match self.file_index {
            0 => "a".to_string(),
            1 => "b".to_string(),
            2 => "c".to_string(),
            3 => "d".to_string(),
            4 => "e".to_string(),
            5 => "f".to_string(),
            6 => "g".to_string(),
            7 => "h".to_string(),
            num => format!("Invalid file index: {num}"),
        }
    }

    fn get_rank_from_index(&self) -> String {
        match self.rank_index {
            0..8 => (self.rank_index + 1).to_string(),
            num => format!("Invalid rank index: {num}"),
        }
    }
}
