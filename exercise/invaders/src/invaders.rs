use crate::{NUM_ROWS, NUM_COLS};
use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;
use Direction::{Right, Left, Down};

pub struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    army: Vec<Invader>,
    timer: Timer,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
    Down,
}

const INVADER_FRONT_RATIO: f32 = 0.45;

impl Invaders {
    pub fn new() -> Self {

       let mut army = Vec::new();

        (0..NUM_COLS)
            .for_each(|x| (0..NUM_ROWS)
                .for_each(|y| {
                    if x > 2 && x < NUM_COLS-3
                        && y < (INVADER_FRONT_RATIO*NUM_ROWS as f32) as usize
                        && y > 1 && x % 2 == 0 && y % 2 == 0
                    {
                       army.push(Invader {x, y})
                    }
                }));

        Self {
            army,
            timer: Timer::from_millis(1500),
            direction: Direction::Right,
        }
    }

    fn right_most_pos(&self) -> usize {
        // find the right most position of the army
        self.army
            .iter()
            .map(|invader| invader.x)
            .max().unwrap()
    }

    fn left_most_pos(&self) -> usize {
        // find the left most position of the army
        self.army
            .iter()
            .map(|invader| invader.x)
            .min().unwrap()
    }

    fn update_direction(&mut self) {
        let right_most = self.right_most_pos();
        let left_most = self.left_most_pos();

        if right_most == NUM_COLS - 1{
            match self.direction {
                Right => self.direction = Down,
                Down => self.direction = Left,
                _ => (),
            }
        } else if left_most == 0 {
            match self.direction {
                Left => self.direction = Down,
                Down => self.direction = Right,
                _ => (),
            }
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.timer.update(delta);

        if self.timer.ready {
            match self.direction {
                Left => self.army.iter_mut().for_each(|invader| invader.x -= 1),
                Right => self.army.iter_mut().for_each(|invader| invader.x += 1),
                Down => for invader in self.army.iter_mut() { invader.y += 1},
            }

            self.update_direction();
            self.timer.reset();

            return true
        }

        return false

    }
}


impl Drawable for Invaders {
    fn draw(&self, screen: &mut Frame) {
        self.army
            .iter()
            .for_each(|invader| {
                match self.timer.time_left.as_secs_f32() / self.timer.duration.as_secs_f32() {
                    ratio if ratio > 0.5 => screen[invader.x][invader.y]="x",
                    _ => screen[invader.x][invader.y]="+",
                }
            })
    }
}