# Space-Invaders-Rust-Project

All the files are stored in the "invaders" folder.


## main.rs
The main function has 3 sections:
1. Main Section 
2. Game Loop Section
3. Clean Up Section

### Main function
Modified the main() function call to return a Result, so we can use '?' ergonomically in the code. <() - for success, Box<dyn Error> box up a dynamic error trait for errors>

Created a mutable audio struct from "rusty_audio" and added all the soundclips that will be played at various instances in the game.

Initialising and entering the alternate terminal in which the game will be run. 



### Game Loop Section
This section runs the named loop ['gameloop] so that it can be exited at any required moment.

Various key inputs are matched to their respective commands which are as follows:
- Exits the game if 'Esc' or 'q' is pressed


### Clean Up Section
This section has all the commands to reverse the actions made, for example:
- Having a .wait() function for audio, to keep the audio tread in hold after it finishes playing. This done because the audio thread is running in parallel and will terminate all the threads when comes to an end
- Showing the hidden cursor after end
- Exiting the alternate terminal screen used for the game
- Disabling the key inputs


