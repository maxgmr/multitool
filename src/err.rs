pub fn empty() {
    println!("error: No command given");
    println!("");
    println!("For more information try cargo run -- help");
}

pub fn unknown() {
    println!("error: Unknown command");
    println!("");
    println!("For more information try cargo run -- help");
}