use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

//use std::error::Error;
//use rusty_audio::Audio;
//use std::io;
//use crossterm::{terminal, ExecutableCommand};
//use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
//use crossterm::cursor::{Hide, Show};

//Main function
//-------------------------
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

    //Initialising terminal
    let mut stdout = io::stdout(); //getting acces to stdout
    terminal::enable_raw_mode()?;  //enable raw mode to get keyboard inputs. The '?' operator is used so thhat the program will crash if we have an error

    stdout.execute(EnterAlternateScreen)?;  //Entering our alternate screen
    stdout.execute(Hide)?;                  //hiding the cursor

    
    //Game loop
    //---------------------
    'gameloop: loop{                //named loop so that we can exit from any point necessary
        
        //Input handling
        while event::poll(Duration::default())? {               //poll funcion takes a duration [using default() which is 0]
            if let Event::Key(key_event) = event::read()? {     //checking for key_events
                match key_event.code {                          //matching for specific keycodes for different events
                    KeyCode::Esc | KeyCode::Char('q') => {      //for exiting the game
                        audio.play("lose");                     //play losing sound because we exited early
                        break 'gameloop;                        //breaking out of the game loop
                    }
                    _ => {}                                     //temporary ignoring all key press to check if game exits when 'Esc' or 'q' is pressed
                }
            }
        };    

    }


    //Cleanup section
    //-----------------------

    //if we leave the audio in the main we wont be able to hear anything coz the audio sys plays the audio in a separate thread in parallel
    //so when the main func ends all threads will be closed

    audio.wait();                               //this will block until all the audio is done playing
    stdout.execute(Show)?;                      //showing the cursor
    stdout.execute(LeaveAlternateScreen)?;      //leaving the alternate screen
    terminal::disable_raw_mode()?;              //disabling the raw mode
    Ok(())
}
