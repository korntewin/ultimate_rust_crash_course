use rusty_time::timer::Timer;
use std::time::Duration;
use crate::frame::{Frame, Drawable};

pub struct Shot {
    x: usize,
    y: usize,
    explode: bool,
    timer: Timer,
}


impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x, y,
            explode: false,
            timer: Timer::from_millis(40),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);

        if self.timer.ready && !self.explode {
            if self.y > 0 {
                self.y -= 1
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.explode = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.explode && self.timer.ready) || (self.y == 0)
    }
}


impl Drawable for Shot {
    fn draw(&self, screen: &mut Frame) {
        screen[self.x][self.y] = if self.explode {"*"} else {"|"}
    }
}