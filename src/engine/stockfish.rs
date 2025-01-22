use std::error::Error;
use std::process;

pub struct Stockfish {
    process: process::Child,
}

impl Stockfish {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            process: spawn_stockfish_process()?,
        })
    }

    pub fn _get_move() {
        println!("Get move not yet implemented");
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        self.process
            .kill()
            .expect("Failed to kill stockfish process");
    }
}

fn spawn_stockfish_process() -> Result<process::Child, Box<dyn Error>> {
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
