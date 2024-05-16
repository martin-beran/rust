fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first<'a, 'b>(x: &'a str, _y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
    part_s: &'static str,
}

fn main() {
    let s1 = String::from("abcd");
    let _s2 = "xyz";
    let s2: &'static str = "XYZ";
    let result = longest(s1.as_str(), s2);
    println!("longest={}", result);
    let result = first(s1.as_str(), s2);
    println!("first={}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
        part_s: s2,
    };
    println!("{} {}", i.part, i.part_s);
}
