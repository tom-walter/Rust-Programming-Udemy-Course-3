# 1. Introduction

## 1. Intro to the Course
* this is a hands-on course for practical, everyday Rust knowledge
* the contents includes intermediate concepts like
    * idomatic Rust
    * documentation
    * traits
    * testing
    * error (handling)
    * testing
    * and more
* at the end of the course, we'll build a game prototype with the Rust engine
    * this requires a physical machine with a graphics card

## 2. Exercise Overview
* this course comes with exercises available through [this repo](https://github.com/CleanCut/ultimate_rust2)
* you can clone this repo to do the exercises on your own machine or in a virtual environment with Rust compiler toolchain
* each exercise is a small, standalone Rust project, which will compile and run, if you complete tasks

# 2. Intermediate Concepts

## 1. Idomatic Rust
### What is "_ideomatic_"?
1. expressions that are natural to a native speaker
2. appropriate to the style of a particular group
* so ideomatic Rust means using it like a daily practitioner or how it is commonly used by the Rust community

### Tools for Ideomatic Rust
* there are 2 main tools for writing ideomatic Rust
1. `rustfmt`
    * it is the de facto Rust style guide
    * can be run with `cargo fmt` command
    * it takes care of indentation, brace-alignment, trailing commas, etc.
    * it can also be configured through a `.rustfmt.toml` file at the root of the project
2. `clippy`
    * checks for over 450 specific problems
    * there are 4 main types of problems
        1. style
        2. correctness
        3. complexity
        4. performance
    * this makes clippy extremly useful
    * checkout all [linting rules](https://rust-lang.github.io/rust-clippy/master/)
    * can be run with `cargo clippy` command 

### Attributes
* attributes are code decorators that allow you to modify a function, struct or script with very little code
* they always start with a `#` and are framed by `[]`
* e.g.
    ```rust
    #[allow(clippy::too_many_arguments)]
    ```

### Exercises
* I used the following commands
    ```
    cargo fmt
    cargo clippy
    cargo clippy --fix --bin exercise_a --allow-staged
    cargo run
    ```
* I manually implemented the suggestions from `clippy` that could not be done automatically

## 2. Documentation
### Generating Documentation 
* documentation is only useful, if it's readable
* Rust can generate a website (html-files) with standardized documentation layout with ` cargo doc`
* but even better is the command `cargo doc --no-deps --open`
    * `--no-deps` will not render documentation for dependencies (only for your code)
    * `--open` will open the index page of the documentation

### Writing Documentation
* there are two styles of documentation
    1. triple slash (per-line style)
    2. double asteriks (block style)
* these are outer doc-strings (for functions, struct and constants)

#### Triple Slash
* each line of a doc-string beginswith 3 slahes `///`
* we want to document the following constant
    ```rust
    /// Number of pieces in the puzzle
    pub const PUZZLE_PIECES: u32 = 42;
    ```
* these doc-strings are always written above the constant or function-signature
* the rendering of doc-string support common **markdown syntax** with headers, bold/italic, links, code snippets, etc.
    * beware that code snippets inside doc-strings must be valid rust
* let's add some infos to our constant:
    ```rust
    /// Number of pieces in the puzzle
    ///
    /// # History
    ///
    /// This is a separate paragraph.
    /// - Clickable link: [`PUZZLE_PIECES`]
    /// - We tried `7`, but this is better 🧩
    pub const PUZZLE_PIECES: u32 = 42;
    ```
* here is another example for a `struct`
    ```rust
    /// This is a puzzle
    pub struct Puzzle {
        /// Number of pieces
        pub num_pieces: u32,
        /// Descriptive name
        pub name: String,
    }
    ```
* and for associated `impl`
    ```rust
    impl Puzzle {
        /// Make a new puzzle
        pub fn new() -> Self {
            Self {
                num_pieces: PUZZLE_PIECES,
                name: "Forest Lake".into(),
            }
        }
    }
    ```

#### Double Asteriks
* we can also write documentation using a multi-line comment style with double asteriks
    ```rust
    /**
    Number of pieces in the puzzle

    # History

    This is a separate paragraph.
    - Clickable link: [`PUZZLE_PIECES`]
    - We tried `7`, but this is better 🧩
    **/
    pub const PUZZLE_PIECES: u32 = 42;
    ```
* the rest of the markdown features are almost the same
* for markdown links
    * any code object in scope of the script can be linked
    * any code object out of scope can be linked with an absolute path
    ```rust
    /// [Spawn a thread](std::thread::spawn)
    ``` 

#### Inner Doc-Strings
* are used to document libraries and modules
* these are constructed with 
    * `//!` for per-line style
    * `/*! !*/` for block style 
* let's write documentation for our puzzle library
    ```rust
    //! HI! I'm a friendly Rust Puzzle Library.
    //!
    //! This is my documentation
    ```
### Exercise
* render the documentation by running `cargo doc --no-deps --open`
* challenge solution
    ```
    cargo doc --help | grep private
      --document-private-items  Document private items
    ```

## 3. Publishing
## 4. Closures
## 5. Iterators
## 6. Common Traits
## 7. Creating Errors
## 8. Handling Errors
## 9. Unit Tests
## 10. Integration Tests
## 11. Benchmarks
## 12. Logging
## 13. Multi-Threading
## 13. Channels

# 3. Game Prototype with Rusty Engine

## 1. Project Overview
## 2. Configuration
## 3. Engine Initialization
## 4. Game State
## 5. Game Logic Function
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