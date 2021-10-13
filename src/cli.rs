use crate::cmd;
use crate::err;

pub enum CliCommand {
    Empty,
    Unknown,
    Help,
}

pub struct Cli {
    command: CliCommand,
    args: Option<Vec<String>>
}

impl Cli {
    pub fn new(cli_args: &[String]) -> Cli {
        parse_args(cli_args)
    }
}

pub fn execute_command(cli: Cli) -> () {
    print!("\x1B[2J\x1B[1;1H");
    println!("=======");

    match cli.command {
        CliCommand::Empty => err::empty(),
        CliCommand::Unknown => err::unknown(),
        CliCommand::Help => cmd::help()
    }
}

fn parse_args(args: &[String]) -> Cli {
    match &args.len() {
        0 | 1 => Cli {command: CliCommand::Empty, args: None},
        2 => Cli {command: parse_command(&args[1]), args: None},
        _ => Cli {command: parse_command(&args[1]), args: Some((&args[2..]).to_vec())}
    }
}

fn parse_command(args: &String) -> CliCommand {
    match &args.as_str() {
        &"help" | &"h" => CliCommand::Help,
        _ => CliCommand::Unknown
    }
}