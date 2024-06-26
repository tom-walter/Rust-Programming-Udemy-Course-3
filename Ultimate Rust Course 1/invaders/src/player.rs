use std::time::Duration;
use crate::invaders::Invaders;
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use crate::shot::Shot;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>
}

impl Player {
    pub fn new () -> Self {
        Self {
        x: NUM_COLS / 2, // center horizontal
        y: NUM_ROWS -1, // last row, bottom
        shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS -1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        // no more than 2 shots on screen
        if self.shots.len() < 2 {
            // shot should start directly above player
            self.shots.push(Shot::new(self.x, self.y- 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}


impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        // draw player
        frame[self.x][self.y] = "⟁";
        // draw shots
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}