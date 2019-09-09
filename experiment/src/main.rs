use std::collections::BTreeMap;
use std::env;
use std::result::Result;

struct CommandRec {
    fun: fn(),
    help: String
}

fn help() {
    println!("Help");
}

fn main() -> Result<(), ()> {

    let mut commands = BTreeMap::new();
    commands.insert(
        String::from("help"),
        CommandRec{fun: help, help: String::from("Display help")
    });

    let mut args = env::args();
    args.next();
    if args.len() < 1 {
        println!("Missing arguments");
        return Err(());
    }

    let cmd = args.next().expect("None in next()");
    println!("Command: {}", cmd);

    match commands.get(&String::from(&cmd)) {
        None => {
            println!("Invalid command {}", cmd);
            return Err(());
        },
        Some(cr) => (cr.fun)()
    }
    return Ok(());
}
