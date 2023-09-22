use std::error::Error;
use rusty_audio::Audio;

//-->> Start of main function
fn main() -> Result<(), Box<dyn Error>> {       //making main return a Result so we can use ? ergonomically. <() - for success, Box<dyn Error> box up a dynamic error trait for errors>
    let mut audio = Audio::new();               //creating a mutable audio, imported from rusty_audio
    //adding all audio to our audio manager. syntax -> audio.add("[name]", "[.wav file]");
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");    

    //playing the startup sound
    audio.play("startup");

    //if we leave the audio in the main we wont be able to hear anything coz the audio sys plays the audio in a separate thread in parallel
    //so when the main func ends all threads will be closed
    //so we add a cleanup section
    //Cleanup
    audio.wait();   //this will block until all the audio is done playing
    Ok(())
}
