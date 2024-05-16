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

    // string len, various ownership
    let s = String::from("a string value");
    //let n = str_len(s);
    let n = str_len(s.clone());
    println!("s={} val {}", s, n);
    let (mut s, n) = str_len_copy(s); // mut needed by str_append below
    println!("s={} copy {}", s, n);
    let n = str_len_ref(&s);
    println!("s={} ref {}", s, n);
    str_append(&mut s);
    println!("s={}", s);
    
    // multiple references
    let sr = &mut s;
    //let _l = str_len_ref(&s);
    let sr2 = &sr;
    println!("sr={} sr2={}", sr, sr2);
    
    // string slices
    let mut s = String::from("Some string with multiple words");
    let w1 = first_word(&s);
    //s.clear();
    println!("first_word={}", w1);
    let so = "Another multi-word string";
    println!("{}", first_word(so));
    println!("{}", first_word(&so[8..]));
    println!("{}", first_word(&so[8..13]));

    // array slices
    let a = [0, 1, 2, 3];
    print_array_slice(&a);
    print_array_slice(&a[1..3]);
}

fn report_num(s: String, n: u32) {
    println!("{s}: {n}");
}

fn report_num_ret(s: String, n: u32) -> String {
    println!("{s}: {n}");
    s
}

fn str_len(s: String) -> usize {
    s.len()
}

fn str_len_copy(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn str_len_ref(s: &String) -> usize {
    s.len()
}

fn str_append(s: &mut String) {
    s.push_str(" tail");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; 
        }
    }
    &s[..]
}

fn print_array_slice(a: &[i32]) {
    for v in a {
        println!("{v}");
    }
}
