//! A graph of nodes connected by shared and weak pointers in Rust
//!
//! This is a reimplementation of C++ program graph_cpp.
//! A classical example of a dynamic graph data structure, with ownership managed by shared and weak
//! pointers. The graph topology is like in an application that dynamically creates sessions and
//! registers handlers for events on each session. Handlers keep references (smart pointers) to
//! sessions. If all handlers for a session are deleted (executed or canceled), the session is deleted
//! as well.

use crate::hnd_table::HndTable;

/// The program entry point.
fn main() -> impl std::process::Termination {
    let mut argv = std::env::args();
    let argv0 = argv.next();
    if argv.len() != 0 {
        return usage(&argv0.unwrap());
    }
    run()
}

/// Reports usage instructions and return failure exit code.
fn usage(argv0: &str) -> std::process::ExitCode {
    eprintln!("usage: {}{}", argv0, "

Maintains a graph inspired by relation of objects in an application based
on Boost.Asio. There are session and handler nodes. Each handler keeps a shared
or weak pointer to a session. There is a table of all waiting handlers, but
sessions are referenced only from handlers. So, when all handlers pointing to a
session are deleted, the session is deleted as well.

The program reads commands from stdin (H and S are string names of a handler
and a session, respectively), each command on a separate line:

H + S ... creates a new session S and a new handler H, and stores a shared
          pointer to S in H

H1 => H2 ... creates a new handler H1 with a shared pointer pointing to the
             same session as H2

H1 -> H2 ... creates a new handler H1 with a weak pointer pointing to the
             same session as H2

! H ... \"executes\" and deletes handler H

? ... displays list of handlers and their pointers to sessions
");
    std::process::ExitCode::FAILURE
}

/// The main loop.
///
/// It reads and process lines with commands from stdin and executes them.
fn run() -> std::process::ExitCode {
    let mut hnd_table: HndTable;
    let mut line = String::new();
    while let Ok(len) = std::io::stdin().read_line(&mut line) {
        if len == 0 {
            break
        }
        println!("len={} line=\"{}\"", len, line);
        line.clear();
    }
    std::process::ExitCode::SUCCESS
}

/// A registered session
mod session {
}

/// A registered handler
mod handler {
}

/// A table of all registered handlers
mod hnd_table {

    /// The structure that keeps the table of handlers
    pub struct HndTable {
    }

}
