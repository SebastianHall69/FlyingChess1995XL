use crate::chess_game::standard_chess_game::Color;
use std::error::Error;
use std::process;
use std::time::Duration;
use thirtyfour::{By, DesiredCapabilities, WebDriver};

pub struct ChessDotComInterface {
    chromedriver: process::Child,
    selenium: WebDriver,
}

impl ChessDotComInterface {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            chromedriver: spawn_chromedriver_process().await?,
            selenium: get_selenium_driver().await?,
        })
    }

    pub async fn login(&self) -> Result<bool, Box<dyn Error>> {
        self.selenium
            .goto("https://www.chess.com/login_and_go?returnUrl=https://www.chess.com/")
            .await?;

        let email_input = self.selenium.find(By::Css("input[type='email']")).await?;
        let password_input = self
            .selenium
            .find(By::Css("input[type='password']"))
            .await?;
        let enter_button = self.selenium.find(By::Css("button[type='submit']")).await?;

        email_input
            .send_keys("enpassantsworstnightmare@gmail.com")
            .await?;
        password_input.send_keys("bovh34ZYQpqBn6").await?;
        enter_button.click().await?;

        Ok(true)
    }

    pub async fn requeue(&self) -> Result<bool, Box<dyn Error>> {
        let requeue_button_div = self
            .selenium
            .find(By::Css("div.game-over-buttons-component"))
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

    pub async fn is_match_in_progress(&self) -> bool {
        // Presence of resign button indicates match is in progress
        self.selenium
            .find(By::Css("button.resign-button-component"))
            .await
            .map_or(false, |_| true)
    }

    pub async fn get_piece_color(&self) -> Result<Color, Box<dyn Error>> {
        // Board class has 'flipped' when playing as black
        self.selenium
            .find(By::Css("wc-chess-board#board-single"))
            .await?
            .class_name()
            .await?
            .map(|class| match class.contains("flipped") {
                true => Color::Black,
                false => Color::White,
            })
            .ok_or_else(|| "Could not find color".into())
    }

    pub async fn is_my_turn(&self) -> Result<bool, Box<dyn Error>> {
        // Bottom clock class has 'clock-player-turn' when it is my turn
        self.selenium
            .find(By::Css("div.clock-bottom"))
            .await?
            .class_name()
            .await?
            .map(|class| class.contains("clock-player-turn"))
            .ok_or_else(|| "Could not identify turn".into())
    }
}

impl Drop for ChessDotComInterface {
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
