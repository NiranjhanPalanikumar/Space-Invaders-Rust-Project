use crate::{
    frame::{Drawable, Frame},
//    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

//Creating s struct for player
pub struct Player {
    x: usize,               //horizontal position
    y: usize,               //vertical position
    shots: Vec<Shot>,       //shots vector
}

impl Player {
    pub fn new() -> Self {
        Self {
            //Setting the initial starting postion
            x: NUM_COLS / 2,            //middle of the columns
            y: NUM_ROWS - 1,            //last row at bottom, y=0 at top of screen and increases as we go downt he screen 
            shots: Vec::new(),          //initialising a new vector
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

    //Function for Shooting 
    pub fn shoot(&mut self) -> bool {                               //returns a boolean to make sure the shots does not exceed teh max number of shots
        if self.shots.len() < 2 {                                   //maximum number of shots allowed
            self.shots.push(Shot::new(self.x, self.y - 1));         //shot origin location, (x,y) represent the player location and (y-1) to show the shot starts above the spaceship
            true                                                    //sucessfully shot
        } else {
            false                                                   //unsucessful shot due to max value
        }
    }

    //Updating the timer 
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {                         //iterating through our shots
            shot.update(delta);                                     //updating it with delta
        }
        self.shots.retain(|shot| !shot.dead());                     //cleanup - retain shots that are no dead and have not reached the boundaries
    }

/* 
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
        for shot in self.shots.iter() {         //iterate through our shots
            shot.draw(frame);                   //draw each shot
        }
    }
}
