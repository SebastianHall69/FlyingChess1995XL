use crate::chess_game::standard_chess_game::StandardChessGame;
use crate::web_interface::chess_dot_com_interface::ChessDotComInterface;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
pub enum BotState {
    EntryPoint,
    Login,
    WaitingForMatch,
    PlayMatch,
    Requeue,
    Error,
}

pub struct ChessBot {
    should_run_state_machine_flag: Arc<Mutex<bool>>,
    web_interface: ChessDotComInterface,
}

impl ChessBot {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let chess_bot = Self {
            should_run_state_machine_flag: Arc::new(Mutex::new(true)),
            web_interface: ChessDotComInterface::new().await?,
        };
        connect_exit_signal_to_flag(chess_bot.should_run_state_machine_flag.clone());
        Ok(chess_bot)
    }

    pub async fn run_bot_state_machine(&self) -> Result<(), Box<dyn Error>> {
        let mut state = BotState::EntryPoint;

        while self.should_run_bot_state_machine() {
            println!("Current state: {:?}", state);

            match state {
                BotState::EntryPoint => state = BotState::Login,
                BotState::Login => {
                    let is_login_successful = self.web_interface.login().await?;
                    match is_login_successful {
                        true => state = BotState::WaitingForMatch,
                        false => state = BotState::Error,
                    }
                }
                BotState::WaitingForMatch => {
                    let is_match_found = self.web_interface.is_match_in_progress().await;
                    match is_match_found {
                        true => state = BotState::PlayMatch,
                        false => state = BotState::WaitingForMatch,
                    }
                }
                BotState::PlayMatch => {
                    self.play_match().await?;
                    state = BotState::Requeue;
                }
                BotState::Requeue => {
                    let is_requeue_successful = self.requeue().await?;
                    match is_requeue_successful {
                        true => state = BotState::WaitingForMatch,
                        false => state = BotState::Requeue,
                    }
                }
                BotState::Error => println!("I really don't know what I'm doing now..."),
            }

            println!();
            tokio::time::sleep(Duration::from_millis(1500)).await;
        }

        Ok(())
    }

    fn should_run_bot_state_machine(&self) -> bool {
        *self.should_run_state_machine_flag.lock().unwrap() == true
    }

    async fn play_match(&self) -> Result<(), Box<dyn Error>> {
        StandardChessGame::new(self.should_run_state_machine_flag.clone())?
            .run_match_state_machine(&self.web_interface)
            .await
    }

    async fn requeue(&self) -> Result<bool, Box<dyn Error>> {
        tokio::time::sleep(Duration::from_secs(10)).await;
        self.web_interface.requeue().await
    }
}

fn connect_exit_signal_to_flag(should_run: Arc<Mutex<bool>>) {
    ctrlc::set_handler(move || {
        *should_run.lock().unwrap() = false;
    })
    .expect("Failed to set SIGINT exit handler");
}
