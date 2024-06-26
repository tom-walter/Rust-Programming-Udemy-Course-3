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
### Publishing on Crates.io
* [crates.io](www.crates.io) is Rust's community crate registry
* whatever you publish there will be **permanent**
    * don't publish your private credentials, ssh keys, API keys, access tokens, etc.
* package names need to be _unique_ because there is only one global namespace for all crates

Publication Pre-Requisites
1. log into crates.io with you GitHub account
2. goto account settings and generate an access token
3. run `cargo login` using your access token
4. have package that you want to publish

Polishing the `Cargo.toml`
* the `Cargo.toml` has many additional fields that can provide useful information to the users of your crate
* here is an example from the ´rusty_engine´
    ```toml
    [package]
    name = "rusty_engine"
    version = "6.0.0"
    description = "Learn Rust with a simple, cross-platform, 2D game engine."
    edition = "2021"
    homepage = "https://github.com/CleanCut/rusty_engine"
    repository = "https://github.com/CleanCut/rusty_engine"
    readme = "README.md"
    keywords = [ "game", "engine", "graphics", "audio", "rusty" ]
    categories = [ "game-engines" ]
    license = "MIT OR Apache-2.0"
    ```
    * adding a code `repository` makes it easier for other users to contribute or report issues
    * `keywords` allows a max of 5 SEO-keywords to for better finding your crate on Crates.io
    * `categories` allows a max of 5 _valid category slugs_ from Crates.io

### Publishing
* after the initial setup and after you developed a package, you can publish with `cargo publish`
    ```console
    $ cargo publish --help
    Upload a package to the registry  

    Usage: cargo.exe publish [OPTIONS]

    Options:
    -n, --dry-run              Perform all checks without uploading
        --index <INDEX>        Registry index URL to upload the package to
        --registry <REGISTRY>  Registry to upload the package to
        --token <TOKEN>        Token to use when uploading
    -h, --help                 Print help

    Package Selection:
    -p, --package [<SPEC>]  Package to publish
    ```
* this will re-compile (only publishes if compiled without error), check the versioning, check for un-committed code, etc.
* anything published on Crates.io automatically gets its documentation published online on [docs.rs](https://docs.rs/) (for free)

### No Exercise
* there will be no exercise to prevent useless crates from littering Crates.io and limiting the the unique global namespace
* if personal namespacing will be implemented for Crates.io, this section will be updated with an exercise

## 4. Closures
### What are closures?
* a closure is an anonymous, enclosed function that can borrow or capture some data from the scope it is nested in
* the syntax consists of a parameter list in pipes and an expression (in curly braces)
    ```rust
    |params| expr
    |params| {expr1; expr2}
    ```
* it can also be assigned to a variable and called later 
* the compiler will automatically figure out the parameters types and return types by how you use the closure
* example
    ```rust
    let add = |x, y| x + y;
    let z = add(1, 2) // returns 3 
    ```

### Capturing and Borrowing
* a closure will automatically borrow a reference to the values in the enclosing scope
* example
    ```rust
    let s = "🍓";
    let f = move || {
        println!("{}", s);
    };
    f(); // prints 🍓
    ```
* the value borrowed by the closure must live at least as long as the closure (otherwise the code will not compile)
* closures also support `move` semantics which allows to transfer the ownership of the variable to the closure (remedying the lifetime issue)

### Closures in Higher-Order Function
* in order to use closures as function parameters, then need to be type-annotated from [`std:ops`](https://doc.rust-lang.org/std/ops/index.html) with
    * `Fn`
    * `FnMut`
    * `FnOnce`

## 5. Iterators
### Iterators
* in the previous course, we learned that `for` loop can iterate over any any iterable collection
* but what makes a collection data type iterable?
* a collection becomes iterable through the method `.int_iter()` which is implemeted onto a struct via the trait `IntoIterator`
* anything that implements this trait can called iterator function
* the method `.int_iter()` returns an iterator which takes _ownership_ of the collection it is called on and consumes the collection

Examples
* `for` loop style iteration
    ```rust
    let v = vec![6, 7, 8, 9];
    for n in v.into_iter() {
        println!("{}", n);
    }
    ```
    * a vector implements the trait `IntoIterator`
    * a vector automatically turns into an iterator in a `for` loop
    * that means if a vector goes out of scope at the end of the loop
* functional style iteration
    ```rust
    let v = vec![6, 7, 8, 9];
    v.into_iter().for_each(|num| println!("{}", num));
    ```
* functional iterator are faster than for loops
    * this is because `for` loop may be compiled line-by-line
    * whereas the whole functional iterator is evaluated by the compiler and optimized together

### Iterator Adaptors
* in functional paradigm, the iterator adaptor takes an iterator and outputs a different iterator and takes some action on values that are passed through 
* many iterator methods in Rust are also iterator adaptors, allowing for method chaining
    ```rust
    let v = vec![6, 7, 8];
    let total: i32 = v
        .into_iter()            //  6,  7,  8
        .map(|x| x * 3);        // 18, 21, 24
        .filter(|y| *y % 2 == 0)// 18, 24
        .sum();                 // 42
    ```
    * `.map()` applies a function or closure to each value (and passed them along)
    * `.filter()` evaluates a condition and only values that evaluate true under this condition are passed along
    * `map`'s closure takes ownership of its values but `filter`'s closure only takes an immutable reference
    * iterator adaptors need to end in _iterator consumer_ causing the chain of adaptors to actually process
* how do you what each iterator adaptor need?
    * every iterator adaptor is documented

### Iterator Consumers
    * `.for_each()` passes each values to a closure and consumes them indivdually
    * `.sum()` is another useful consumer
    * `.collect()` gather all the values passed through the iterator chain and puts them into a new collection
* but iterators rely heavily on generics that the compiler can't know anymore what is the result of the final data type
    * you can either annotate the variable the iterator feeds into
    * or you can use the turbofish `::<>` syntax
* the turbofish `::<>` declares the type of generic function
    * it goes between the method/function name and the argument list: `.sum::<i32>()`
* a special consumer is the `.drain()` that removes some or all values from a collection and returns the empty collection

### Mutable Iterators
* what if you don't want to consume the collection while iterating over it?
    * you can eihter call the method     
    ```rust
    v.into_iter()   // consumes v, returns owned items
    v.iter()        // returns immutable references
    v.iter_mut()    // returns mutable references
    ```
    * or use its syntactic sugar
    ```rust
    for _ in v      // v.into_iter()
    for _ in &v     // v.iter()
    for _ in &mut   // v.iter_mut()
    ```
* hashmaps have iterator methods that can go over keys only, values only, or key-value pairs together

## 6. Common Traits
* we will learn about traits from the standard library
* what can implement a trait? -> 4 things
    1. `struct`s
    2. `enum`s
    3. `closure`s
    4. `function`s
* but closures and functions can only implement a view advanced traits, we will focus on implementing traits for structs and enums

### Derivable Traits
* a trait can be derived if it has a `#[derive()]` macro defined it
* the documentation for a trait will tell if it can be derived

#### Debug
* the most common trait to derive is `Debug`
    ```rust
    #[derive(Debug)]
    pub struct Puzzle {
        pub num_pieces: u32,
        pub name: String,
    }
    ```
* as long as all elements inside your struct or enum also have this trait, it can be derived for the super entity
* all primitives and most standard library types already posses the trait
* Debug trait is used for debug formatting
```rust
println!("{:?}", puzzle); // Debug 
println!("{:#?}", puzzle); // Pretty Debug
```
* `"{:?}"` tries to create a one line representation of the struct
* `"{:#?}"` tries to create a multi-line, indented representation of the struct

#### Clone
* another common trait to derive is `Clone`
    * allows an object to be cloned by called the method `.clone()`
    ```rust
    #[derive(Clone, Debug)]
    pub struct Puzzle {
        pub num_pieces: u32,
        pub name: String,
    }
    // ...
    let puzzle2 = puzzle.clone()
    ```
* Rust is not an interpreted language like Python or Ruby, where the interpreter can introspect objects at run-time
* therefore, Rust objects need to implement `Clone` at compile-time

#### Copy
* related to `Clone` is the `Copy` trait 
    * copy is a sub-trait of clone, meaning it cannot be derived without the parent trait 
* if your type implements `Copy`, then it will be copied instead of moved in `move` situations
* this makes sense for small values that fit entirely on the stack (integer, float, boolean have copy trait)
* if a type uses the heap, it cannot implement `Copy`
* for our `Puzzle` example, we cannot derived `Copy` since `String` uses the heap
* another example with single type `enum`
    ```rust
    #[derive(Clone, Copy)]
    pub enum PuzzleType {
        Jigsaw
    }
    ```
    * small types likes enums are easier to copy than to deal with references and move semantics

### Standard Library Traits
* there are 3 steps to manually implement a trait
    1. bring it into scope
    * some traits are in the standard library prelude (automatically in scope)
    * but many useful traits need to imported first
    2. boilerplate
    * write the interfaces that are needed for this trait
    * your IDE can generate a skeleton or you can copy it from the documentation
    3. implementation
    * write the actual functions that you want to implement

#### Default
* the `Default` trait is included in the standard library prelude (already in scope)
* it gives you zero numbers, empty strings, and empty collections, but we can use to define our type's own default-values
    ```rust
    impl Default for Puzzle {
        fn default() -> Self {
            todo!();
            //panic!("not yet implemented");
        }
    }
    ```
* let's write the actual implementation
    ```rust
    impl Default for Puzzle {
        fn default() -> Self {
            Puzzle {
                num_pieces: PUZZLE_PIECES,
                name: "Forest Lake".to_string(),
            }
        }
    }
    ```
* there are several patterns of idiomatic usage
    * initialize a struct with some default values
    ```rust
    let puzzle = Puzzle {
        num_pieces: 3000,
        ..Default.default()
    }
    ```
    * the `..` is the range operator and using it this way fills out all remaining values (called "struct update syntax")

#### PartialEq / Eq
* partial equality and equality traits are `PartialEq` and `Eq`
* `PartialEq` does the actual calculations to test for equality
* `Eq` is a marker trait for when the equality logic is _reflexiv, transitive and symmetric_
    * _reflexiv_: any number `a` is equal to itself `a == a`
    * _transitive_: for any numbers: `a, b, c`; if `a == b` and `b == c`, then `a == c`
    * _symmetric_: any numbers: `a, b`; if `a == b` then `b == a`
* let's write the actual implementation
    * remember `&` is an immutable reference
    * this compares to objects of the same type against ech other returning a `true` when equal and a `false` when unequal
* 1st impl for `PartialEq` & `Eq`
    ```rust
    impl PartialEq for Puzzle {
        fn eq(&self, other: &Self) -> bool {
            (self.num_pieces == other.num_pieces) && (self.name && other.name)
        }
    }

    impl Eq for Puzzle {} // marker trait
    ```
    * implementing `eq` allows us to use the struct as a key in a hashmap 
* 2nd impl for only `PartialEq`
    ```rust
    impl PartialEq for Puzzle {
        fn eq(&self, other: &Self) -> bool {
            (self.num_pieces == other.num_pieces) &&
            (self.name.to_lowercase() && other.name.to_lowercase())
        }
    }
    ```

#### From / Into
* they are an extremely useful pair (included in the standard prelude)
* if you implement `From`, then `Into` gets automatically implemented for you
    * so implement `From` to get both
* however, it's more ideomatic to use the `Into` in generic functions
* they describe the transformation between two types fro, different viewpoints
    ```rust
    From<T> for U
    Into<U> for T
    ```
* for our example, we want to have
    ```rust
    From<Puzzle> for String
    Into<String> for Puzzle    
    ```
* in this "conversion" for our example, we want to keep the name and lose the info on number of pieces
    ```rust
    impl From<Puzzle> for String {
        fn from(puzzle: Puzzle) -> Self {
            puzzle.name
        }
    }
    // use `From` like this
    let puzzle = Puzzle::default();
    let s = String::from(puzzle);
    // use `Into` like this
    let t: String = puzzle.into();
    ```
* usage in generic functions
    ```rust
    pub fn show<T: Into<String>>(s: T) {
        println!("{}", s.into());
    }
    ```
    * the input argument is of generic type `T`, which is quanlified as anything that can turn into a string `<T: Into<String>>`
* BUT the drawback of `From`/`Into` implemenation is that it consumes the puzzle
    * to avoid this we can use immutable references `&` and cloning
    * be careful with cloning though, because cloning large objects is expensive
    ```rust
    impl From<&Puzzle> for String {
        fn from(puzzle: &Puzzle) -> Self {
            puzzle.name.clone()
        }
    }
    // use in the generic show function
    let puzzle = Puzzle::default();
    show(&puzzle);
    ```

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