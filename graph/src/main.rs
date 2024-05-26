//! A graph of nodes connected by shared and weak pointers in Rust
//!
//! This is a reimplementation of C++ program graph_cpp.
//! A classical example of a dynamic graph data structure, with ownership managed by shared and weak
//! pointers. The graph topology is like in an application that dynamically creates sessions and
//! registers handlers for events on each session. Handlers keep references (smart pointers) to
//! sessions. If all handlers for a session are deleted (executed or canceled), the session is deleted
//! as well.

use crate::hnd_table::HndTable;
use std::env;
use std::io;
use std::process::{ExitCode, Termination};

/// The program entry point.
fn main() -> impl Termination {
    let mut argv = env::args();
    let argv0 = argv.next();
    if argv.len() != 0 {
        return usage(&argv0.unwrap());
    }
    run()
}

/// Reports usage instructions and return failure exit code.
fn usage(argv0: &str) -> ExitCode {
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
    ExitCode::FAILURE
}

/// The main loop.
///
/// It reads and process lines with commands from stdin and executes them.
fn run() -> ExitCode {
    let mut hnd_table = HndTable::new();
    let mut line = String::new();
    while let Ok(len) = { line.clear(); io::stdin().read_line(&mut line) } {
        if len == 0 {
            break
        }
        let tokens: Vec<&str> = line.trim().split([' ', '\t']).collect();
        match tokens.len() {
            3 =>
                match tokens[1] {
                    "+" => {
                        hnd_table.create_session(tokens[0], tokens[2]);
                        continue
                    }
                    "=>" => {
                        hnd_table.add_rc(tokens[0], tokens[2]);
                        continue
                    }
                    "->" => {
                        hnd_table.add_weak(tokens[0], tokens[2]);
                        continue
                    }
                    _ => ()
                }
            2 =>
                if tokens[0] == "!" {
                    hnd_table.erase(tokens[1]);
                    continue
                }
            1 =>
                if tokens[0] == "?" {
                    hnd_table.display();
                    continue
                }
            _ => ()
        }
        println!("???");
    }
    ExitCode::SUCCESS
}

/// A registered session
mod session {
    pub struct Session {
        name: String,
    }
    impl Session {
        pub fn new(name: &str) -> Self {
            Session {
                name: String::from(name)
            }
        }
        pub fn name(&self) -> &str {
            self.name.as_str()
        }
    }
    impl Drop for Session {
        fn drop(&mut self) {
            println!("deleted session {}", self.name());
        }
    }
}

/// A registered handler
mod handler {
    use crate::session::Session;
    use std::cmp::Ordering;
    use std::rc::{Rc, Weak};

    pub struct Handler {
        name: String,
        shared_p: Option<Rc<Session>>,
        weak_p: Weak<Session>,
    }
    impl Handler {
        pub fn new(name: &str) -> Self {
            Handler {
                name: String::from(name),
                shared_p: None,
                weak_p: Weak::new(),
            }
        }
        pub fn with_shared(name: &str, shared_p: &Rc<Session>) -> Self {
            Handler {
                name: String::from(name),
                shared_p: Some(Rc::clone(shared_p)),
                weak_p: Weak::new(),
            }
        }
        pub fn with_opt_shared(name: &str, shared_p: &Option<Rc<Session>>) -> Self {
            Handler {
                name: String::from(name),
                shared_p: shared_p.clone(),
                weak_p: Weak::new(),
            }
        }
        pub fn with_weak(name: &str, shared_p: &Option<Rc<Session>>) -> Self {
            Handler {
                name: String::from(name),
                shared_p: None,
                weak_p: match shared_p {
                    None => Weak::new(),
                    Some(p) => Rc::downgrade(p),
                }
            }
        }
        pub fn name(&self) -> &str {
            self.name.as_str()
        }
        pub fn get_shared(&self) -> Option<Rc<Session>> {
            match &self.shared_p {
                None => Weak::upgrade(&self.weak_p),
                Some(p) => Some(Rc::clone(&p)),
            }
        }
        pub fn is_shared(&self) -> bool {
            self.shared_p.is_some()
        }
    }
    impl PartialEq for Handler {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }
    impl Eq for Handler {}
    impl PartialOrd for Handler {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.name.cmp(&other.name))
        }
    }
    impl Ord for Handler {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(&other).unwrap()
        }
    }
}

/// A table of all registered handlers
mod hnd_table {
    use crate::session::Session;
    use crate::handler::Handler;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    /// The structure that keeps the table of handlers
    pub struct HndTable {
        data: BTreeSet<Handler>,
    }

    impl HndTable {
        pub fn new() -> Self {
            Self {
                data: BTreeSet::new()
            }
        }
        pub fn create_session(&mut self, handler_name: &str, session_name: &str) {
            if self.data.insert(Handler::with_shared(handler_name, &Rc::new(Session::new(session_name)))) {
                println!("Created");
            } else {
                println!("Handler already exists");
            }
        }
        pub fn add_rc(&mut self, from: &str, to: &str) {
            if let Some(h) = self.data.get(&Handler::new(to)) {
                if self.data.insert(Handler::with_opt_shared(from, &h.get_shared())) {
                    println!("Created");
                } else {
                    println!("Handler already exists");
                }
            } else {
                println!("Target handler does not exist");
            }
        }
        pub fn add_weak(&mut self, from: &str, to: &str) {
            if let Some(h) = self.data.get(&Handler::new(to)) {
                if self.data.insert(Handler::with_weak(from, &h.get_shared())) {
                    println!("Created");
                } else {
                    println!("Handler already exists");
                }
            } else {
                println!("Target handler does not exist");
            }
        }
        pub fn erase(&mut self, handler_name: &str) {
            if self.data.remove(&Handler::new(handler_name)) {
                println!("Handler erased");
            } else {
                println!("Handler does not exist");
            }
        }
        pub fn display(&self) {
            for h in &self.data {
                print!("{}", h.name());
                if let Some(p) = h.get_shared() {
                    print!(" {} {}", if h.is_shared() { "=>" } else { "->" }, p.name());
                }
                println!("");
            }
        }
    }
}
