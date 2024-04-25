use crate::{frame::{Drawable, Frame}, NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new () -> Self {
        Self {
        x: NUM_COLS / 2, // center horizontal
        y: NUM_ROWS -1, // last row, bottom 
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x <= NUM_COLS -1 {
            self.x += 1;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}


impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "🚀";
    }
}