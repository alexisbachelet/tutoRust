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
