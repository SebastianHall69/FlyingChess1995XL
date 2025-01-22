mod chess_bot;
mod chess_game;
mod web_interface;
mod engine;

use chess_bot::chess_bot::ChessBot;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = ChessBot::new().await?;
    bot.run_bot_state_machine().await
}

/*
NOTES
<wc-simple-move-list>
    - class = 'move-list-row' - locate all moves played
    - get latest
    - class = 'white-move' or 'black-move' to determine latest move
 */

/*
BUGS
- Using resign button for is_match_in_progress breaks on daily chess. We hate daily chess though so
  this is fine for now. We can use clock icon instead to indicate match start and end game modal to
  indicate match over
 */
