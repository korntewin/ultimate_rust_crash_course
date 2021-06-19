use std::io;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;

pub fn render(stdout: &mut Stdout, cur_frame: &Frame, new_frame: &frame, force: bool) {

    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, rows) in cur_frame.iter().enumerate() {
        for (y, val) in rows.iter().enumerate() {
           if *val != cur_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *val);
           } 
        }
    }

    stdout.flush().unwrap();

}

#[cfg(test)]
mod tests {
    use crate::render::*;

    #[test]
    fn test_render() {
        render(&mut io::stdout(), frame::new_frame(), frame::new_frame(), true)
    }
}
