fn main() {
    let v = 3;
    if v < 10 {
        println!("true");
    } else if v == 0 {
        println!("zero");
    } else {
        println!("false");
    }
    let c = if v < 10 { "true" } else { "false" };
    println!("c={c}");
}
