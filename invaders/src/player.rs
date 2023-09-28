use crate::{
    frame::{Drawable, Frame},
//    invaders::Invaders,
//    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

//Creating s struct for player
pub struct Player {
    x: usize,               //horizontal position
    y: usize,               //vertical position
//    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            //Setting the initial starting postion
            x: NUM_COLS / 2,            //middle of the columns
            y: NUM_ROWS - 1,            //last row at bottom, y=0 at top of screen and increases as we go downt he screen 
//            shots: Vec::new(),
        }
    }

    //For moving the player
    //Move Left
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    //Move Right
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

/* 
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
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
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
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
*/

}



//Drawing the player in screen
impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";            //character for our player (looking like a space ship)
 //       for shot in self.shots.iter() {
 //           shot.draw(frame);
 //       }
    }
}
