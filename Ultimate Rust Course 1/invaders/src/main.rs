use std::{error::Error, sync::mpsc, thread, time::{Duration, Instant}};
use invaders::{frame::{self, new_frame, Drawable}, invaders::Invaders, player::Player, render};
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

    // create objects
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // game loop
    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // moving player
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    // shooting
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() { audio.play("pew"); }
                    }
                    // closing game
                    KeyCode::Esc | KeyCode::Char('q') => {audio.play("lose");
                    break 'gameloop; 
                    }
                    _ => {}
                }
            }
        }
        // updates
        player.update(delta);
        // make sound of invaders moving
        if invaders.update(delta) { audio.play("move"); }
        // make sound of hitting invader
        if player.detect_hits(&mut invaders) { audio.play("explode"); }

        // draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables { drawable.draw(&mut curr_frame); }
        let _ = render_tx.send(curr_frame);
        // draw refresh rate
        thread::sleep(Duration::from_millis(2));

        // win?
        if invaders.all_killed() { 
            audio.play("win");
            break 'gameloop
        }
        // lose?
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop
        }


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
