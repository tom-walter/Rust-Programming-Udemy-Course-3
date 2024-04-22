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

## 5. Functions
### Functions
* so far we have seen the `main` functions which is the entry point for any executable
* functions in general are defined by the `fn` keyword and should written in snake-case (i.e. lower letters with underscores)
* functions don't have to be defined before they are called (it's not a scripting language)

### Parameters
* function parameters are always defined with `name: type`
    * there so no default in the function signature (like Python allows)
* a return type must also be defined, if a function returns a  value
* Rust does not support name arguments, when calling the function 

### Return Values
* the `return` keyword can be used to return a value explicitly
* or omitting the semi-colon `;` will return a value implicitly (called a tail-expression)

### Macros
* macros are magic functions that you recognize by the `!` at the end of function
* they are a special type of function that often generate code as needed under the hood

## 6. Exercise B

## 7. Module System
### Modular Structure
* Rust can compile to executables or libraries
    ```bash
    ./hello
    ├── Cargo.toml
    └── src
        ├── lib.rs  // the hello library
        └── main.rs // the hello binary
    ```
* you can also use this to build modules for your applications
    ```rust
    # src/lib.rs
    pub fn greet() {
        println!("Hi!");
    }
    ...
    # src/main.rs
    use hello::greet; // item import

    fn main() {
        hello::greet(); // absolute path
        greet();        // relative path avail through import
    }
    ```

### Importing
* by default, all functions are private in Rust (even in the same project)
* the `pub` keyword can make functions available outside of their original file
* then you can use it either through the absolute path or relative path (see above)
* we can import functions from standard libraries or our own modules with `use` keyword
* the standard library is called with `use std::`

### Crates.io
* crates.io is Rust official package registry
* once you found a necessary packages, you can add in the dependencies section of the `Cargo.toml`
    ```toml
    [dependencies]
    rand = "0.6.5"
    ```
* this also allows you to import/`use` this package inside your code 

# 3. Primitive Types and Flow Contol

## 1. Scalar Types
### The 4 Scalar Types
1. Integers
2. Floats
3. Booleans
4. Characters

### Integers
* unsinged integers are always positive
* singed integers can be positive and negative
* `usize` represents the machine's pointer type and represent every memory address
* `isize`'s maximum value is the upper bound of object and array size
* un-typed integers default to `i32`

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Number Literals
* numbers of different numerical systems are represented as shown in the next table
* `u8` stands for byte in Rust
* underscores are ingored by the compiler and help with readibility 

| Number literals | Example     | Starts with
|-----------------|-------------|------|
| Decimal         | 98_222      |      |
| Hex             | 0xff        | `0x` |
| Octal           | 0o77        | `0o` |
| Binary          | 0b1111_0000 | `0b` |
| Byte (u8 only)  | b'A'        | `b`  |

### Floats
* there are `f32` and `f64`
* `f64` is the default but can be slow on small machines
* they follow the IEEE-754 standard

### Suffixing
* you can suffix numbers with a type to turn them into literals
    ```rust
    let x = 5u16;
    let y = 3.14f32;
    ```

### Boolean
* the type is specified with `bool` and its two cases are `true` & `false`
* they are not integers and don't support arithmetic (unless you cast them)

### Character
* `char` can be anything from
    * a character of our alphabet
    * to a character of someone else's alphabet
    * to an ideograph
    * or a diacritic or an emoji
    * or a non-printable Unicode control character
* they are always 4 bytes (32 bits)
* USC-4 or UTF-32 string

## 2. Compound Types
### Collections
* compound or collection types carry multiple values of other types into one

### Tuples
* can hold multiple of any type
* there are 2 ways of accessing its members
    * 1: dot syntax with zero-indexed
    * 2: de-structuring
    ```rust
    let info: (u8, f64, i32) = (1, 3.3, 999)
    let jets = info.0; // dot syntax
    let (jets, fuel, ammo) = info; // de-structure
    ```
* Rust tuples have an arity of 12 (i.e. how many values are collected)

### Arrays
* arrays store mulitple of the same type
* can be specified literally with square brackets `[]` and commas or with how many values you want
* accessing is done by index in square brackets
* have an arity of 32 because arrays live on the stack (fast memory) be default are of fixed size
    ```rust
    let buf: [u8; 3] = [1, 2, 3];
    let first = buf[0];
    ```

### Vectors
* vectors are of flexible size and therefore live on the heap (dynamic, but slower memory)
* they are usually the prefered way of handling your list-like data collection

## 3. Exercise C

## 4. Control Flow
### If-Else Expressions
* everything between the `if` and `{` is the condition
* Rust does not like type conversion, so the condition must evaluate to a boolean
* chaining is done by `else if`

### Expression not a Statement
* statements don't return values but expresions do!
* `if` is an expression, because `if-else` can return a value
    ```rust
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    ```
* this implicit return works via tail-expression
* Rust also does not support ternary operations

### Uncoditional Loop
* Rust can do unconditional loops, which allow for optimization at compile time
* but they are not truly infinite, they end with a break
    ```rust
    loop {
        break;
    }
    ```
* also nested, unconditional loops are supported
* label each loop and use the label to break oute
    * such labels must start with a single apostrophe
    ```rust
    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }
    ```

### While Loop
* have the same behavior as unconditional loops, but they break when their condition evaluates to false
* again Rust refuses types coercion to booleans

### For Loop
* the for loop iterates over any iterable value, compound types or collection types
* usually, these types have the `.iter()` method
* for functional programming, methods like `.map()` can be chained onto `.iter()`, which is in Rust code

### Ranges
* you can also iterate over ranges, denoted by two dots `..`
* the start is inclusive and the end is exclusive
* you can make the end inclusive with `..=`
    * but this is not recommended because the LLVM compiler cannot optimize this (recommend for optimization is +1) 

## 5. Strings
### Strings are Complicated
* there are at least 6 types of strings in the Rust standard library
* but we mostly care for 2 types: `String` and `str` (called string slice)

### str
* `str` is most often encountered as borrowed `&str`
* a literal string is always a string slice
    ```rust
    let msg = "hello 🌎"; // is string slice
    ```
* `&str` are immutable

### String
* `String` can be modified 
* you can create a `String` from a `str` by called `.to_string()` method on a borrowed slice
* or by passing to `String::from()`
    ```rust
    let msg = String::new("hello 🌎"); // is String
    ```

### String vs str
* a borrowed string slice `&str` is made of pointer to some bytes and a length
* a string `String` is made of a pointer to some bytes, a lenght, and a capacity
* thus, a borrowed string slice `&str` is subset of a string `String` in more than one sense
* both are valid UTF-8 types by definition (i.e. by compiler enforcement and run-time checks)
* they cannot be indexed by character position because of unicode characters
    * they differentiate between graphenes and diacritics
    * they can combine into character and thus have different lengths
* a stings supports the method `.bytes()` which can be indexed
* a stings supports the method `.chars()` which can be used to iterate through unicode scalars
* or use library like `unicode-segmentation`

## 6. Exercise D