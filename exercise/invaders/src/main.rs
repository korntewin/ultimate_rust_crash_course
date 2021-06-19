use crossterm::cursor::{Hide, Show};
use crossterm::event;
use crossterm::{terminal, ExecutableCommand};
use rusty_audio::Audio;

use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::io;
use std::time::Duration;

use invaders::run;

fn main() {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");

    // run game
    let render_handle = run(audio);

    // wait for the thread to finish and drop receiver
    render_handle.unwrap().join();

}
