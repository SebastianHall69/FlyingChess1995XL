use std::collections::HashMap;
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use crate::chess_game::standard_chess_game::StandardChessGame;

#[derive(Debug)]
pub enum BotState {
    EntryPoint,
    Login,
    SearchingForMatch,
    PlayMatch,
    Requeue,
    Error
}

pub struct ChessBot {
    chromedriver: process::Child,
    selenium: WebDriver,
    piece_positions: HashMap<String, String>,
    should_run_state_machine_flag: Arc<Mutex<bool>>
}

impl ChessBot {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let chess_bot = Self {
            chromedriver: spawn_chromedriver_process().await?,
            selenium: get_selenium_driver().await?,
            piece_positions: HashMap::new(),
            should_run_state_machine_flag: Arc::new(Mutex::new(true))
        };
        connect_exit_signal_to_flag(&chess_bot.should_run_state_machine_flag);
        Ok(chess_bot)
    }

    pub async fn run_bot_state_machine(&mut self) -> Result<(), Box<dyn Error>> {
        let mut state = BotState::EntryPoint;

        while self.should_run_bot_state_machine() {
            println!("Current state: {:?}", state);

            match state {
                BotState::EntryPoint => state = BotState::Login,
                BotState::Login => {
                    let is_login_successful = self.login().await?;
                    match is_login_successful {
                        true => state = BotState::SearchingForMatch,
                        false => state = BotState::Error
                    }
                },
                BotState::SearchingForMatch => {
                    let is_match_found = self.is_match_in_progress().await?;
                    match is_match_found {
                        true => state = BotState::PlayMatch,
                        false => state = BotState::SearchingForMatch
                    }
                },
                BotState::PlayMatch => {
                    self.play_match().await?;
                    state = BotState::Requeue;
                },
                BotState::Requeue => {
                    let is_requeue_successful = self.requeue().await?;
                    match is_requeue_successful {
                        true => state = BotState::SearchingForMatch,
                        false => state = BotState::Requeue
                    }
                },
                BotState::Error => println!("I really don't know what I'm doing now...")
            }

            println!();
            tokio::time::sleep(Duration::from_millis(1500)).await;
        }

        Ok(())
    }

    async fn login(&self) -> Result<bool, Box<dyn Error>> {
        self.selenium
            .goto("https://www.chess.com/login_and_go?returnUrl=https://www.chess.com/")
            .await?;

        let email_input = self.selenium
            .find(By::Css("input[type='email']"))
            .await?;
        let password_input = self.selenium
            .find(By::Css("input[type='password']"))
            .await?;
        let enter_button = self.selenium
            .find(By::Css("button[type='submit']"))
            .await?;

        email_input.send_keys("enpassantsworstnightmare@gmail.com").await?;
        password_input.send_keys("bovh34ZYQpqBn6").await?;
        enter_button.click().await?;

        Ok(true)
    }

    async fn requeue(&self) -> Result<bool, Box<dyn Error>> {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let requeue_button_div = self.selenium
            .find(By::Css("div[class='game-over-buttons-component']"))
            .await?;

        let new_game_button = requeue_button_div
            .find(By::Css("button:not([aria-label])"))
            .await;
        let _offer_rematch_button = requeue_button_div
            .find(By::Css("button[aria-label='Rematch']"))
            .await;
        let _accept_rematch_button = requeue_button_div
            .find(By::Css("button[aria-label='Accept Rematch']"))
            .await;
        let decline_rematch_button = requeue_button_div
            .find(By::Css("button[aria-label='Decline Rematch']"))
            .await;

        // TODO: GET SOME REAL LOGIC IN HERE
        if decline_rematch_button.is_ok() {
            println!("Declining that shit!");
            decline_rematch_button?.click().await?;
        } else if new_game_button.is_ok() {
            println!("Starting a new game");
            new_game_button?.click().await?;
            return Ok(true);
        }

        Ok(false)
    }

    fn should_run_bot_state_machine(&self) -> bool {
        *self.should_run_state_machine_flag.lock().unwrap() == true
    }

    async fn is_match_in_progress(&self) -> Result<bool, Box<dyn Error>> {
        // Presence of resign button indicates match is in progress
        let resign_button_result = self.selenium
            .find(By::Css("button[class='resign-button-component']"))
            .await;

        match resign_button_result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    async fn play_match(&self) -> Result<(), Box<dyn Error>> {
        StandardChessGame::new(self.should_run_state_machine_flag.clone())?
            .run_match_state_machine(&self.selenium) // TODO: CHANGE THIS TO WEB DRIVER CLASS
            .await
    }
}

impl Drop for ChessBot {
    fn drop(&mut self) {
        self.chromedriver
            .kill()
            .expect("Failed to kill chromedriver process");
    }
}

async fn spawn_chromedriver_process() -> Result<process::Child, Box<dyn Error>> {
    let chromedriver = process::Command::new("./bin/chromedriver")
        .arg("--port=9515")
        .spawn()
        .map_err(|err| Box::new(err) as Box<dyn Error>)?;
    verify_chromedriver_is_running().await?;
    Ok(chromedriver)
}

async fn verify_chromedriver_is_running() -> Result<(), Box<dyn Error>> {
    let chromedriver_status_page = "http://localhost:9515/status";

    for _ in 0..10 {
        if reqwest::get(chromedriver_status_page).await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "chromedriver failed to start",
    )))
}

async fn get_selenium_driver() -> Result<WebDriver, Box<dyn Error>> {
    let capabilities = DesiredCapabilities::chrome();
    Ok(WebDriver::new("http://localhost:9515", capabilities).await?)
}

fn connect_exit_signal_to_flag(should_run: &Arc<Mutex<bool>>) {
    let should_run_clone = should_run.clone();
    ctrlc::set_handler(move || {
        *should_run_clone.lock().unwrap() = false;
    }).expect("Failed to set SIGINT exit handler");
}
