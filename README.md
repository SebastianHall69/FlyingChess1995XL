# FlyingChess1995XL

## Overview

This bot is a chess trainer and partner for playing games on [https://chess.com](https://www.chess.com/). You can launch the bot, the bot
will launch a GUI, sign in (update with your own credentials), and you can begin playing a game against your human account to test your skill.
Currently, the bot only supports standard chess games so this excludes puzzles and non standard game formats (4 player chess, etc).

## Platform Dependencies

The two executables in the `/bin` directory are platform specific. These will need to be replaced with platform specific downloads.

Note: Mac will require you trust these applications before the project will work. Attempting to run the project should trigger some
pop ups asking you to trust the stockfish and chromedriver executables.

### Stockfish CLI

1. Download a version of the [Stockfish CLI](https://stockfishchess.org/download/) executable that is compatible with your machine.
2. Rename the executable to `stockfish_engine`.
3. Place in the `/bin` directory of the project.

### Chromedriver

The project requires that Google Chrome be installed. I have had some issues with chromedriver not being able to detect my Google Chrome
installation because it was installed via Flatpak on Linux. Installing via a standard package manager (e.g. Fedora `dnf`) resolved the issue for me.

Chromedriver is required to match the version of your existing Google Chrome installation.

1. Open this link [chrome://settings/help](chrome://settings/help) in Chrome to get your version id.
2. Find your version (or the closest number to it) in the [Chromedriver JSON file](https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json).
3. In the JSON object for your version, grab the url for `chromedriver` and your `platform`.
4. Double check that you got the url for Chromedriver, not Chrome or Chrome headless shell.
5. Download the Chromedriver.
6. Unzip and move the chromedriver executable into the `/bin` folder of the project.
7. Ensure that the executable is just named `chromedriver`.

## How To Run

### Prerequisites

This project uses Rust [Cargo](https://doc.rust-lang.org/cargo/) package manager for compilation and build.

If you don't have Rust installed, you can [install it via Rustup](https://rustup.rs/).

### Build Commands

Generate executable

```
cargo build
```

Generate execute able and run
```
cargo run
```
