fn main() {
    //let s = "Hello, World!";
    {
        let s = "Hi!";
        println!("{s}");
    }
    //println!("{s}");
    
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");
    //let s2 = s1;
    let s2 = s1.clone();
    println!("{s1} {s2}");

    // move/copy to fun
    let mut s = String::from("value");
    let n = 234;
    s = report_num_ret(s, n);
    println!("s={s} n={n}");
    report_num(s, n);
    //println!("s={s}");
    println!("n={n}");
}

fn report_num(s: String, n: u32) {
    println!("{s}: {n}");
}

fn report_num_ret(s: String, n: u32) -> String {
    println!("{s}: {n}");
    s
}
