# Rust tutorial

## Git init

Init a git hub repo

```bash
git init
git remote add origin
git add .  # After gitignore
git commit -m "First commit"
git push -u orgin master  # Upstream branche, after ssh-agent.
```

Git ignore file:

Ignore all exept directory and file with an extension.  
No need with cargo.

* gitignore file search pattern in all directories.
* `hello*`  # Any file or directory whose name begin with an hello.
* `/hello*`  # **Absolute path:** match `/hello.c` but not `/a/hello.c`
* `hello*/`  # Only for **directory**: match `/hello/` but not `/hello.c`
* `/hello/sir` = `doc/frotz`  # No need to begin with "/" if there is one in the middle.

```git
*  # all files and directories
!/**/  # a/**/b matches "a/b", "a/x/b", "a/x/y/b" directory, all dir are unban.
!*.*  # If there is a extension. 
target  # Ignore target folder.
```

SSH-Agent: To save password:

```bash
eval "$(ssh-agent -s)"
ssh-add 
```

## Hello World

### Rust instalation

```bash
sudo apt install cargo rust-src rustfmt
```

```bash
rustup update
rustup self uninstall
cargo --version
```

### The Rustc Way

```bash
mkdir projects
cd ./projects
mkdir hello_world
cd hello_world
```

```rust
// /main.rs
fn main() {
    println!("Hello, world!");
}
```

```bash
rustc main.rs
./main
```

### The Cargo Way

```bash
mkdir projects
cd ./projects
cargo new hello_cargo
```

Cargo do not generate git file if it is already on a github file

```bash
cargo check
```

```bash
cargo run
```

```bash
cargo build
./target/debug/hello_cargo
```

## Why rust

1. Quick, safe and modern
1. High level with low level
1. More secure:
    * Warning at compilation
    * Invalid memory canot be accessed with higger index in a array

## Real Example

Import Crates (Boite ou conteur à fruit)

```toml
# /Cargo.toml
[dependencies]
rand = "0.8.5"
```

It's algo generate a Cargo.lock which there is all the dependency and more used to compile the code. The dependency which are realy used, not whised like Cargo.toml

```rust
use rand::Rng; // Random, range.
use std::cmp::Ordering; // Compare.
use std::io; // Standard Input, Output

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Re assign a new variable to change it's type.
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /*
        Remember that parse returns a Result type
        Result is an enum that has the variants Ok and Err
        Ok value that contains the resultant number
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catchall value;
            // match all Err values
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // Comare guess with the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

* i for integer
* u for unsigned
* f for floating f32  or f64 bits
* & for reference
* `r#"myRawString"`

## Basic Concepts

* `let` to declare a variable
* `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
* Rustaceans the name of people who code in rust
* Shadowing when we re declare a variable to change it's type for example.  
Only work on a inner scope "{}"!
* `myVar_str`, `myVar_enum`
* We can't mutate the variable type with an assignement, we need to do shdowing for this
* We can write `1_000` for `1000`
* We can compile in a debug mode, to have more informations about exec
* Rust panic when it encounter integer overflow (more than 255)
* Modulo with `%`
* `''` for `char`acter and `""` for string
* Snake case convention: `my_fct`
* **statement:** are actions like assignation (not return value) `let x = 3`
* **expression:** evaluate to a value `2 + 3` or `8` are expressions
* Add a semicolon to the end of an expression turn it into a statement

### Compound Types

```rust
let myTuple: (u32, i8, f64) = (500, -20, 2.3);
let (x, y, z) = myTup;  // destructuring a tuple into three variables.
println!("The value of y is: {y}");
println!("The value of x is: {myTuple.0});
```

```rust
let myArr: [i32; 5] = [0, 1 , 2, 3, 4,  5];
let equalArr = [1; 3]  // ";" is like a repeat [1, 1, 1].
let first = myArr[0];
```

### Functions

```rust
fn main() {
    another_function(5);  // call a function before definie it.
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

```rust
fn plus_one(x: i32) -> i32 {
    x + 1  // never end with ';', because it's expression not a statement.
}
```

### Control flow

```rust
if number % 4 == 0 {
    ...;
} else if number % 3 {
    ...;
} else {
    ...;
}
```

Always return the same type in a if no mater the legs.

```rust
let a = [10, 20, 30, 40, 50];
for element in a {  //  No need to define element.
    println!("the value is: {element}");
}
```

## Owenership

**Owenership** is a set of rules on how manage the memory.

* Allow memory saftety (No compile)
* No garbage collector (When a variable is not use it's throw)
* Mange Heap data

Two concepts stack ('pile', push) and heap ('amas', allocate):

* **Stack:** The space is define
* **Heap:** Need to find an unkwnol variable space on the memory. Generate a pointer (memory location) to be more quick when we want to acces to the varible. Like a waiter in a diner.

```rust
//  A String is made up of three parts: 
// a pointer to the memory (Index and value)
// a length (the actual size of the string)
// and a capacity (The mawimum lenght tat the memoty can hold)
let s1 = String::from("hello");
let s2 = s1;  // s1 is pop to avoid double free error.
```

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // S1 is not poped, borowed by references.
}

fn calculate_length(s: &String) -> usize {  // ARCHitecture size (32 or 64).
    s.len()
}
```

**Slice** Continus sequence of elements. (".." for range). Create varibale from a part of an alreadydefined variable. So it's much suicker: no need to realy store the data again on the memory.

```rust
fn first_word(s: &String) -> &str {  // &str is a string slice
    let bytes = s.as_bytes();

    // An array of bytes (A = 64).
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();  // Error, we can't clean s because word (a reference) borrow it.
}
```

```rust
// s is a &str type because string literal are stortered direcltly on binary.
let s = "Hello, world!"; 
```

## Structure

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // To copy all the remain attributes.
};
```

```rust
struct Color(i32, i32, i32);  // Tuple struct: a struct without name attributes.
struct AlwaysEqual;  // Unit like struct.

fn main() {
    let black = Color(0, 0, 0)
    let subject = AlwaysEqual;
}
```

Debug in rust:

* **println!()**: Take references, output stream, print.
* **dbg!()**: Take ownership, error stream, print and return.

```rust
#[derive(Debug)]  // # is the outer attribute.
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);  // {:?} or {:#?} for debug print.
    dbg!(&rect1);  // Give ref to avoid ownership.
}
```

## Enum

Define a new type that is like a radio button list  
You can choose a value only a a resticted list

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

 Like struct we cann add method:

 ```rust
impl MyEnum {
    fn my_Method(&self) {
        // method body would be defined here
    }
}
```

There is a special enum for null value (invalid or missing):

```rust
enum Option<T> {
    None,
    Some(T),
}

//  Option is so common: no need to prefix with Option.   
let some_number = Some(5);
let absent_number: Option<i32> = None;  // Rust cano't infer None type.
```

## Match: a special if

 ```rust
 [derive(Debug)]
 enum Coin {
    Penny,
    Nickel,
    Dime(UsState),  // UsState is an another Enum.
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,  // An arm has two parts: a pattern and some code.
        Coin::Nickel => 5,
        Coin::Dime(Starter) => {  // Patterns That Bind to Values.
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
````

Also usefull with `Option<T>`:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
```

The default value:Define a real variable or `_ => ()`

```rust
match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  // other is the default, "_" to not use
    }
```

A Shortcut:

```rust
let config_max = Some(3u8);  // Value 3 but in a u8 format.

match config_max {
    Some(max) => println!("The maximum is {}", max),
    _ => (),
}

// Shortcut to exec the code "{}" with a variable affectation before.
if let Some(max) = config_max {
     println!("The maximum is {}", max);
}
```
 
 ## Crates

### Definitions

Crate: Is the main file that link to other file. Two forms:
* Binary (main)
* Library (sub fct)

Package: Contain as many binary crates as you like, but at most only one library crate

Recall to crate a packge:

```bash
$ cargo new my-project
$ cargo new restaurant --lib
```

The main file location by default:

```tree
src/main.rs
src/lib.rs
```

We can also create several binary create with a different name of the package

```tree
src/bin/my_bin.rs
```

In a crate we have module: there are like a section of codes  
In module we use path to identify item location  


To import (declare) a module code: `Mod myModule`  
If the importation is in a main file search in a src/myModule.rs  
If the declaration is inside a module, search in: src/myModule/mySub.rs

Iteam inside a module is private from it's parent module, access is deny   
Two things to know:
* Use `mod my_module {}` to create (define) a private module (item not accesable)
* Use `pub mod my_module  {}` to define a public module
* In a PUBLIC MODULE we can also use `pub enum` to make a ITEM PUBLIC too  
* Use `mod my_module;` to load a module for  private use (in crate module)
* Use `pub mod my_module;` to load a module with public use. So it's possible to access item in a external ways. (in sub module)



Don't need to write:
* `crate::garden::vegetables::Asparagus`
* `use crate::garden::vegetables::Asparagus`

`crate` id the default name of the main file (the current crate)  
Child module can access code in parent module but not reverse

### Public Vs private

In a restaurent:
* The front of the house is where the clients enter
* Back is the kitchen: where you prepare dishes to serve the client

 