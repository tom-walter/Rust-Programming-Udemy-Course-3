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
## 7. Coliders
## 8. Keyboard Input
## 9. Mouse Input
## 10. Text
## 11. Audio
## 12. Timer
## 13. Engine & Game Structs
## 14. Common Setup
## 15. Road Race