use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut arg0 = true;
    for a in std::env::args() {
        if arg0 {
            arg0 = false;
            continue;
        }
        let content = read_file(&a).expect(&format!("Cannot read {}", a));
        println!("{}={}", a, content);
    }
}

fn read_file(f: &str) -> Result<String, io::Error> {
    let mut file = File::open(f)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
