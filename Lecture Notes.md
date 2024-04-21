# 1. Introduction

## 1. Intro to the Course
### Learning
* Exploration are encouraged
* The fundamentals of Rust have a step learning curve
* But after that it gets easier
* The inlcuded exercises are designed to internalize the lessons

### Why Rust?
* Rust is a "systems programming language" with 3 key features
    1. Safety
    2. Concurrency
    3. Speed
* Other high-level languages like Python or Ruby other safety but not concurrency or speed
* Other systems languages like C or C++ give you speed but no enforced safety

### Rust Origin
* started by Mozilla employee Graydon Hoare
* Mozilla started to officially sponsor in 2009
* version 1.0 was released in 2015
* Firefiox was rewritten in Rust in 2017 and became twice as fasted

## 2. Exercises
### Companion Repo
* the exercises and other helpful materials from the course can be found at this repo: [ultimate_rust_crash_course](https://github.com/CleanCut/ultimate_rust_crash_course)
* clone or download the repo from the main branch and follow along

# 2. Fundamentals
## 1. Cargo
### The Build-System and Package-Manager
* while Python or JavaScript are well-known to have package-managers, systems language usually don't
* but Rust does and it's called `cargo`
* it can search, install, and update packages called `crates`
* `cargo` is also the build-system, the test-runner, the doc-generator and more

### Getting Started
* you can create a new project with 
    ```bash
    cargo new new_project
    ```
* the initial project structure is
    ```bash
    ./new_priject
    ├── Cargo.toml
    └── src
        └── main.rs
    ```
    * Rust files have a `.rs` ending
* `Cargo.toml` is the config file of the project
    ```toml
    [package]
    name = "new_project"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    ...
    ```
    * this is the authorative source of information for the compiler and other contributors
* compiling and running your project
    ```bash
    cargo run
    ```
    * this command will compile your project and then run it
    * `cargo` can tell if you made additions to your project (only new additions will be compiled)
    * the results will be stored in `new_project/target/debug`

## 2. Variables
### The Rust Language
* Rust uses curly braces `{}` and semi-colons `;` to delimit statements and scopes respectively
* the `let` keyword declares a variable
* Rust is a strongly typed language
    * as long as the compiler cna infer the type you don't need to annotate it
* `let` can destructure data and create mulitple variables

### Immutability
* by default all variables are _immutable_, i.e. un-change-able
* this is a safety-feature because immutable data can be shared between threads without locks
* immutable data can be stored on the stack making the program faster
* variables can be initialized as mutable with `mut` keyword

### Constants
* constants are even more immutable
* the are initialized by the `const` keyword and written in screaming snake-case (i.e. all upper letters with underscores)
* they must be type-annotated
* constants are inline to compile-time, they are really fast

## 3. Scope
### Variales have Scope
* variables have a scop, which is the place in the code where you are allowed to use them
* the scope of a variable 
    * begins at where it is created
    * extends to the end of the block (of curly braces)
    * or end when it is moved or consumed by a function
* there is no garbage collector!
    * variables are immediately dropped when they go out of scope

### Scope, Nesting and Shadowing
* variables can be shadowed, i.e. they can overlap in scope through nesting without affecting each other
    ```rust
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // prints 99
    }
    println!("{}", x); // prints 5
    ```
* variables can even be shadowed inside the same scope
    ```rust
    let mut x = 5;  // x is mutable
    let x = x // x is now immutable
    ```
* this is a cool trick because the compiler will optimize away this operation in assembly
* this enables different data representations in the code without slowing the program

## 3. Memory Safety
### Rust Guarantees
* Rust guarantees memory safety at compile-time
* the compiler will will not let code compile with unsafe or uninitialized data types
* this means only memory-safe code will compile
    * (there are exceptions when you explicitly use the unsafe macro!)
* your code and the compiler are subject to a contract that you will both uphold to make a program compile and run

## 4. Exercise A
* the exercise for this section is available [here](https://github.com/CleanCut/ultimate_rust_crash_course/tree/main/exercise/a_variables)
