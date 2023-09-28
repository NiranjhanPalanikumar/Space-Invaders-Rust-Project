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

use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    //invaders::Invaders,
    //level::Level,
    //menu::Menu,
    player::Player,
    render,
    //score::Score,
};


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

    
    //Render loop in a separate thread
    //setting up a channel to communicate with our thread
    let (render_tx, render_rx) = mpsc::channel();
    //thread
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();                //variable to hold the last_frame (initialised with new empty frame)

        //rendering stdout
        let mut stdout = io::stdout();

        //ready to render the entire screen
        render::render(&mut stdout, &last_frame, &last_frame, true);    //2nd "last_frame" arg is give in place of current frame -> because in the beginning everything has to be drawn. and "true" for force draw

        //performing the incremental updates in a loop
        loop {
            //getting our current_frame
            //moving render_rx into this thread. This returns a result -> either "current_frame" or "error" if channel is closed
            //matching with the result
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,                                     //if ok, then returns a frame
                Err(_) => break,                                //if error, break out of the rendfer loop shutting down the child thread
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);                                    
            last_frame = curr_frame;                            //setting up last_frame for the next loop
        }
    });


    //Game loop
    //---------------------

    let mut player = Player::new();
    'gameloop: loop{                //named loop so that we can exit from any point necessary

        //Per frame initialisation
        let mut curr_frame = new_frame();

        
        //Input handling
        while event::poll(Duration::default())? {               //poll funcion takes a duration [using default() which is 0]
            if let Event::Key(key_event) = event::read()? {     //checking for key_events
                match key_event.code {                          //matching for specific keycodes for different events
                    KeyCode::Left => player.move_left(),        //left arrow to move player left
                    KeyCode::Right => player.move_right(),      //right arrow to move player right
                    KeyCode::Esc | KeyCode::Char('q') => {      //for exiting the game
                        audio.play("lose");                     //play losing sound because we exited early
                        break 'gameloop;                        //breaking out of the game loop
                    }
                    _ => {}                                     //temporary ignoring all key press to check if game exits when 'Esc' or 'q' is pressed
                }
            }
        };    


        //Draw & Render section
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);                     //will fail the first few times, because this game loop will get going before that child thread is up and start receiving, which will not be available
        thread::sleep(Duration::from_millis(1));                //game loop is faster than render loop. to balance the delay and not spend too much resource on rendering
    }


    //Cleanup section
    //-----------------------

    //joining the thread for good clean up practice
    drop(render_tx);
    render_handle.join().unwrap();

    //if we leave the audio in the main we wont be able to hear anything coz the audio sys plays the audio in a separate thread in parallel
    //so when the main func ends all threads will be closed

    audio.wait();                               //this will block until all the audio is done playing
    stdout.execute(Show)?;                      //showing the cursor
    stdout.execute(LeaveAlternateScreen)?;      //leaving the alternate screen
    terminal::disable_raw_mode()?;              //disabling the raw mode
    Ok(())
}
