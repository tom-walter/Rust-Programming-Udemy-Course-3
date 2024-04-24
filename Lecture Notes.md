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

# 4. The Heart of Rust
## 1. Ownership
### What is Ownership?
* ownership is a feature that makes Rust different from other systems language
* ownership makes memory-safety and informative compiler messages possible

### Three Golden Rules
__1. Each Values has an Owner__
* there is no value/data in memory that doesn't have a variable that owns it

__2. Only one Owner__
* no variables may share ownership
* other variables may borrow (or move the ownership)

__3. Dropped Values go out of Scope__
* when the owner goes out of scope, the values gets dropped from memory immediatly

### Computer Memory
* during runtime, programs have access to 2 kinds of memory: the stack & the heap

    | Feature    | STACK      | HEAP                      |
    |-----------:|------------|---------------------------|
    | Allocation | In Order   | Unordered                 |
    | Size       | Fixed Size | Variable / Dynamical Size |
    | Ordering   | LIFO       | Unordered                 |
    | Speed      | Fast       | Slow                      |

* LIFO = last in, first out

### Ownership in Action
_MOVE_
* moving ownership from one to another variables
    ```rust
    let s1 = String::from("abc");
    let s2 = s1;
    println!("{}", s1); // error[E0382]
    ```
    * compiler error "borrow after moved value"
* a `String` exists as a pointer on the stack and as data on the heap
* moving it creates a new stack pointer but no heap data
* to have memory-safety, the first pointer on the stack is deleted so that no 2 owners of the heap data exist


_CLONE_
* cloning a value
    ```rust
    let s1 = String::from("abc");
    let s2 = s1.clone();
    println!("{}", s1);
    ```
* this clones the pointer on the stack as well as the data on the stack
* therefore, 2 pointers with identical but separate data can exist without creating memory-unsafety
* in other languages, this is a _deep copy_

_COPY_
* Rust reserves copy for when only stack variable (without heap data) is duplicated

_DROP_
* a value is dropped when a variable goes out of scope
* at this point three things happen
    * 1: a destructor deletes the variable
    * 2: the heap memory is freed
    * 3: the stack pointer is popped
* thus, there will be neither leaks nor  dangling pointers

_SCOPE_
* obviously, functions have their own scope
* meaning: if variable is moved into a function, it is dropped after the scope of the function
    ```rust
    fn do_stuff(s: String) {
        // does stuff
    }

    let s1 = String::from("abc");
    do_stuff(s1);
    println!("{}", s1); // error
    ```
* if you don't want the function to consume the variable, we can use referencing

## 2. References & Borrowing
### References on Function Parameters
* the ampersand `&` before a type in the function's signature indicates a refernce to a type
* when we call such a function, we must pass the variable as reference (and the variable retains ownership of the value)
* only the reference gets moved into function and afterwards only the reference goes out of scope
    ```rust
    fn do_stuff(s: &String) {
        // does stuff
    }

    let s1 = String::from("abc"):
    do_stuff(&s1);

    println!("{}", s1); // no error
    ```
* references are pointers to the pointers in the stack

### Lifetimes
* references must always be kept valid through lifetimes
* the compiler will not let you create a reference that outlives the data (that it is referencing)
* you can never point to null
* references are by default immutable (even if the data is mutable)
* we must create a mutable reference to mutable data in order to change the data
    ```rust
    fn do_stuff(s: &mut String) {
        // auto deref
        s.insert_str(0, "Hi ");
        // manual deref
        (*s).insert_str(0, "Hi ");
    }

    let mut s1 = String::from("abc"):
    do_stuff(&s1);
    ```
* dot-methods automatically dereference
* we can also dereference with an asteriks before the reference `*`

### Summary
* variables and references
    ```
    x       ← variable/owner
    &x      ← reference to x
    &mut x  ← mutable reference to x
    ```
* types and references
    ```
    i32      ← a type
    &i32     ← reference to a type
    &mut i32 ← mutable reference to a type
    ```
* dereferencing
    ```
    # if you have 
    x: &mut i32
    # deref with
    *x // a mutable i32
    ...
    # if you have 
    x: &i32
    # deref with
    *x // an immutable i32
    ```

### Reference Counting
* Rust has the following rule
* you can have
    * either exactly _one mutable reference_
    * or _any number of immutable reference
* this makes referencing thread-safe by default
* but Rust additionally provides reference counters `Rc` and `Arc` to handle them better across threads

### Compiler as ...
_Rule-Enforcer_
* all memory-safety rules are enforced at compile-time, which may lead to many errors
* but then, there will be no runtime errors, like segfault

_Helper_
* compiler error are very informative and mostly tell you how to fix them
* if there is only one obvious solution, you can use `cargo fix` to fix it


## 3. Exercise E

# 5. The Meat of Rust
## 1. Structs
### Struct (better than Classes)
* `struct`s can have
    * data fields
    * methods
    * associated functions
* they are created with the `struct` keyword
    ```rust
    struct RedFox {
        enemy: bool,
        life: u8,
    }
    ```
* they are instantiated like
    ```rust
    let fox = RedFox {
        enemy: true,
        life: 70,
    };
    ```

### Implementation Block
* but you can create associated functions, like a constructor 
    ```rust
    impl RedFox {
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }
    };
    ```
* this is the implemenation block started by `impl`
* it creates associated function and methods for the `struct`
* an associated function does not have a form of `self` as its 1st parameter
* now, we can instantiate like this
    ```rust
    let fox = RedFox::new();
    ```
* the double colon `::` is the scope operator and it's used to acces s part of namespace-like things

### Methods & Associated Functions
* methods and associated functions are both created inside the implementation block
    ```rust
    impl RedFox {
        // associated functions
        fn new()
        fn function()
        // methods
        fn move(self)
        fn borrow(&self)
        fn mut_borrow(&mut self)
    }
    ```
* methods always take some form of `self` as its 1st parameter 

### Does Rust have inheritance?
* question if Rust is OOP, does not matter to the Rust community
* classes and inheritance bring abstraction costs that Rust wants to avoid 
* so, no classes, no inheritance, right?
* inheritance problems are solved by traits `trait`

## 2. Traits
### Traits are like Interfaces
* Rust takes the composition over inheritance
* they are created with `trait` keyword
* traits define required behavior
* this behavior (aka functions) must be implemented for the struct, if the struct should have this trait
    ```rust
    struct RedFox {
        enemy: bool,
        life: u8,
    }

    trait Noisy {
        fn get_noise(&self) -> &str;
    }

    impl Noisy for RedFox {
        fn get_noise(&self) -> &str { "meow?" }
    }
    ```

### Why not implement without trait?
* once we have a trait shared across multiple different struct, wen can start writing generic functions that accept any value, which has this trait
    ```rust
    fn print_noise<T: Noisy>(item: T) {
        println!("{}", item.get_noise());
    } 
    ```
* this function takes any item of type `T` (generic) which implements the noisy `trait`
* in Rust, you can implement a trait for any struct, making structs (even ones that you didn't create) extremly flexible
    * e.g.
    ```rust
    impl Noisy for u8 {
        fn get_noise(&self) -> &str { "Byte!" }
    }

    fn main() {
        print_noise(5_u8); // prins "Byte!"
    }
    ```

### Game Example
* assume we have a game with a goose, a pegasus, and a horse
    * they share attributes as shown in the table

    | trait   | Goose | Pegasus | Horse |
    |--------:|:-----:|:-------:|:-----:|
    | fly     | ☑️   | ☑️     | ❎    |
    | ride    | ❎   | ☑️     | ☑️    |
    | explode | ☑️   | ❎     | ☑️    |

* if we implement traits rather than object inheritance, we can keep the attributes well separated

### Trait Inheritance
* traits themselves can inherit from another trait, e.g. 
    ```bash
    Movement
    └── Run
        ├── Ride
        └── Fly

    Damage
    └── Explode
    ```
* now Horse would need to implement the following traits
    ```bash
    Horse
    ├── Movement
    │   └── Run
    │       └── Ride
    └── Damage
        └── Explode
    ```
* all trait inheritance means is that for every trait you have to implement the parent 

### Trait Default Behavior
* if you want to implement default trait behavior, you can do so inside your trait definition
* when you implement the trait on a struct, you don't need to provide a new definition, just use an empty blocl
    ```rust
    trait Run {
        fn run(&self) {
            println!("I'm running!");
        }
    }

    struct Robot
    // no overriding of default behavior
    impl Run for Robot {}

    fn main() {
        let robot = Robot {};
        robot.run();
    }
    ```

## 3. Exercise F
## 4. Collections
* `std::collections` is a standard library with the common data structures known from other languages

### Vector
* a vector `Vec<T>` is a generic collection that holds a bunch of one type
* similar to a list or array in other languages
* it's the most commonly used collection
* when you create a empty vector, it must be typed, afterwards you can push values of that type onto the vector
    ```rust
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    ...
    let last = v.pop(); // last is 6
    ```
* vectors act like stack
    * `.push()` appends things to the end
    * `.pop()` removes and returns the last item
* vectors objects of known size next to each other on memory, so you can index into it
    * if an index is out of bound, Rust will panic 
* vectors can be created with a macro
    ```rust
    let mut v = vec![2, 4, 6];
    ```
* vectors provided low-level control with methods like: `.iter()`, `.insert()`, `.split()`, `.remove()`, `splice()`, `.sort()`

### HashMap
* a hashmap is generic collection, where you store key-value pairs
* you need to specify type for the key and one for the value
* in some languages, this is called a dictionary
* their purpose is to be able to insert, look up, and remove values by key in constant time $O(1)$
    ```rust
    // create
    let mut h: HashMap<u8, bool> = HashMap::new();
    // insert
    h.insert(5, true);
    h.insert(6, false);
    // remove + return
    let have_five = h.remove(&5).unwrap();
    ```

### More Collections
* `VecDeque`:
    * uses a ring buffer to implement a double-ended queue
    * it can efficiently remove items from front and back
* `LinkedList`
    * each element points to the previous element in the list
    * is quick at adding or removing items at an arbitrary point in th list (but slow at anthing else)
* `HashSet`
    * is an implementation of a mathematical set that implements set-operation efficiently
* `BinaryHeap`
    * is like a priorty queue which always pops off the max value 
* `BTreeMap` & `BTreeSet`
    * are alternative map and set implemenations using a modified binary tree
    * use them if you need the map-keys or set-values to always be sorted

## 5. Enums
### Algebraic Data Types
* Rust `enum`s are more like algebraic data types in Haskell than C-like enums
* you can specify them with the `enum` keyword, the name in capital camel-case, and the variants in a block `{}`
    ```rust
    enum Color {
        Red,
        Green,
        Blue,
    }
    ```

### Powerful Enums
* real power of Rust `enum`s comes form associating data and methods with variants
* a variant can have no data, a single type of data, a tuple of data, or an anonymous struct of data
* even better, you can implement functions and methods for an enum
    ```rust
    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place {x: i32, y: i32},
    }

    impl DispenserItem {
        fn display(&self) { }
    }
    ```
* you can also use options with an enum
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```

### Working with Enums
* because enums can represent all sorts of data, you need to use patterms to examine them

_Single Variant_
* `if let` takes a patterm amd will match one variant of the enum
* if the pattern does match, then the condition is true and the variables inside the pattern are created for the scope of `if let` block
    ```rust
    if let Some(x) = my_variable {
        println!("value is {}", x);
    }
    ```

_All Variants at Once_
* if we want to check all variants, we can use Rust's powerful `match` expression
    ```rust
    match my_variable {
        Some(x) => { println!("value is {}", x); },
        None => { println!("no value"); },
    }
    ```
* match arms require a branch for every possible outcome, i.e. _they must be exhaustive_
* all branch arms must return the same type
* exhaustive matching is enforced by the compiler and makes your code better

### Option
* options are very common in Rust
* option is used whenever something can be absent
    ```rust
    // definition
    enum Option<T> {
        Some(T),
        None,
    }
    ```
* create an optional integer like
    ```rust
    // with typing
    let mut x: Option<i32> = None;
    // with type-inference
    let mut x = None;
    x = Some(5);
    x.is_some(); // true
    x.is_none(); // false
    ```

### Result
_Definition_
* the result type is Rusts way of making error handling easier and more explicit
    ```rust
    // definition
    #[must_use]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
* the `Ok(T)` type and `Err(E)` type are independent of one another
* `#[must_use]` means that you have to handle possible error and make a conscious choice on what to do

_Usage_
* importing/reading a file will return a result-type rather than the contents directly
* this is because reading a file may fail
* without error handling
    ```rust
    use std::fs::File;

    fn main () {
        File::open("foo.txt"); // compiler warning: Result must be used
    }
    ```
* with error handling
    ```rust
    use std::fs::File;

    fn main () {
        let res = File::open("foo.txt");
        // unwraps with Ok result
        // an error still crashes
        let file = res.unwrap();
        // expect method
        let file = res.expect("error message");
        // never crash
        if res.is_ok() {
            let file = res.unwrap();
        }
        // full pattern matching
        let file = match res {
            Ok(f) => { /* do stuff*/ },
            Err(e) => { /* do stuff*/ },
        }
    }
    ```

## 6. Exercise G
# 6. Closures & Threads
## 1. Closures
### What are Closures?
* you'll encounter closures when you work with
    * threads
    * functional programming
    * iterators
    * standard libraries
* a closure is an anonymous function that can borrow or capture data from the scope it is nested in
    * in other languages, this is called a lambda function
* the syntax is a parameter list within two pipes without type annotation followed by a block
    ```rust
    |a, b| {a + b}
    ```
* the types and return values are all infered from arguments
* you can assigned closures to a variable
    ```rust
    let add = |a, b| {a + b};
    let c = add(1, 2); // c=3 
    ```

### CLosures and Scopes 
* a closure will borrow a reference to values in the enclosing scope
    ```rust
    let s = ":)".to_string(); 
    let f = || {println!("{}", s)};
    let f();
    ```
* you don't need to specify arguments that you borrow from the enclosing scope
* but this can lead to trouble, if the closure out lives the arguments it calls
* closures support the `move` semantic so we can force the closure to move the used variable (and take ownership of it)
* now, the variable will go out of scope when the closures is dropped

### Functional Programming
* funtional programming makes use of higher functions
* higher order functions take other functions as arguments
* this is were closures come in handy
* Rust supports method chaining
    ```
    let mut c = vec![2, 4, 6];

    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x|, acc + x);
    ```

## 2. Threads
### Threads  in Rust
* Rust threading is portable and compiles across different systems: MacOS, Windows, Linux, etc.
    ```rust
    use std::thread;

    fn main() {
        let handle = thread::spawn{move ||
            // do stuff in child thread
        };

        // do stuff simultaneously in main thread

        // wait until all threads exited
        handle.join().unwrap();
    }
    ```
* 1st: we pull the standard library for threading into scope
* 2nd: we spawn a thread with an emtpy, movable closure
    * this closure is executed as the main function of the thread
    * being empty allows us to do anything 
* 3rd: we spawn child threads that do the actual tasks
* 4th: we join all threads into the main one
    * this pauses the main thread until all child threads are done executing
    * the join will return a Result type

### Threading and Resource
* threading is heavy-weighted
* creating a thread allocated an operating-system-dependent amount of RAM for the threads own stack (a couple of megabytes)
* whenever a CPU switches from running one thread to another, it has to do an expensive context switch
* more threads = more CPU overhead
* but threads are fantastic when you need to run tasks concurrently 
* they allow you to use multiple cores simultaneously 
* tasks that require disk I/O or network I/O should implemented with asynchronous function rather than threading

## 3. Exercise H
### Commnunicating between Threads
* Rust allows you to transfer data between threads
* of course, this transfer needs to follow Rust strict safety rules
* we'll use the external crate `crossbeam` rather than the standard library
    * crossbeam has more features and higher performance
* to communicate between threads, we need a channel
* creating a channel always creates an sending side and receiving side
    * senders are called by convention `tx`
    * receivers are called by convention `rx`
* cloning a sender `tx` means multiple threads can send to one receiver `rx`
    * e.g. when you want child threads to update the main thread
    * closing all the sender sides will close the receiver side
    ```rust
    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100);

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();
    ```
