mod character;
mod logger;
mod ui;

use crossterm::{
    event::{read, Event::Key, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use logger::Logger;
use std::io::Result;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut logger = Logger::new("logs", "menu.log")?;
    logger.log("Game started.")?;

    loop {
        if let Key(key_event) = read()? {
            // filter only key press events
            if key_event.kind != KeyEventKind::Press {
                continue;
            }
            match key_event.code {
                KeyCode::Char(c) => match c {
                    '1' => {
                        logger.log("Started New Game.")?;
                        print!("Starting a new game...\n");
                        break;
                    }
                    '2' => {
                        logger.log("Entered `Load Game` menu.")?;
                        print!("Loading a saved game...\n");
                        break;
                    }
                    '3' => {
                        logger.log("Game exited.")?;
                        print!("Exiting game...\n");
                        break;
                    }
                    _ => {
                        let message = format!("Made an invalid selection: '{}'", c);
                        logger.log(&message)?;
                        print!("Invalid selection. Please try again.\n");
                    }
                },
                // handle non-character inputs
                _ => {
                    logger.log("Made an invalid selection: Non-character key pressed.")?;
                    print!("Invalid selection. Please try again.\n");
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
