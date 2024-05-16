fn main() {
    let mut args = Vec::new();
    for a in std::env::args() {
        args.push(a);
    }
    open_file(&args[1]);
}

fn open_file(arg1: &str) {
    let _fh = match std::fs::File::open(arg1) {
        Ok(fh) => fh,
        Err(e) => panic!("Cannot open file: {:?}", e),
    };
}
