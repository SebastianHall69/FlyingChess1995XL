use crate::chess_move::ChessMove;
use anyhow::Context;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

const DEFAULT_MOVE_LIST: &str = "position startpos moves";
const MOVE_LIST_CAPACITY: usize = 1000;

pub struct Stockfish {
    process: Child,
    set_position_command: String,
}

impl Stockfish {
    pub fn new() -> Result<Stockfish, Box<dyn Error>> {
        let mut process = Command::new("./bin/stockfish_engine")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        match process
            .try_wait()
            .context("Failed to check status of Stockfish process")?
        {
            Some(status_code) => {
                return Err(Box::from(format!(
                    "Stockfish process exited with status: {status_code}"
                )));
            }
            None => {}
        }

        let mut engine = Stockfish {
            process,
            set_position_command: String::with_capacity(MOVE_LIST_CAPACITY),
        };
        engine.reset().expect("Failed to reset engine");

        Ok(engine)
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_position_command.clear();
        self.set_position_command.push_str(DEFAULT_MOVE_LIST);
        self.run_fishy_command("ucinewgame")
    }

    pub fn get_best_move(&mut self) -> Result<ChessMove, Box<dyn Error>> {
        self.run_fishy_command(self.set_position_command.clone().as_str())?;
        // self.run_fishy_command("go movetime 1000")?;
        self.run_fishy_command("go depth 1")?;

        let best_move_algebraic_notation = self
            .query_fishy_output("bestmove")?
            .split_whitespace()
            .nth(1)
            .context("Could not parse best move from Stockfish output")?
            .to_string();

        ChessMove::from_uci_notation(&best_move_algebraic_notation)
    }

    pub fn record_move_played(&mut self, chess_move: &ChessMove) {
        self.set_position_command
            .push_str(format!(" {}", chess_move.uci_notation()).as_str());
    }

    fn run_fishy_command(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        let input_stream = self
            .process
            .stdin
            .as_mut()
            .context("Failed to open Stockfish stdin")?;

        writeln!(input_stream, "{command}").context(format!("Failed to run command: {command}"))?;

        Ok(())
    }

    fn query_fishy_output(&mut self, pattern: &str) -> Result<String, Box<dyn Error>> {
        let stdout_buffer = BufReader::new(
            self.process
                .stdout
                .as_mut()
                .expect("Failed to open process stdout"),
        );

        for line in stdout_buffer.lines() {
            let line = line.context("Failed to read line from Stockfish stdout")?;
            if line.contains(pattern) {
                return Ok(line);
            }
        }

        Err(Box::from(format!(
            "Failed to find {pattern} in Stockfish output"
        )))
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        self.process
            .kill()
            .expect("Failed to kill stockfish process");
    }
}
