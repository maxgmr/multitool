/*
 * Multitool, by Max Gilmour
 * 
 * A really weird command-based Rust program that has a bunch of different functions.
 * This is my first Rust program- please be nice.
 * 
 * Made with help from https://stevedonovan.github.io/rust-gentle-intro/
 */

mod cli;
mod cmd;
mod err;
mod edit;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = cli::Cli::new(&args);

    cli::execute_command(cli);
}