# Space-Invaders-Rust-Project

All the files are stored in the "invaders" folder.


## main.rs
The main function has  sections:
1. Main Section 
2. Render loop in a separate thread
2. Game Loop Section
4. Draw and render section
3. Clean Up Section

### Main function
Modified the main() function call to return a Result, so we can use '?' ergonomically in the code. <() - for success, Box<dyn Error> box up a dynamic error trait for errors>

Created a mutable audio struct from "rusty_audio" and added all the soundclips that will be played at various instances in the game.

Initialising and entering the alternate terminal in which the game will be run. 


### Render Loop Section
This is done in a separate thread using mpsc channel to setup and communicate with our thread. A thread is spawned initialised with variable "render_handle" taking in the argument "move" or "performing an incremental update". The "last_frame" is intialised with an empty new frame and renders the entire screen to start the game. The incremental loop is started here which gets the current_frame from the receiver channel. This retuns either the "curent_frame" or "error if the channel has been closed implying the game loop is terminated. The result is matched accordingly and breaks for the latter case. The screen is rendered with the resulting current frame, and the last_frame is updated with the current_frame, continuing the loop. 


### Game Loop Section
This section runs the named loop ['gameloop] so that it can be exited at any required moment.

Various key inputs are matched to their respective commands which are as follows:
- Exits the game if 'Esc' or 'q' is pressed


### Draw and Render Section
The render transmitter sends the current frame. The thread is made to pause and sleep for 1 millisecond because the game loop runs faster thant he render loop. This might not be problem in high end machine, but to balance out the delay and not invest too much resource on rendering this small sleep duration is added. 


### Clean Up Section
This section has all the commands to reverse the actions made:
- Dropping the render transmitter.
- Joining the render handle, this is not mandatory in the current scenario but is a good practice for complex cases.
- Having a .wait() function for audio, to keep the audio tread in hold after it finishes playing. This done because the audio thread is running in parallel and will terminate all the threads when comes to an end
- Showing the hidden cursor after end
- Exiting the alternate terminal screen used for the game
- Disabling the key inputs


## Module files

## lib.rs
This is the root module of our directory.
Initialises the number of rows and columns, and makes the other modules avaliable to use.


## frame.rs
Initialise a type Frame which is a Vector of Vector of borrowed static string slices. 

- function new_frame()
Returns an output of type Frame. A loop is run for total number of columns, and in each iteration a single column is constructed looping through the number of rows. Each generated column is pushed individually into the final vector creating the whole saetup.

Adds a trait 'Drawable' with function draw() that takes in the arguments immutable 'self', and a mutable reference to a frame.


## render.rs
This module has a function render() that takes int he arguments mutable reference for stdout for rendering, immutable reference of current frame and last frame of type Frame, and a boolean value for force which renders the whole screen if true or renders the changes from last frame to current frame if false.

If the boolean value for force is true 
- Sets the background color to Blue, and clears the whole screen. 
- Sets the background color of the playing area to black, to show a clear boundary.
- Sets the foreground color to white for all our objects and characters

If the boolean value for force is false
Iterates through the whole frame in current frame and draws the changes that have occurred since the previous frame.


## player.rs
This module handles all the informtion for the player.

The Player struct has the 2 positional parameters 'x' and 'y' and a Vector of Shot for shots fired

The player object is implemented at the bottom row and in the middle, which are num_rows - 1 and num_cols/2 respectively. Different functions are created for each action

- function move_left() - it makes sure the value is greater than 0 and decreases it by 1 unit.
- function move_right() - it makes sure the value is less than num_cols-1, and increases it by 1 unit.


Implementing Drawable for player. The function takes in 2 arguments an immutable reference to self, and a mutable reference to Frame. The player is represented with the character 'A' with the current location. 


## shot.rs
This module is to handle the laser shots fired fired by our spaceship. The laser is represented by the "|" symbol and explosion after itting a target is represented by '*' symbol.

A struct is intialised position 'x' and 'y', a 'bool' for explosion (default false), and a 'timer' to track the movement. 

The Shot object is implemented with 
- function new() to initialise its position position and 50 ms timer. 
- function "update()" to update the timer and move the laser shot upward making sure it doesnt do go out of bounds, and finally resetting timer for the next. 
- function explode() to show the result of shot
- function dead() to signify if we died in the game

Drawing the shot fired using function draw(), this takes the position values in the frame and draws the laser shot fired with a '|' or '*'.

