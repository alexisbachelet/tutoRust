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
* statement: are actions like assignation (not return value) `let x = 3`
* expression: evaluate to a value `2 + 3` or `8` are statements
* Add a semicolon to the end of an expression, you turn it into a statement

### Compund

```rust
let myTuple: (u32, i8, f64) = (500, -20, 2.3);
let (x, y, z) = tup;  // destructuring a tuple into three variables.
println!("The value of y is: {y}");
println!("The value of x is: {myTuple.0});
```

```rust
let myArr: [i32; 5] = [0, 1 , 2, 3, 4,  5];
let equalArr = [1; 3]  // ";" is like a repeat [1, 1, 1].
let first = myArr[0];
```

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
