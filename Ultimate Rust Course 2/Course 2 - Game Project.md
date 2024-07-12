# 3. Game Prototype with Rusty Engine

## 1. Project Overview
* there will be a tutorial section and a development section
* the `rusty_engine` is a wrapper around the `bevy_engine`, a real game engine: https://bevyengine.org/
* the `rust_engine` and tutorials can be found here: https://github.com/CleanCut/rusty_engine

## 2. Configuration
* add the `rusty_engine` to the Cargo.toml and then run `cargo build`
    ```toml
    [dependencies]
    rusty_engine = "5.2.1"
    ```
* this may take a while, which we can use to download the assets from [here](https://github.com/CleanCut/rusty_engine/tree/main/assets) 

## 3. Engine Initialization
* import the `rust_engine` prelude
* create an initial, mutable game object
* compiling this should open an empty window

## 4. Game State
* you will need to store the game's data
* that's the game state and it containts, for instance:
    * player's name
    * player's status & inventory
    * current scores, events, etc.
    * anything the needs to persist across a single frame
* let's also implement some default value for the game state with the trait `Default`

## 5. Game Logic Function
* a game is divided into frames
* a frame is one run through your game's logic to produce a new image to display on screen
* we need to define and add the game logic function to the game
* the game logic requires access to the engine and the game state
* as an example the game logic may increment the current score

## 6. Sprites
* a sprite in `rusty_engine` is a 2D image
    * its transform (translation, rotation, and scale)
    * its colliders
    * and other associated metadata
* we will use sprites for all in-game things like
    * player character, a car, etc.
* sprites are created through the engine struct
* sprites have label by which they can be called from the assets
* the **position** of the sprite on screen can be changed by the attribute `sprite.translation = Vec2::new(x: f32, y: f32)`
* the **rotation** of the sprite on screen can be changed by the attribute `sprite.rotation = std::f32::consts::FRAC_PI_2`
    * sprites are assumed to be facing right at zero degrees
    * 0 is right, π/2 is up, π is down, 3/2π is down 2π is full circle
    * the `rust_engine::prelude` also includes constants for directions
* the **size** of the sprite on screen can be changed by the attribute `sprite.scale = 3.0`
    * large than 1.0 increases the size
    * smalelr than 1.0 decreases the size
* the **layer** of the sprite on screen can be changed by the attribute `sprite.layer = 1.0`
    * the default layer is 0.0
    * NOTE: sprites on the same layer will non-deterministically overlap each other
    * thus, we should explicitly control the layer position 

## 7. Colliders
* all sprites also come with colliders that are disabled
* the `rusty_engine` comes with a demo on colliders
* to enable collision set the attribute `sprite.collision = true`
* we can add another sprite to collide into 
* a collision causes a two collision event
    * at the start and the end of the collision
    * a collision event also shows the pair of objects that collided
* colliders are convex polygon that can
    * detect collisions between sprintes
    * be displayed on screen `engine.show_colliders = true`

## 8. Keyboard Input
* for games to be interactive, it needs to process user input
* `rusty_engine` simplifies handling keyboard and mouse input
* there are keyboard states and keyboard events
* keyboard events will handle multiple inputs (like typing in a player's name)
    * we'll not use it in this tutorial
* the keyboard state is a snapshot of which keys are pressed at the beginning of each frame
    * this best for interactive things like character movements
* the keycode is an enum with all possible keys on a keyboard 
    * this comes from the bevy engine

## 9. Mouse Input
* there are mouse states and mouse events
* the mouse state also provides the mouse location, meaning that if the player clicks on the screen we can directly create an interaction

## 10. Text
* in `rusty_engine` text handes very similar to sprites
* text has a 
    * position (translation)
    * scale 
    * rotation
* additionally, text has a font-type and font-size
* NOTE: changing font-size is expensive but changing scale is cheap, so use the sprite-style attributes
* inside the main function, we can add text just like we added sprites
    ```rust
    let score = game.add_text("score", "Current Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);
    ```
* then in the game logic, we can access these text objects through the engine 
    ```rust
    let score = engine.texts.get_mut("score").unwrap();
    score.value = format!("Current Score: {}", game_state.current_score);
    ```

## 11. Audio
* `rusty_engine` has support for one looping audio-track like a background music
* but you can play a few concurrent sound-effects (at least 12, but it can depend on the hardware)
* there are some tracks and sounds included in the asset directory
* supported audio formats are `ogg`, `mp3`, `flac`, `wav`
* audio is accessed through the audio manager inside the game engine `game.audio_manager`
* the method `.play_music()` plays looping music, it takes parameters:
    * MusicPreset or music file path
    * float between 0.0 and 1.0 for volume
* sound effects are played in a "fire and forget" manner
    * each on a different channel in available
    * they terminate when the reach their end
* the method `.play_sfx()` takes parameters:
    * SfxPreset or file path
    * float between 0.0 and 1.0 for volume

## 12. Timer
* `rusty_engine` re-exports bevy's timer struct
* timers are cheap performance-wise
* create them with 
    ```rust
    Timer::from_seconds(2.0, false);
    ```
    * the number of seconds to count down
    * and a boolean for whether the timer is repeating
* timers are updated by the method `.tick()`
* if you don't tick a timer it is effectively paused
* tick returns an immutable reference to the timer
* together with what we learned so far, we can now spawn obejcts with a timer on the screen
* add `rand` to Cargo.toml for generating random positions
    ```toml
    [dependencies]
    rand = "0.8"
    ```
* now we have the essential pieces to build a small game

## 13. Engine & Game Structs
* let's learn some more vital information about the engine and game structs
* the `enine` parameter passed to the game_logic is a mutable reference to an Engine struct
* the engine already cleanly exits if you hit `ESC` or the press the close button no the window
* but we can also a quit function with `engine.should_exit = true;` 
* so far we have relied on absolute positions of our sprites and texts, but if you resize the window they may disappear
* let's make the relative to the screen size
* another usefule thing is the `time_since_startup_f64`
    * a value that tracks how long the game has been running
    * this can be usef for periodic animations like oscilations or circles
* we can use this to make our score-text hover up and down
* there are also more options to configure the `window_settings`
* this tales a `WindowDescriptor` struct with
    * a title, a width, and a height
    * the rest can be set to default

## 14. Common Setup
* every game scenario in `rusty_engine` goes through the same setup
* this setup can be found [here](https://github.com/CleanCut/rusty_engine/tree/main/scenarios#common-setup-do-this-first)
    1. create a new Rust project
    2. add ´rusty_engine´ to Cargo.toml
    3. download asset pack
    4. copy the skeleton logic
    5. then compile in release mode
* the basic setup needs some time to compile but only needs to be compiled once (it's done when a gray window opens)

## 15. Road Race
* we'll build the road race game ([repo tutorial](https://github.com/CleanCut/rusty_engine/blob/main/scenarios/road_race.md))
* in the game state, implement player health
* in the game loop, add a player sprite
* in the game logic, take player's keyboard input and translate into movements
    * let's make it so the car also looks like it's turning
    * going off the road leads to losing the game
* we also want to the car/road to appear moving
    * they should move from right to left
    * and re-appear again
* for a challenge, we want obstacles to appear at random
    * we need to the `rand` crate
    * obstable sprites are in the assets
* obstacles need to have a collision event with the player
    * any other collisions can be skipped
    * there should be a sound on collision and health should drop
* when the player runs out of health, it's game over
    * display a game over messages