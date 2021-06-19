use std::io::Stdout;
use rusty_audio::Audio;
use std::sync::mpsc;
use std::{thread, io};
use std::thread::JoinHandle;
use crossterm::event;
use std::time::Duration;
use crate::frame::Drawable;
use std::error::Error;

pub mod frame;
pub mod player;
pub mod render;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;


pub fn run(mut audio: Audio) -> Result<(JoinHandle<()>), Box<dyn Error>> {

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

    let mut my_player = player::Player::new();

    // game loop
    'gameloop: loop {
        let mut curr_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Up=> my_player.move_up(),
                    event::KeyCode::Down=> my_player.move_down(),
                    event::KeyCode::Left => my_player.move_left(),
                    event::KeyCode::Right=> my_player.move_right(),
                    event::KeyCode::Esc | event::KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {},
                }
            }
        }

        my_player.draw(&mut curr_frame);
        let _ = tx.send(curr_frame);
        thread::sleep(Duration::from_millis(10));
    }

    audio.wait();

    // Cleanup
    Ok(render_handle)

}