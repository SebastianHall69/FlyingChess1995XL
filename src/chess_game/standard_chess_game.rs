use crate::chess_game::board::Board;
use crate::engine::stockfish::Stockfish;
use crate::web_interface::chess_dot_com_interface::ChessDotComInterface;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum GameState {
    EntryPoint,
    Setup,
    WaitForTurn,
    ReadVictimMove,
    PlayMove,
    Error,
}

pub struct StandardChessGame {
    color: Color,
    board: Board,
    _engine: Stockfish,
    should_run_state_machine_flag: Arc<Mutex<bool>>,
}

impl StandardChessGame {
    pub fn new(should_run_state_machine_flag: Arc<Mutex<bool>>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            color: Color::White,
            board: Board::new(),
            _engine: Stockfish::new()?,
            should_run_state_machine_flag,
        })
    }

    pub async fn run_match_state_machine(
        &mut self,
        web_interface: &ChessDotComInterface,
    ) -> Result<(), Box<dyn Error>> {
        let mut state = GameState::EntryPoint;

        while self.should_run_match_state_machine(web_interface).await? {
            println!("Current state: {:?}", state);

            match state {
                GameState::EntryPoint => state = GameState::Setup,
                GameState::Setup => {
                    self.setup(&web_interface).await?;
                    state = match self.color {
                        Color::White => GameState::PlayMove,
                        Color::Black => GameState::WaitForTurn,
                    }
                }
                GameState::WaitForTurn => {
                    state = match web_interface.is_my_turn().await? {
                        true => GameState::ReadVictimMove,
                        false => GameState::WaitForTurn,
                    };
                }
                GameState::ReadVictimMove => {
                    self.read_victim_move(&web_interface).await?;
                    state = GameState::PlayMove;
                }
                GameState::PlayMove => {
                    self.play_move(&web_interface).await?;
                    state = GameState::WaitForTurn;
                }
                GameState::Error => println!("I really really don't know how I ended up here"),
            }

            println!();
            tokio::time::sleep(Duration::from_millis(1500)).await; // TODO: DON'T FORGET TO REMOVE THIS DELAY
        }

        Ok(())
    }

    async fn setup(&mut self, web_interface: &ChessDotComInterface) -> Result<(), Box<dyn Error>> {
        self.color = web_interface.get_piece_color().await?;
        let start_position = web_interface.get_position().await?;
        self.board.update_position(start_position);
        Ok(())
    }

    async fn read_victim_move(
        &mut self,
        web_interface: &ChessDotComInterface,
    ) -> Result<(), Box<dyn Error>> {
        let new_position = web_interface.get_position().await?;
        let _victim_move = self
            .board
            .calculate_move_made_in_long_algebraic_notation(&new_position);

        println!("VICTIM MADE MOVE: {:?}", _victim_move);

        self.board.update_position(new_position);
        // TODO: TELL STOCKFISH ABOUT MOVE
        Ok(())
    }

    async fn play_move(
        &mut self,
        web_interface: &ChessDotComInterface,
    ) -> Result<(), Box<dyn Error>> {
        // Get move from stockfish
        // Play move in browser
        // Report move to stockfish
        // TODO: REMOVE AFTER MOVING PIECE
        println!("PlayMove not yet implemented");
        while web_interface.is_my_turn().await? {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        let new_position = web_interface.get_position().await?;
        self.board.update_position(new_position);
        // TODO: REPORT MOVE TO THE FISH
        Ok(())
    }

    async fn should_run_match_state_machine(
        &self,
        web_interface: &ChessDotComInterface,
    ) -> Result<bool, Box<dyn Error>> {
        let should_run_match = *self.should_run_state_machine_flag.lock().unwrap() == true
            && web_interface.is_match_in_progress().await;
        Ok(should_run_match)
    }
}
