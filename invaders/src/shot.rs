use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;

//struct for shot
pub struct Shot {
    pub x: usize,               //x-position
    pub y: usize,               //y-position
    pub exploding: bool,        //to track explosion
    timer: Timer,               //internal timer to keep track of movement
}

//implementing shot object
impl Shot {
    //constructing a shot
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,                       //false because it is the initial state after shooting
            timer: Timer::from_millis(50),          //shot will move upwards one cell every 50 milliseconds
        }
    }

    //updating the internal timer for shooting
    pub fn update(&mut self, delta: Duration) {     //delta -> duration from standard time
        self.timer.update(delta);                   //updating timer with delta, this will make the timer start counting down by "delta" amount
        if self.timer.ready && !self.exploding {    //2nd condition make sure the laser shot is not moving any further aftr exploding
            if self.y > 0 {                         //to check if shot has reached the top of the screen
                self.y -= 1;                        //moving 1 cell upwards
            }
            self.timer.reset();                     //reseting the timer
        }
    }

    //function to handle explosion
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);       //increased time to show the explosion as 50 ms will be too short
    }

    //function to tell when we are dead
    pub fn dead(&self) -> bool {                                //returning "true" or "false"
        (self.exploding && self.timer.ready) || (self.y == 0)   //condition-1 -> means that we hv exploded and the time has run out or condition-2 -> implies we reached the top of screen
    }
}

//drawing the shot fired
impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { '*' } else { '|' };     //if exploding-> '*' symbol else shot continues till it reaches top '|'
    }
}
