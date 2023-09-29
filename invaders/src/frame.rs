//Frame module

use crate::{NUM_COLS, NUM_ROWS};

//alias for our frame -> because the frame is going to be a vector of vector of borrowed static string slices
pub type Frame = Vec<Vec<& 'static str>>;

//to generate new frame
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);            //initiating the whole num of column "cols"
    for _ in 0..NUM_COLS {                                  //looping through num of columns to create individual coloumns (not inclusive)
        let mut col = Vec::with_capacity(NUM_ROWS);         //individual column vector "col"
        for _ in 0..NUM_ROWS {                              //looping through num of rows
            col.push(" ");                                  //adding blank spaces in a single column 
        }
        cols.push(col);                                     //pushing the column "col" into "cols"
    }
    cols                                                    //returning the full column of columns
}


//Adding trait
pub trait Drawable {                                        
    fn draw(&self, frame: &mut Frame);                      //immutable reference to our self and a mutable reference to a frame
}