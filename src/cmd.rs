use std::fs::File;
use std::io::Read;

/*
 * =========
 * HELP
 * =========
 * This command shows a list of commands and general program info.
 */
pub fn help() {
    println!("Multitool");
    println!("Has a variety of relatively unrelated functions");
    println!("");

    println!("USAGE:");
    println!("\tcargo run [COMMAND]");
    println!("");

    println!("COMMANDS:");
    println!("\t-- help, -- h                            Prints usage information and command list");
    println!("\t-- hello <NAME>                          Says hello to you and your friends");
    println!("\t-- rangesum <LOWER BOUND> <UPPER BOUND>  Sums all the numbers from the lower bound to the higher bound");
    println!("\t-- factorial <NUMBER>                    Returns the factorial of the given number");
    println!("\t-- notes, -- n                           Shows personal notes on Rust saved to computer");
}

/*
 * =========
 * HELLO
 * =========
 * This command greets you using your name, or produces a list of greetings if given a list of names.
 */
pub fn hello(args: Option<Vec<String>>) {
    match args {
        //Some(xs) => println!("Hello, {}!", xs[0]),
        Some(xs) => {
            for x in xs.iter() {
                println!("Hello, {}!", x)
            }
        }
        None     => println!("Hello there!")
    }
}

/*
 * =========
 * RANGESUM
 * =========
 * This command adds up all the numbers from arguments a to b.
 */
pub fn rangesum(args: Option<Vec<String>>) {
    match args {
        Some(xs) => {
            let range = rangesum_get_range(xs);

            match range {
                Ok((lower_bound, upper_bound)) => {
                    if lower_bound > upper_bound {
                        println!("error: Lower bound must be smaller than upper bound");
                        println!("");
                        println!("USAGE:");
                        println!("\tcargo run -- rangesum <LOWER BOUND> <UPPER BOUND>");
                    } else {
                        let mut sum = 0;
                        for i in lower_bound..(upper_bound + 1) {
                            sum += i;
                        }

                        println!("Sum from {} to {} is {}", lower_bound, upper_bound, sum);
                    }
                }
                Err(e) => {
                    println!("error: {:?}", e);
                    println!("");
                    println!("USAGE:");
                    println!("\tcargo run -- rangesum <LOWER BOUND> <UPPER BOUND>");
                }
            }
        }
        None     => {
            println!("error: No arguments");
            println!("");
            println!("USAGE:");
            println!("\tcargo run -- rangesum <LOWER BOUND> <UPPER BOUND>");
        }
    }
}

fn rangesum_get_range(args: Vec<String>) -> Result<(u32, u32), &'static str> {
    match args.len() {
        2 => {
            match args[0].parse::<u32>() {
                Ok(n1) => {
                    match args[1].parse::<u32>() {
                        Ok(n2) => Ok((n1, n2)),
                        Err(_) => Err("arguments must be positive integers")
                    }
                }
                Err(_) => Err("arguments must be positive integers")
            }
        }
        _ => Err("incorrect number of arguments")
    }
}

/*
 * =========
 * FACTORIAL
 * =========
 * This command prints the factorial of the given number.
 */
pub fn factorial(args: Option<Vec<String>>) {
    match args {
        Some(xs) => {
            match factorial_get_num(xs) {
                Ok(n) => {
                    let result = factorial_recursive(n);

                    if result == 0 {
                        println!("error: Argument must be positive");
                        println!("");
                        println!("USAGE:");
                        println!("\tcargo run -- factorial <NUMBER>");
                    } else {
                        println!("{}! = {}", n, result)
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                    println!("");
                    println!("USAGE:");
                    println!("\tcargo run -- factorial <NUMBER>");
                }
            }
        }
        None     => {
            println!("error: No arguments");
            println!("");
            println!("USAGE:");
            println!("\tcargo run -- factorial <NUMBER>");
        }
    }
}

fn factorial_recursive(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n-1)
    }
}

fn factorial_get_num(args: Vec<String>) -> Result<u128, &'static str> {
    match args.len() {
        1 => {
            match args[0].parse::<u128>() {
                Ok(n) => Ok(n),
                Err(_) => Err("argument must be positive integer")
            }
        }
        _ => Err("incorrect number of arguments")
    }
}

/*
 * =========
 * NOTES
 * =========
 * This command prints the Rust notes saved to the computer.
 */
pub fn notes() {
    match read_to_string(&"rust-notes.txt") {
        Ok(text) => println!("{}", text),
        Err(e) => println!("{}", e)
    }
}

fn read_to_string(filename: &str) -> Result<String, &'static str> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => return Err("Error opening file.")
    };
    let mut file_text = String::new();
    match file.read_to_string(&mut file_text) {
        Ok(_) => Ok(file_text),
        Err(_) => Err("Error reading file.")
    }
}

/*
 * =========
 * SUM
 * =========
 * This command prints the sum of a given list of numbers.
 */
pub fn sum(args: Option<Vec<String>>) {
    match args {
        Some(xs) => {
            let numbers: Vec<f64> = xs.iter().flat_map(|x| x.parse()).collect();

            if numbers.len() == 0 {
                println!("error: Invalid arguments. Arguments must be numbers.");
                println!("");
                println!("USAGE:");
                println!("\tcargo run -- sum <NUMBER1> <NUMBER2>...");
            } else {
                println!("Sum = {:.2}", sum_numlist(&numbers[..]))
            }
        }
        None => {
            println!("error: No arguments");
            println!("");
            println!("USAGE:");
            println!("\tcargo run -- sum <NUMBER1> <NUMBER2>...");
        }
    }
}

fn sum_numlist(list: &[f64]) -> f64 {
    match list {
        [] => 0.0,
        [n, ns @ ..] => n + sum_numlist(ns)
    }
}