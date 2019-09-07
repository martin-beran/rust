use std::collections::BTreeMap;
use std::env;
use std::result::Result;

struct CommandRec {
}

fn help() {
}

fn main() -> Result<(), ()> {
    let commands: BTreeMap<&str, CommandRec>;
    let mut args = env::args();
    args.next();
    if args.len() < 1 {
        println!("Missing arguments");
        return Err(());
    }
    let cmd = args.next();
    println!("Command: {}", cmd.expect("None in next()"));
    return Ok(());
}
