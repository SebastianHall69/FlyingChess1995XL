use crate::chess_dot_com_interface::ChessDotComInterface;
use crate::game_board::GameBoard;
use crate::piece::PieceColor;
use crate::stockfish::Stockfish;
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

enum BotState {
    Start,
    Login,
    WaitForGame,
    GameFound,
}

enum MatchState {
    Start,
    MyTurn,
    WaitingForTurn,
    MatchOver,
}

pub struct ChessBot {
    site: ChessDotComInterface,
    engine: Stockfish,
}

impl ChessBot {
    pub async fn new() -> Result<ChessBot, Box<dyn Error>> {
        Ok(ChessBot {
            site: ChessDotComInterface::new().await?,
            engine: Stockfish::new()?,
        })
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut bot_state = BotState::Start;

        loop {
            match bot_state {
                BotState::Start => {
                    println!("Starting bot");
                    bot_state = BotState::Login
                }
                BotState::Login => {
                    println!("Logging in");
                    self.site.login().await?;
                    bot_state = BotState::WaitForGame;
                }
                BotState::WaitForGame => {
                    if self.site.is_match_available().await {
                        bot_state = BotState::GameFound;
                        println!("Found a game!");
                    } else {
                        println!("Nuttin yet chief...");
                    }
                }
                BotState::GameFound => {
                    self.play_match().await?;
                    bot_state = BotState::WaitForGame
                }
            }

            sleep(Duration::from_secs(3)).await;
        }
    }

    async fn play_match(&mut self) -> Result<(), Box<dyn Error>> {
        let mut old_board_state = GameBoard::new()?;
        let mut match_state = MatchState::Start;
        let color = self.site.get_player_color().await?;
        let is_board_flipped = match color {
            PieceColor::White => false,
            PieceColor::Black => true,
        };
        self.engine.reset()?;

        loop {
            if !self.site.is_match_available().await {
                match_state = MatchState::MatchOver;
            }

            match match_state {
                MatchState::Start => {
                    match color {
                        PieceColor::White => match_state = MatchState::MyTurn,
                        PieceColor::Black => {
                            match_state = MatchState::WaitingForTurn;
                            println!("Bro's really taking this long...");
                        }
                    };
                }
                MatchState::WaitingForTurn => {
                    if self.site.is_my_turn().await? {
                        println!("Bro finally");
                        match_state = MatchState::MyTurn;
                    }
                }
                MatchState::MyTurn => {
                    sleep(Duration::from_millis(150)).await;
                    let current_board_state = self.site.get_current_board_state().await?;

                    match old_board_state.get_move_from_difference(&current_board_state)? {
                        Some(move_made) => {
                            println!("MOVE MADE: {move_made}");
                            self.engine.record_move_played(&move_made);
                        }
                        None => println!("Starting position detected"),
                    }

                    let best_move = self.engine.get_best_move()?;
                    println!("BEST MOVE: {best_move}");

                    println!("ABOUT TO PLAY MOVE");
                    let human_thinking_time = rand::rng().random_range(4500..15000);
                    println!("A real human would clearly spend {} seconds thinking about this position", human_thinking_time / 1000);
                    sleep(Duration::from_millis(human_thinking_time)).await;
                    self.site.play_move(&best_move, is_board_flipped).await?;
                    println!("MOVE HAS BEEN PLAYED");

                    self.engine.record_move_played(&best_move);
                    sleep(Duration::from_millis(150)).await;
                    old_board_state.set_state(&self.site.get_current_board_state().await?); // TODO: CAN OPTIMIZE BY REMOVING NETWORK CALL

                    match_state = MatchState::WaitingForTurn;
                    println!("Bro's really taking this long...");
                }
                MatchState::MatchOver => {
                    println!("gg2ez");
                    break;
                }
            }

            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }
}
