use crate::engine::stockfish::Stockfish;
use crate::web_interface::chess_dot_com_interface::ChessDotComInterface;
use std::collections::HashMap;
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
    GetMove,
    PlayMove,
    Error,
}

pub struct StandardChessGame {
    color: Color,
    _game_state: HashMap<String, String>,
    _engine: Stockfish,
    should_run_state_machine_flag: Arc<Mutex<bool>>,
}

impl StandardChessGame {
    pub fn new(should_run_state_machine_flag: Arc<Mutex<bool>>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            color: Color::White,
            _game_state: HashMap::new(),
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
                    self.color = web_interface.get_piece_color().await?;
                    println!("I am playing as {:?}", self.color);
                    // Get starting positions
                    // Reset engine
                    state = GameState::WaitForTurn;
                    println!("Setup not implemented yet");
                }
                GameState::WaitForTurn => {
                    state = match web_interface.is_my_turn().await? {
                        true => GameState::ReadVictimMove,
                        false => GameState::WaitForTurn,
                    };
                }
                GameState::ReadVictimMove => {
                    // Get piece position updates. Either real move or startpos if no moves yet
                    println!("ReadVictimMove not implemented yet");
                }
                GameState::GetMove => {
                    // Get stockfish move
                    println!("GetMove not implemented yet");
                }
                GameState::PlayMove => {
                    // Play move in browser
                    // Report move to stockfish
                    // Update positions map
                    println!("PlayMove not implemented yet");
                }
                GameState::Error => println!("I really really don't know how I ended up here"),
            }

            println!();
            tokio::time::sleep(Duration::from_millis(1500)).await; // TODO: DON'T FORGET TO REMOVE THIS DELAY
        }

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
