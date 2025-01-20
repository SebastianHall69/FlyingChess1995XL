use std::collections::HashMap;
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thirtyfour::WebDriver;

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
    game_state: HashMap<String, String>,
    stockfish: process::Child,
    should_run_state_machine_flag: Arc<Mutex<bool>>,
}

impl StandardChessGame {
    pub fn new(should_run_state_machine_flag: Arc<Mutex<bool>>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            color: Color::White,
            game_state: HashMap::new(),
            stockfish: spawn_stockfish_process()?,
            should_run_state_machine_flag,
        })
    }

    pub async fn run_match_state_machine(&self, selenium: &WebDriver) -> Result<(), Box<dyn Error>> {
        let mut state = GameState::EntryPoint;

        while self.should_run_match_state_machine().await? {
            // TODO: FIX
            println!("Current state: {:?}", state);

            match state {
                GameState::EntryPoint => state = GameState::Setup,
                GameState::Setup => {
                    // Get color
                    // Get starting positions
                    // Reset engine
                    println!("Setup not implemented yet");
                }
                GameState::WaitForTurn => {
                    // Wait for clock
                    println!("WaitForTurn not implemented yet");
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

            tokio::time::sleep(Duration::from_millis(300)).await;
        }

        Ok(())
    }

    async fn should_run_match_state_machine(&self) -> Result<bool, Box<dyn Error>> {
        let should_run_match = *self.should_run_state_machine_flag.lock().unwrap() == true
            && is_match_in_progress().await?; // TODO: IMPLEMENT THIS
        Ok(should_run_match)
    }
}

impl Drop for StandardChessGame {
    fn drop(&mut self) {
        self.stockfish
            .kill()
            .expect("Failed to kill stockfish process");
    }
}

pub fn spawn_stockfish_process() -> Result<process::Child, Box<dyn Error>> {
    let mut stockfish = process::Command::new("./bin/stockfish_engine")
        .spawn()
        .map_err(|err| Box::new(err) as Box<dyn Error>)?;
    verify_stockfish_is_running(&mut stockfish)?;
    Ok(stockfish)
}

fn verify_stockfish_is_running(process: &mut process::Child) -> Result<(), Box<dyn Error>> {
    match process.try_wait() {
        Ok(Some(_exit_status)) => Err(Box::from("Stockfish process not running")),
        Ok(None) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}
