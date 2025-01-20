mod chess_bot;
mod chess_game;
mod web_interface;

use chess_bot::chess_bot::ChessBot;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = ChessBot::new().await?;
    bot.run_bot_state_machine().await?;
    Ok(())
}

/*
<wc-chess-board>
    - id = 'board-single' - to locate chess board
    - class = 'flipped' for black

<wc-simple-move-list>
    - class = 'move-list-row' - locate all moves played
    - get latest
    - class = 'white-move' or 'black-move' to determine latest move

<div>
    - class = 'clock-bottom' locate player clock
    - class = 'clock-player-turn' - identify if its a player's turn

Find all 4 new game buttons
- aria-label "Accept Rematch", "Decline Rematch", "Rematch", "button:not([aria-label])" for new game
- find button with <span>New xx min<span/>

 */
