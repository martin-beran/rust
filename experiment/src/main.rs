use std::env;

fn main() {
    let args = env::args();
    let mut i = 0;
    for a in args {
        arg(i, a);
        i += 1;
    }
    for e in env::vars() {
        let (n, v) = e;
        env(n, v);
    }
}

fn arg(i: i32, v: String) {
    println!("argv[{}] = {}", i, v);
}

fn env(n: String, v: String) {
    println!("{}={}", n, v);
}
