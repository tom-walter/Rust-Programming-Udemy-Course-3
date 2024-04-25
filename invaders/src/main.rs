use std::{error::Error, sync::mpsc, thread, time::Duration};
use invaders::{frame::{self, new_frame, Drawable}, player::Player, render};
use rusty_audio::Audio;
use std::io;
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};

fn main() -> Result<(), Box<dyn Error>> {
    // add all audio files
    let mut audio = Audio::new();

    audio.add("explode", "explode.wav"); 
    audio.add("lose", "lose.wav"); 
    audio.add("move", "move.wav"); 
    audio.add("pew", "pew.wav"); 
    audio.add("startup", "startup.wav"); 
    audio.add("win", "win.wav"); 

    audio.play("startup");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // rendering loop in seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // create player
    let mut player = Player::new();

    // game loop
    'gameloop: loop {
        // per frame init
        let mut curr_frame = new_frame();

        // input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // moving player
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    // closing game
                    KeyCode::Esc | KeyCode::Char('q') => {audio.play("lose");
                    break 'gameloop; 
                    }
                    _ => {}
                }
            }
        }
        // draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        // draw refresh rate
        thread::sleep(Duration::from_millis(1));

    } 

    // clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
