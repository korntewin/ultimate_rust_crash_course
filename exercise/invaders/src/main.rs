use crossterm::cursor::{Hide, Show};
use crossterm::event;
use crossterm::{terminal, ExecutableCommand};
use rusty_audio::Audio;

use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::io;
use std::time::Duration;

use invaders::frame;
use invaders::render;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // render loop in separate thread
    let (tx, rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let cur_frame = match rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &cur_frame, &last_frame, false);
            last_frame = cur_frame;
        }
    });


    // game loop
    'gameloop: loop {
        let curr_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Esc | event::KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        let _ = tx.send(curr_frame);
        thread::sleep(Duration::from_millis(10));
    }

    // Cleanup
    drop(tx);
    render_handle.join().unwrap();
    audio.wait();

    Ok(())
}
