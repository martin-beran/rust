fn main() {
    let mut args = Vec::new();
    for a in std::env::args() {
        args.push(a);
    }
    do_panic(&args[1]);
}

fn do_panic(arg1: &str) {
    panic!("arg={arg1}");
}
