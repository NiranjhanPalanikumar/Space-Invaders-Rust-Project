use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

//Fn arguments
//Stdout -> to rendrr
//last_frame, curr_frame -> we will only render what has changed 
//force -> to force render entire frame
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();                     //clearing the screen and setting background to blue
        stdout.queue(Clear(ClearType::All)).unwrap();                               //clear all of the screen
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();                    //setting playing field background to black
        stdout.queue(SetForegroundColor(Color::White)).unwrap();                    //
    }

    //iterating through the entire frame and draw whatever is changed
    for (x, col) in curr_frame.iter().enumerate() {                                 //x value loop                       
        for (y, s) in col.iter().enumerate() {                                      //y value loop
            if *s != last_frame[x][y] || force {                                    //"*s" -> dereferencing; comparing with the position at last_frame(if changed) [OR] if "forcing" to draw everything 
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);                                                   //prints the character at s
            }
        }
    }
    stdout.flush().unwrap();                                                        //update to the screen
}


