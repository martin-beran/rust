fn main() {
    try_vec();
    try_string();
    try_hashmap()
}

fn try_vec() {
    let mut v0: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    print_vec("v0", &v0);
    print_vec("v1", &v1);

    for e in &v1 {
        v0.push(10 * e);
    }
    print_vec("v0", &v0);
    for e in &mut v0 {
        *e = -*e;
    }
    print_vec("v0", &v0);

    //println!("last={} after={}", v1[v1.len() - 1], v1[v1.len()]);
    println!("last={:?} after={:?}", v1.get(v1.len() - 1), v1.get(v1.len()));
}

fn print_vec<T: std::fmt::Display>(name: &str, v: &Vec<T>) {
    let mut delim = '{';
    print!("{name}=");
    if v.is_empty() {
        print!("{delim}");
    }
    for e in v {
        print!("{delim}{e}");
        delim = ',';
    }
    println!("}}");
}

fn try_string() {
    let mut s0 = String::new();
    let s1 = "abc".to_string();
    let s2 = String::from("ABC");
    println!("s0={s0}");
    println!("s1={s1}");
    println!("s2={s2}");
    s0.push_str(&s1);
    s0.push_str(&s2);
    println!("s0={s0}");
    s0 = s2 + &s1;
    println!("s0={s0}");
    // moved
    //println!("s2={s2}");
    let s3 = format!("\"{s0}-{s1}\"");
    println!("s3={s3}");
    for c in "Здравствуйте नमस्ते".chars() {
        println!("'{c}'");
    }
    for b in "Здравствуйте नमस्ते".bytes() {
        println!("{b}");
    }
}

fn try_hashmap() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let v = scores.get("Blue").copied().unwrap_or(0);
    println!("v={v}");
    for (k, v) in &scores {
        println!("{k}->{v}");
    }
    println!("{:?}", scores);
    scores.insert(String::from("Yellow"), 51);
    println!("{:?}", scores);
    scores.entry(String::from("Blue")).or_insert(21);
    println!("{:?}", scores);
    scores.entry(String::from("Red")).or_insert(22);
    println!("{:?}", scores);
    let mut scores2 = HashMap::new();
    scores2.insert("Blue", 30);
    scores2.insert("Yellow", 10);
    scores2.insert("Red", 20);
    println!("{:?}", scores2);
    for k in "Blue Red Green Yellow".split_whitespace() {
        let v = scores2.entry(k).or_insert(0);
        *v += 100;
    }
    println!("{:?}", scores2);
}
