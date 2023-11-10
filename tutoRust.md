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
# eval to make a double evaluation:
# 1 generate variable, 2 print it.
eval "$(ssh-agent -s)"  # $(cmd) = `cmd` = run cmd and place output on main.
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
mkdir hello_world
cd hello_world
```

```rust
// main.rs
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

### Test aguments

```bash
cargo test argForSelectTest -- argToAllTest
```

```bash
cargo test -- --test-threads=1  # 1 Thread so no parallelism.
cargo test -- --show-output  # Show the print in test functions.
cargo test sub_set_of_test_name  # To only select subset of tests.
```

```rust
#[test]
#[ignore]  // To ignore a test.
fn expensive_test() { ... }
```

### Test organisation

Two types:

* **Unit test**
* **Integration tests:** import (use) our library and tets it like an external user.

#### Unit Tests

You’ll put unit tests in the **src** directory in each file with the code that they’re testing. The convention is to create a module named **tests** in each file to contain the test functions and to annotate the module with cfg(test).

```rust
// `pub` on a module only lets code in its ancestor modules refer to it.
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]  // Compile this piece of code only for `cargo test` not `build`.
mod tests {
    use super::*;

    #[test]
    fn internal() {
        // We can use (import) private item (function) in child module
        // from their ancestor mudule.
        // So we use internal_adder in `tests` mod.
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

#### Integration Tests

We nneed to create a **tests** repo with as many file as we want.

```tree
my_project
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```rust
// tests/integration_tests.rs
// No need to add a #[cfg(test)]: tests files are separated crate from main.
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

We can use a setup test file to share code with other test files:

```rust
// tests/common/mod.rs
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

```rust
// tests/integration_test.rs
use adder;  // Because is an external crate.
mod common;  // Because is in the same crate but in different files.

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

We cannot use integration tests on binary crate. So we need to create a library crate with all the code and a minimum binary crate that use the lib.
So we can test the lib and use it with the bin.

### Project: Create a command line program

We are going to make `minigrep` binary code: We specify a path file and a string to find.
All the lines of the file that contain the specific word are printed

```bash
cargo new minigrep
cd minigrep
cargo run -- mySubString myFile.txt  # cargoArg -- binArgs.
```

```rust
// use is a shortcut to avoid write full path many times.
use std::env; // To get cmd line param.
use std::process; // To exit code.

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);  // to print all data of an object.

    //  err = the iner value of an Err type
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // The exit status code.
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // exec run and if it's return an Err then exec the {}.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

```rust
// src/lib.rs 
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// Step 1: create an object.
impl Config {
    // build and not new because new must never fail.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Step 2: use the object.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> mean return a type that implements the Error trait.

    // .expect("myErrorMessage") ; "?" extract or return err.
    let contents = fs::read_to_string(config.file_path)?;
    
    //println!("With text:\n{contents}");
    for line in search(config.query, config.contents) {
        println!("{line}")
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

The first step in TDD is to fail, so we must write a failling test.
We write begining (var init) and ending (test) but the middle part must return nothing.

```rust
// lib.rs
#[cfg(test)]  // Only compile in test case.
mod tests {
    use super::*;  // Relative to the file (mod). So here import the lib.

    #[test]  // Create a test function and run it with `cargo test`.
    fn one_result() {
        let query = "duct";
        // The '\' is to avoid a new line.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

We can capt `env` variable with:

```rust
use std::env;
let ignore = env::var("IGNORE_CASE").is_ok();  // If is defined by the user.
```

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

We can also pritn on error steam than standard output:

```rust
eprintln!();
```

Step to know for dev:

1. Split your program into a *main.rs* and a *src/lib.rs*. The main call the lib and handle error on it. Main init a *lib* object and use it.
1. Always use struct instead of tuples

## Iterator and closures

**Functional programming:** A value can be a function for latter used. They are named **Closures**.

### Closures

```rust
// # to speack to the compiler.
// derive to quickly implement a trait (program derivation) to our struct.
// Instead of write a generic code by ourself we can quickly automaticaly do it.
// Trait is a collection of method for genereic type.
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
```

```rust
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

Closure are short function with no type:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

let res = add_one_v4(4);  // To use it.
```

The type of a closure is automatically determined by the fist used of the closure.
So if we changer the paramaters type in a second use, there is a bug.

A closure automaticaly determined if it need to : borrowing immutably, borrowing mutably or taking ownership. SO no need to use `&`. Recall: no other borrows are allowed when there’s a mutable borrow.

```rust
let list = vec![1, 2, 3];
let only_borrows = || println!("immutable borrow: {:?}", list);
```

But we can force a closure to take ownerchip if we want (with `move`):

```rust
move || x
```

How much we can repeat (call) a closure:

* `FnOnce` because can only be call once. Because we `move` value, so it can all only work once. he second time the value is aleready gone.
* `FnMut` to `mut` values. On a slice call the cosure multiple times.
* `fn` multiple time with no change.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);  // FnMut: no return just mutation
}
```

### Iterator

In Rust, iterators are lazy, meaning they are not use imediatly.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

`Item` type is used in the return type of the `next` method. The first next is always the first element in a `Option` type.

```rust
let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter();  // With next we need to def a mut var iterator.
assert_eq!(v1_iter.next(), Some(&1));

for v in v1.iter() {}  // A for take owernship so it's implied mut.
```

Methods that call next are called **consuming adaptors**:

```rust
let total: i32 = v1.iter().sum();
```

**Iterator adaptors** are methods that don’t consume the iterator. Instead they produce a new different iterator.

```rust
// "_" to let Rust infer the Vec type.
// We need to use a .coolect() to consume the iterator and get res.
 let v: Vec<_> = v1.iter().map(|x| x + 1).collect();

//
v1.iter().filter(|s| s.size == 10).collect()
 ```

### Improve code with iterator and closure

Instead of borrowing a slice `&[String]` we can use iterator.

```rust
impl Config {
    // mut args beacause we are going to mut it by iterating over it.
    // args: generic type that implements the Iterator trait on string.
    pub fn build(mut args: impl Iterator<Item = String>) -> ...
    //pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path,ignore_case, })
}
```

```rust
pub fn build(...) {
    args.next();  // Because the first arg is always the code name.
    
    // No if let because we exec code on None variant.
    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("No word query"),  // Early return of builf function.
    };

    Ok(Config { query, })
}
```

We can rewrite also like this:

```rust
for line in contents.lines() {
    if line.contains(query) {
        results.push(line);
    }
}
result  // return

// We chain the dot.
// Here no mutable intermediate results vector: so we can have multiple thread.
contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
```

## Cargo bonus

### Compile

There is a special `build` when we have finish the dev:

```bash
cargo build  # dev profile.
cargo build --release  # To have more optimizations on compilation.
```

### Documentation

Documentation is how to use the code. USe three `///` and support Markdown.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```bash
cargo doc  # To generate the doc.
cargo doc --open  # Open on a web browser.
```

List:

* `///` To comment the next item.
* `//!` To comment the current item, if there is no item: the binary is commented.

```rust
//! # My Lib
//! 
//! Here we speak about the concept of this lib.
//! There is a blank line just after to ot commented `Kinds` but the bin.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}
```

### Package

We can also create a **package** to have more than one lib on a project. So our main.rs can use two or more lb.rs

Step one manuanly create a directory:

```bash
mkdir add
cd add
```

Then manualy add a `toml`:

```toml
# Cargo.toml
[workspace]

members = [
    "adder",
]
```

```bash
cargo new adder  # Add the bin on workspace.
```

Then rewrite `toml` to add the first lib.

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

```bash
cargo new add_one --lib
```

We can write our code on this `lib.rs`

Then we must link our `bin.rs` to `lib.rs`

```toml
# adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

```rust
use add_one;
```

Then we can exec code:

```bash
$ cargo build
cargo run -p adder  # p stand for package.
```

We need to sepecify external package depensy on `toml` files of lib or bin but not the workspace `toml`, to use it:

```toml
[dependencies]
rand = "0.8.5"
```

## Smart Pointers

A pointer is a variable that point (refer) to the memory adress of some other data.
A reference `&` is a pointer.

Diference bettween pointers ans references:

* A smart pointer have some feature thar reference doesent have.
* *Reference* **borrow** but *smart pointer* **own** the data they point to.

### Box: to store on the heap

Two reason to chose on the heap instead of stack:

* When it's impossible to know the size of an object (recurcive type)
* Transfer ownership without coping. Just pointer are copied

```rust
fn main() {
    let b = Box::new(5);
    // Here b is a pointer but act like a reference to 5.
    // println! use ref, so don't need to deref to print.
    println!("b = {}", b);
}
```

```rust
enum List {
    Cons(i32, List),
    Nil,
}

//with:
use crate::List::{Cons, Nil};
// No need anymore to write:
let list = List::Cons(1, List::Nill)
// but just:
let list = Cons(1; Nill)  // nil is a empty list

fn main() {
    // Cons for Construct
    // Nil for the end of a recurcion.
    // (1, (2, (3, Nil)))
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

Instead of have an object with infinite size. We use indirection.  
To store data indirectly thank to a pointer. Instead to have on infinite big block we have several definite blocks of data. Each of them point to the next.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

### Deref

Deref is used on pointer (reference) to return the value they point to.
A pointer is a reference (adress memory) of a variable. With deref instead of return the ref we return the value.

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

To implent the deref trait. We need in one to define our own type:

```rust
//Define a "tuple structs", to store only one value of any type.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Next we can add the deref trait:

```rust
use std::ops::Deref;  // ops = operators.

// Associated types connect a type placeholder with a trait such that:
// the trait method definitions can use these placeholder in their signatures.
// slightly different way of declaring a generic parameter
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // "*y" is equivalent to "*(y.deref())"
    // so here y.deref() return the value in "myBox"
    // and "*()"  return the real value even if y.deref() is a reference.
    assert_eq!(5, *y);
}
```

Deref coercion: when we return a ref of the wrong type. Rust can automaticaly change the type.

```rust
fn hello(name: &str) {  // use "&str"
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // we give a "&String" but it's OK for hello function.
}
```

There is two options:

* Deref: fot ref
* DerefMut: for mutable references

### Cleanup memory

Automaticaly exec the drop trait when a variable go out of scope.
Variables are dropped in the reverse order of their creations.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

```text
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

We can also early drop if we want (no need to wait a variable go out of scope):

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

```text
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

### Multiple Ownership. Reference Counted

Ownership is clear: one variable owns it's value.
In some case it's usefull to have multiple ownership for one value.
The value is not droped when a variable go out of scope beacause there is another value that own it.
The value is droped when there is no variable that use it.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// "crate" because we used it on main which it's a inner code.
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));  // Rc::new.
    let b = Cons(3, Rc::clone(&a));  // Rc::clone.
}
```

To get the number of references:

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
}
```

### Interior Mutability

Interior mutability: to mut a inmutable refrences.

Unsafe mode: to not use the compiler ownership rules. We manualy manage these rule.
But it's slower because Rust check the borrowing rules not at compile time but in runtime. Because the compiler it's too cautious we can be more greedy with the unsafe mode.

* `Box<T>`: recurcive type, fast copy (just the pointer is copied).
* `Rc<T>`: to have multiple owership of an single data but not mutability.
* `RefCell<T>`: Interior Mutaility, mut a single unmutable variable.

Mock objects are specific types of test doubles (*doublures in french*) to keep recort to what appen in a test.

```rust
// Generic method for miceleaneous struct.
// Important to notice is "&self" so self is unmutable.
pub trait Messenger {
    fn send(&self, msg: &str);
}

// "'a" is a generic lifetime parameters. It's important when we use references.
// Any instance of LimitTracker can’t outlive the reference in "messenger".
// "T" is a generic type  that implement the "Messenger" trait.
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// We add method on our struct.
// We define a struct that use an another struct.
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,  // Just a trait bound to a generic type.
{
    // The first method.
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // The second.
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

So we can define a "Messenger struct" (Mock object) to test the LimitTracker struct. The limitTracker will always send a message to the messegerObject but this not a normal messenger. It we never give the message instead it will store it in a vector.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Define our own Messenger stuct.
    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    // Add some methids.
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    // Implement the trait.
    impl Messenger for MockMessenger {

        // BUG !!! Because &self is unmutable and "push" is a mut cmd.
        // But we cannot use either a "send(&mut self, message: &str)
        // Because it's not fit the trait signature.
        // So we need a "trick" an unmutable var but in fact it's mut
        // So we nned to use a ReffCells.
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

This is the trick the ReffCell an unmutable var whish is mutable:

```rust
use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {

    // Now we can push on a &self
    fn send(&self, message: &str) {
        // borrow_mut() to have a "&mut var" which is a "RefMut<T>"
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

// Borrow() to have a "&var" which is a "Ref<T>"
assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
```

`RefCell<T>`  lets us have many immutable borrows or one mutable borrow at any point in time. But we can change this with `Rc<RefCell<T>>`

```rust
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    // First use of value.
    let a = Rc::new(
        Cons(Rc::clone(&value), Rc::new(Nil))
    );

    // Second and thrid.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // We have multiple ref of &a and &value.
    // But we can even mut &a.
    *value.borrow_mut() += 10;
}
```

### Reference Cycles Can Leak Memory

A memory that it's nver clean when the rust program is finished. Because each variable are in a infinite reference cycle.

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {

    // RefCell to mut an imnmutable variable.
    //
    // Rc to have multiple ownership (of an immutable object).
    // Rc to shared an variable "a" with other variables "b" and "c"
    // without use a ref to "a".
    //
    // So we can:
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // To easily access to the second element.
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

```rust
// b and c are list that use a.

// "a" is on Rc<List> type, so we can have several List.
// 5, Nil.
let a = Rc::new(
    Cons(5, RefCell::new(Rc::new(Nil)))
);

// 10, "a" (5, Nil)
let b = Rc::new(
    Cons(10, RefCell::new(Rc::clone(&a)))
);

// If the last elem in the Cons List 
if let Some(link) = a.tail() {
        // borrow_mut() return a Rc(Nil)
            // borrow_mut() return a Rc(Nil)
    // we change Rc(Nil) by Rc(&b)
    // So "a" link to "b", but also "b" link to "a".
    // It's an infinite loop.
    *link.borrow_mut() = Rc::clone(&b);
}
```

Variable "a" and "b" are pointers so they have a *stack* part and on the *heap* part. When Rust finished to run, it drop varibale on a reverse order.  
So "B" is drop with this stack part but the heap part still remain beacause "B" is still in use in "A".  
Then "A" go out of scope, so heap part is clean but on the heap still remain because it' on "B" part.  
So "A" and "B" are dropped but the heap part still remain because they reference to each others.

We have an danger with `RefCell(Rc<T>)`. But if we use `Rc::downgrad` instead of `Rc::clone` we can avoid references cycles. Because weak references don't increase the strong count (the number of time a variable is shared). So for exemple when "B" go out of scope it will be dropped because it's now theorocally used in "A".

To explain we are goin to create a tree:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,

    parent: RefCell<Weak<Node>>,

    // A Node can have several children so xe use a Vec.
    // A children can be use by other Node, so we need a Rc.
    // RefCell to change the children of a node.
    children: RefCell<Vec<Rc<Node>>>,
}
```

In this exemple we can shared references but att the end Rust clean memory coorectly.

```rust
// The end node.
let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new())
    children: RefCell::new(vec![]),  // No childrens.
});

// The begining node that hold "leaf".
let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

// To return a Weak from a Rw we nned to call a downgrade.
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// We need to upgrade a Weak reference to return a Some(Rc) to use it.
// Because we are not sure the weak reference is still valid.
// Because it can be already cleared. 1 
println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
```
