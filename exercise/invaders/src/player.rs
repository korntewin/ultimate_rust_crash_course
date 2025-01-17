use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
use crate::invaders::{Invader, Invaders};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}


impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS/2,
            y: NUM_ROWS - 1,
            shots: vec![],
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

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.shots.iter_mut().for_each(|shot| shot.update(delta));
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn hit_something(&mut self, invaders: &mut Invaders) -> bool {
        // loop through the invaders and shots to identify that which invader is hit
        // remove that invader out of the frame
        let mut is_hit = false;

        for shot in self.shots.iter_mut() {
            if invaders.detect_hit(shot) {
                shot.explode();
                is_hit = true
            }
        }
        is_hit
    }
}


impl Drawable for Player {
    fn draw(&self, screen: &mut Frame) {
        screen[self.x][self.y] = "A";
        self.shots.iter().for_each(|shot| shot.draw(screen));
    }
}