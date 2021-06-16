use std::io;
use std::error::Error;
use std::time::Duration;
use crossterm::event;
use crossterm::{terminal, ExecutableCommand};
use crossterm::cursor::{Hide,Show};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Esc | event::KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop
                    },
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    audio.wait();

    Ok(())
}
