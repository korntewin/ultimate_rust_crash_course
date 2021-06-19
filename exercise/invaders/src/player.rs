use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Player {
    x: usize,
    y: usize,
}


impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS/2,
            y: NUM_ROWS - 1,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > (2.0/3.0 * NUM_ROWS as f32) as usize {
           self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

}


impl Drawable for Player {
    fn draw(&self, screen: &mut Frame) {
        screen[self.x][self.y] = "A";
    }
}