use crate::chess_move::ChessMove;
use crate::game_board::GameBoard;
use crate::piece::PieceColor;
use anyhow::Context;
use std::error::Error;
use thirtyfour::action_chain::ActionChain;
use thirtyfour::prelude::ElementQueryable;
use thirtyfour::{
    By, CapabilitiesHelper, ChromiumLikeCapabilities, DesiredCapabilities, PageLoadStrategy,
    WebDriver,
};
use tokio::time::{Duration, sleep};

pub struct ChessDotComInterface {
    chromedriver: std::process::Child,
    pub web_driver: WebDriver,
}

impl ChessDotComInterface {
    pub async fn new() -> Result<ChessDotComInterface, Box<dyn Error>> {
        let chromedriver = std::process::Command::new("./bin/chromedriver")
            .arg("--port=9515")
            .spawn()
            .context("Failed to spawn chromedriver")?;
        Self::wait_for_chromedriver_to_be_ready().await?;

        let mut capabilities = DesiredCapabilities::chrome();
        capabilities
            .set_page_load_strategy(PageLoadStrategy::Eager)
            .context("Failed to set eager page load strategy")?;
        capabilities
            .add_arg("--disable-dev-shm-usage")
            .context("Failed to set --disable-dev-shm-usage")?;
        capabilities
            .add_arg("--no-sandbox")
            .context("Failed to set --no-sandbox")?;
        capabilities
            .add_arg("--disable-extensions")
            .context("Failed to set --disable-extensions")?;
        capabilities
            .add_arg("--disable-plugins")
            .context("Failed to set --disable-plugins")?;
        capabilities
            .add_arg("--disable-web-security")
            .context("Failed to set --disable-web-security")?;
        capabilities
            .add_arg("--disable-features=TranslateUI")
            .context("Failed to set --disable-features=TranslateUI")?;
        capabilities
            .add_arg("--disable-ipc-flooding-protection")
            .context("Failed to set --disable-ipc-flooding-protection")?;
        capabilities
            .add_arg("--disable-background-timer-throttling")
            .context("Failed to set --disable-background-timer-throttling")?;
        capabilities
            .add_arg("--disable-backgrounding-occluded-windows")
            .context("Failed to set --disable-backgrounding-occluded-windows")?;
        capabilities
            .add_arg("--memory-pressure-off")
            .context("Failed to set --memory-pressure-off")?;
        capabilities
            .add_arg("--max_old_space_size=4096")
            .context("Failed to set --max_old_space_size=4096")?;
        capabilities
            .add_arg("--aggressive-cache-discard")
            .context("Failed to set --aggressive-cache-discard")?;
        capabilities
            .add_arg("--disable-renderer-backgrounding")
            .context("Failed to set --disable-renderer-backgrounding")?;

        let web_driver = WebDriver::new("http://localhost:9515", capabilities)
            .await
            .context("Failed to initialize web driver")?;

        Ok(ChessDotComInterface {
            chromedriver,
            web_driver,
        })
    }

    pub async fn login(&self) -> Result<(), Box<dyn Error>> {
        self.web_driver
            .goto("https://www.chess.com/login_and_go?returnUrl=https://www.chess.com/play/online")
            .await
            .context("Failed to navigate to login page")?;

        sleep(Duration::from_millis(500)).await;

        self.web_driver
            .query(By::Id("login-username"))
            .and_clickable()
            .single()
            .await
            .context("Failed to find login username input")?
            .send_keys("virtual.baller@proton.me")
            .await
            .context("Failed to write username into input")?;

        self.web_driver
            .query(By::Id("login-password"))
            .and_clickable()
            .single()
            .await
            .context("Failed to find login password input")?
            .send_keys("eWQRV@J$fsjQKxWvt7&oSg")
            .await
            .context("Failed to write password into input")?;

        self.web_driver
            .query(By::Id("login"))
            .and_clickable()
            .single()
            .await
            .context("Failed to find login submit button")?
            .click()
            .await
            .context("Failed to click login submit button")?;

        Ok(())
    }

    pub async fn is_match_available(&self) -> bool {
        // The draw button is only present when a game is in progress
        // #board-layout-sidebar --> nearest parent with an id to limit search
        // .draw-button-label --> div for draw button
        let result = self
            .web_driver
            .query(By::Css("#board-layout-sidebar .draw-button-label"))
            .nowait()
            .wait(Duration::from_millis(500), Duration::from_millis(100))
            .first()
            .await;

        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn get_player_color(&self) -> Result<PieceColor, Box<dyn Error>> {
        // #board-layout-player-bottom --> nearest parent for quick search
        // .clock-bottom --> Players clock
        // .clock-black / .clock-white --> Indicates player color
        self.web_driver
            .find(By::Css("#board-layout-player-bottom .clock-bottom"))
            .await
            .context("Failed to find player clock element for player color")?
            .class_name()
            .await
            .context("Failed to get class name for player color")?
            .map(|class_string| match class_string.contains("clock-black") {
                true => PieceColor::Black,
                false => PieceColor::White,
            })
            .ok_or_else(|| Box::from("Class string not found for player color"))
    }

    pub async fn is_my_turn(&self) -> Result<bool, Box<dyn Error>> {
        // #board-layout-player-bottom --> nearest parent for quick search
        // .clock-bottom --> Players clock
        // .clock-player-turn --> Indicates it's the current players turn
        self.web_driver
            .find(By::Css("#board-layout-player-bottom .clock-bottom"))
            .await
            .context("Failed to find player clock element for player turn")?
            .class_name()
            .await
            .context("Failed to get class name for players turn")?
            .map_or(Ok(false), |class_string| {
                Ok(class_string.contains("clock-player-turn"))
            })
    }

    pub async fn get_current_board_state(&self) -> Result<GameBoard, Box<dyn Error>> {
        // #board-single --> top level board element
        let get_piece_info_script = r#"
            const board = document.getElementById('board-single');
            if (!board) return [];
            const piece_elements = board.querySelectorAll('.piece');
            return Array.from(piece_elements).map(element => element.className);
        "#;

        let piece_info_list = self
            .web_driver
            .execute(get_piece_info_script, Vec::new())
            .await
            .context("Failed executing piece positions script")?
            .convert::<Vec<String>>()
            .context("Failed to parse class name strings for piece positions")?;

        GameBoard::from(&piece_info_list)
    }

    pub async fn play_move(
        &self,
        chess_move: &ChessMove,
        is_board_flipped: bool,
    ) -> Result<(), Box<dyn Error>> {
        // #board-single --> top level board element
        let board = self
            .web_driver
            .query(By::Id("board-single"))
            .nowait()
            .wait(Duration::from_millis(500), Duration::from_millis(100))
            .first()
            .await
            .context("Failed to find board for playing move")?;

        let board_width = board
            .rect()
            .await
            .context("Failed to find rectangle for board")?
            .width;

        let (start_rank_offset, start_file_offset) = chess_move
            .start
            .get_offset_from_board_center(board_width, is_board_flipped)?;

        let (end_rank_offset, end_file_offset) = chess_move
            .end
            .get_offset_from_board_center(board_width, is_board_flipped)?;

        let mouse_delay = Duration::from_millis(220);
        ActionChain::new_with_delay(self.web_driver.handle.clone(), None, Some(mouse_delay))
            .move_to_element_center(&board)
            .move_by_offset(start_file_offset, start_rank_offset)
            .click()
            .move_to_element_center(&board)
            .move_by_offset(end_file_offset, end_rank_offset)
            .click()
            .perform()
            .await
            .context("Failed performing piece movement action chain")?;

        Ok(())
    }

    async fn wait_for_chromedriver_to_be_ready() -> Result<(), Box<dyn Error>> {
        for _ in 0..10 {
            if reqwest::get("http://localhost:9515/status").await.is_ok() {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
        }
        Err(Box::from("Chromedriver process failed to initialize"))
    }
}

impl Drop for ChessDotComInterface {
    fn drop(&mut self) {
        self.chromedriver
            .kill()
            .expect("Failed to kill chromedriver process");
    }
}
