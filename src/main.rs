mod chess_bot;
mod chess_dot_com_interface;
mod chess_move;
mod game_board;
mod piece;
mod square;
mod stockfish;

use crate::chess_bot::ChessBot;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = ChessBot::new().await?;
    bot.main_loop().await?;

    println!("Cheater man signing off!");

    Ok(())
}
