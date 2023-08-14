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
cargo new my-project
cargo new restaurant --lib
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
If the importation is in a main file search in a `src/myModule.rs`  
If the declaration (importation) is inside a module, search in: `src/myModule/mySub.rs`

Item inside a module is private from it's parent module: access is deny  
Two things to know:

* Use `mod my_module {}` to create (define) a private module (item not accesable)
* Use `pub mod my_module  {}` to define a public module
* In a PUBLIC MODULE we can also use `pub myItem` to make a ITEM PUBLIC too  
* Use `mod my_module;` to load a module for  private use (in crate module)
* Use `pub mod my_module;` to load a module with public use. So it's possible to access item in a external ways (in sub module)

Don't need to write:

* `crate::garden::vegetables::Asparagus`
* `use crate::garden::vegetables::Asparagus`

`crate` id the default name of the main file (the current crate)  
Child module can access code in parent module but not reverse

### Public Vs private

In a restaurent:

* The front of the house is where the clients enter
* Back is the kitchen: where you prepare dishes to serve the client

## Common Collection

Recall:

```rust
let myArray = [1, 2, 3];  // Same type, fixed size (Can be mutable).
let myTuple = (1, 2.1, "3")  // Diff type, fixed size.
```

### Vector

#### Creation

```rust
let v: Vec<i32> = Vec::new();  // Type spec because rust connot infer.
let v = vec![1, 2, 3];  // Same type, varible size.
```

#### Add

```rust
v.push(5);  // Only for mutable vector.
```

#### Get

```rust
let i: &i32 = &v[2];  // Crash if the index 2 is not created (OOB).
let i: Option<i32> = v.get(2);  // If OOB then return None
```

#### Loop

Rust store vector's value on the heap and next to each other. If there is not anymore sufficently space in memory. Rust move the vector to get a bigger space. So we cannot borow a value and push in the same type.

```rust
let mut v = vec![1, 2, 3];
let i = &v[0];
v.push(4);  // BUG !!!
```

A reference is a pointer (adress memory) that we can follow to get value but not write on it. To change a pointer value we need to make a dereference:

```rust
let mut v = vec![1, 2, 3];
for i in &v {
    *i += 1;
}
```

**Little ticks:** A vector can grow up in size but only store on type. To have multiple type we need to make a vector of enum.

### String

Rust only have one type of string in the core language the **string slice** `str`.
Slices are continus sequences in memory.

#### String-Creation

In rust `String` are UTF-8 encoded so each letter have 2 or 4 bytes memory.

```rust
let myStr = "hello";  // String literal are directly stored in memory in str type.

let myString = String::new();
let myString = "hello".to_string();
let mySring = String::from("hello");
```

#### String-Add

```rust
myString.push_str("hello");  // Push Borrow "hello".
 
s1 + &s2  // Take ownership of s1 and always use a reference for s2. Like add.
format!("{s1}-{s2}-{s3}");  // Use references, concatenate string but not print.
```

#### String-Loop

```rust
for c in s.chars() {};
```

### Hash map

* **Hash** to protect the data
* **map** like Python's dictionary

#### Hash-Creation

**let** stand for *"alouer"* in french.

```rust
use std::collections::HashMap;  // HasMap rarely used so we need to import them.
let mut scores = HashMap::new();
```

#### Gash-Get

Explenation:

1. `get()` return `Option<&V>`
1. `copied()` return `Option<V>`
1. `unwrap_or()` return `V`. The digit is always on the right near of the parentesis and is it the else method.

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

#### Hash-Add

Preserve or owerwite.

```rust
scores.insert("blue", 10)  // Overwrite if the key already exist. 
scores.entry("blue").or_insert(50)  // Insert only if the key not already exist.
```

`or_insert()` return a `&mut` so we nned to make a dereference to change the pointer value.

```rust
let count = map.entry("blue").or_insert(0);
*count += 1;
```

#### Hash-Loop

```rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

## Error Handling

Two type of error in rust:

* Recoverable: can be fixed by the user (e.g. file not found). Type `Result<T, E>`
* Unrecoverable: cannot be fixed (e.g. OOB Array). Panic macro `panic!`

### Unrecoverable

Panic exit the code and clean up the memory but we can conserve it with:

```toml
[profile.release]
panic = 'abort'
```

Panic also create a backtrace (all functions called by the program). To see the bavktrace:

```bash
RUST_BACKTRACE=1 cargo run
```

### Recoverable Error

```rust
enum Result<T, E> {
    Ok(T),  // Type error in case of success.
    Err(E),  // Error type in case of fail.
}
```

Can be managed by `match`:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,  // return the inner file value out of the Ok variant.
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

We can also have multiple error management. Here if the file not exist we create it.  
The code `File::create("hello.txt")` try to create a file and return an enum of success just like the open ways:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,  // File Creation.
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

Shortcut to just **unwrap** the OK case and **panic** in error case:

```rust
let greeting_file = File::open("hello.txt").unwrap();
let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
```

We can also exec code instead of panic with `unwrap_or_else()`:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap()
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

Just like unwrap but not panic in case of error (`?`):

* In case of OK(T) unwrap T and **continuing** the main function execution (whose called the ? operator)
* In case of Err(E) return Err(E) and **early return** of the main fct

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // So if open() failed the '?' make "read_username_from_file" return Err().
    // We even not exec the "read_to_string".
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

The `?` work also on `Option<T>`

But the best ways to extract the file content in variable is:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

`from` is the Rust way to convert one type to another type

### Advice in error management

* `Panic` in case of tests
* `Result` in case of wrong user input
* Create it's own type to make check at the object creation. So we don't need to make the same check all the time when we used a standard type instead of it's own type

## More Generic

### Generic Data Type

```rust
// For a function.
let myArray = [1, 2, 3];
let myArray: [i32, 3] = [1, 2, 3];
fn larget_i32(list: &[i2]) -> &i32 {};
fn larget<T>(list: &[T]) -> &T {};
```

```rust
// For a struct.
struct Point<T> { x: T, y: T, }
let i = Point( x: 1, y: 2 );

struct Point<T, U> { x: T, y: U, }  // U is just after T.
let i = Point( x: 1, y: 2.1);
```

Double `<T>`: the first is for generic type, the second is to point on the right
struct:

```rust
// For method : Function on struct.
struct Point<T> { x: T, y: T, }

impl Point<f32> {  // This Method only work on Point<f32> type.
    fn x1(&self) -> f32 { ... }
}

impl<T> Point<T> {  // Double <T>, to def method that work on all Point type.
    fn x2(&self) -> &T { self.x }
}
```

```rust
struct Point<T> { x: T}

impl<T> Point<T> {
    // To use generic param inside a generic method.
    fn copy<Y> (&self, other: Point<Y>) -> Point(Y) {...}
}
```

### Trait

#### Basic definitions

To share functionality across many type (struc)

To define a trait specific to each struct:

```rust
// We are in "Agregator" file (library) so pub to be used in main file.
pub trait Summary {
    fn summarize(&self) -> String;  // end with ";" because it's just signature.
}

pub struct Article { pub title: String, pub content: String }
pub struct Tweet { pub user: String, pub content: String }

impl Summary for Article {  // impl myTrait for myStruct
    fn summarize(&self) -> String {  // Always the same signature as the trait.
        format!("{}, by {}", self.title, self.content)
    }  
}
```

But we can be more generic with generic trait. No need to specify a function at each time we use the trait on a new type. By default all struct have the same method.

```rust
pub trait Summary {
    fn summarise(&self) -> String { ... }  // Here is function definition.
}
```

To use a trait in a main file:

```rust
// aggregator is the library where the trait is.
// And we import the trait and it's struct.
use aggregator::{Summary, Article};

fn main() {
    let article = Tweet { ... };
    println!("Article summary: {}", article.summarize());
}
```

#### Cool tips

To have generix trait that work on all spficities:

* We can implement a generic trait with an specific mehod:  

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

* So we can def the specific method:

```rust
impl Summary for Article {
    fn summarize_author(&self) -> String { self.author }
}
```

* So we can call the huge generic method on any specific type.

#### Trait use restriction

We can implement a trait on a type: only if **trait** and/or **type** are defined in the local trait.

So if we are in "aggregator" library (crate) we can link to our local struct "Article" on generic trait like "Display"

#### Restiction on genereic type

```rust
// item can be any type but it need to implement the Summary trait.
pub fn notify(item: &impl Summary) { ... }
pub fn notify<T: Summary>(item: &T) { ... }  // Also work but it's less verbose.
```

```rust
pub fn notify<T: Summary + Display>(item: &T) { ... }  // Double restriction.

pub fn notify<T, U>(t: &T, u: &U) -> i32  // Double restrict more verbose.
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

Define method to all MyStruct type that have the DIplay trait.

```rust
impl<T: Display> MyStruct<T> { 
    fn cmp_display(&self) { ... }
}
```

We can conditionally implement a trait for any type that implements another trait:

```rust
// Implemente trait ToString for all type that have DIsplay trait.
impl<T: Display> ToString for T { ... } 
```

### Lifetime

Lifetime ensure that **reference** are **valid** as long as we need them to be alive.  
Every reference has a lifetime and in magor case Rust can **infer** it.

To avoid a dangling reference, the rust compiler have a borrow checker:

1. **Lifetime:** the time period between when my variable is init and disepear.
1. The borrow checker compare the lifetime of the real data and it's references. The data must have a longer lifetime than it's references.

```rust
fn main() {
    let x =5;              // -----------+  'b is data and 'a the ref.
                           //            |   
    let r = &x;            // --+-- 'a   |
                           //   |        |
    println!("r: {}", r);  //   |        |
}                          // --+--------+
```

Rust can't infer in this case:

```rust
// Rust can't infer is the returned ref is from `x` or `y`.
fn longest(x: &str, y: &str) -> &str {...}
```

So we need to help the borrow checker by given an explicit **lifetime annotations** to the parameters.  
**Lifetime Annotations** are not **Lifetime**!  
**Lifetime Annotations** are here to describes the **relationships** of multiples **lifetime's references** to each others. They don't change the lifetime of references, it's just relationships: a minimum lifetime shared by params.

```rust
// Here 'a is the shortest lifetime of a references between `x` and `y`.
// The return value have so a lifetime equal to 'a (the shortest).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {...}
```

```rust
// 'a is the shorstest lifetime bettwen `x` and `y`.
// But it's also the real lifetime of the return value.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Even if the return value is a reference of string1 the longest string.
// The lifetime of the return value is equal to the shortest lifetime.
// So here the return ref of string1 have the lifetime of string2.
// So we canno't use the `res` value after the end of string2.
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str());
        // println!("The longest string is {}", res);  // Here is OK !
    }
    println!("The longest string is {}", res);  // Here it's work, it's BUG !
}
```

We can define a struc to hold references (instead of owned type):

```rust
// We need to add a lifetime annotation on every struct's reference. 
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// To implemente a method we need to be generic on the imp part.
// because those lifetimes are part of the struct’s type
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

#### Lifetime Elusion (ominision)

Three rule to infer lifetime time annotation to a function:

1. Each parameter have by default there own lifetime annotation `fn foo<'a, 'b>`
1. If there is only one input lifetime: all the output have the same lifetime
1. If there is a `&self` input lifetime: all the output have the same lifetime of `&self`

#### Static lifetime

A Staticsreference is valid for all the program duration (stored in binary)

```rust
// In Rust all string literal are static references. So no disepear 
let s: &'static str = "my_str";
```

## Automated Test

By default rust create a test module for each new library:

```bash
cargo new adder --lib  # My Library.
```

```rust
// src/lib.rs
// By default like an "Hello World!"
#[cfg(test)]
mod tests {

    #[test]  // Attributes are metadata about pieces of Rust code (like derive).
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

}
```

```bash
cargo test  # To run test.
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50, };
    println!("rect is {:?}", rect);  // To quickly print all struct's attributes.
}

#[cfg(test)]
mod tests {
    use super::*;  // Relative path to the current module here: lib.rs

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

So we have in main test function:

* `assert!(add_two(2) == 4)`: for bollean
* `assert_eq!(4, add_two(2))`: the left and right side doesn't matter in Rust.
* `assert_ne!()`: not equal
* `#[should_panic]`: to exec test function. It's pass if the function panik, fail if not.

To use `assert_eq` and `assert_ne` we need to implement the `PartialEq` and `Debug` trait to our custom enum and struct. `#[derive(PartialEq, Debug)]`

We can add custom error message:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

```rust
#[test]
#[should_panic]  // #[should_panic(expected = "substring: beetween 1 and 100")] 
fn greater_than_100() { Guess::new(200); }
```

```rust
// Classicly:
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

// But we can used Result<T, E>
// If the test fail return an Err(E)
#[test]
fn it_works() -> Result<(), String> {  // Result<T, E>   // () is unit and is the Rust empty.
    if 2 + 2 == 4 {  // The test is HERE!!! NO ASSERT HERE like in above example!!!
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
    // Pros: we can use `?` operator to unwrap Ok() or early return Err(E) in case of Err(E).
}
```
