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


