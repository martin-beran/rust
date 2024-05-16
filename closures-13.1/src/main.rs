fn f_once<F>(f: F)
where
    F: FnOnce() -> String,
{
    f();
    //f();
}

fn f_mut<F>(mut f: F)
where
    F: FnMut() -> String,
{
    f();
    f();
}

fn f_fn<F>(f: F)
where
    F: Fn() -> String,
{
    f();
    f();
}

fn main() {
    let f0 = || {
        println!("f0 called");
    };
    f0();
    let f2 = |a, b| a + b;
    let (x, y) = (12, 23);
    println!("{x}+{y}={}", f2(x, y));

    let f2 = |a, b| a + b;
    let (x, y) = (40000, 40000);
    println!("x+y={}", f2(x, y));
    // This line forces parameters of f2, and hence x, y, to be u16. It causes
    // integer overflow in the previous line.
    //println!("x+y={}", f2(1u16, 2u16));

    // Capture by immutable reference
    println!("immutable ref");
    let array = vec![1, 2, 3, 4];
    println!("array before closure={:?}", array);
    let borrows = || println!("array in closure={:?}", array);
    println!("array after closure={:?}", array);
    borrows();
    println!("array after call={:?}", array);

    // Capture by mutable reference
    println!("mutable ref");
    let mut array = vec![1, 2, 3, 4];
    println!("array before closure={:?}", array);
    let mut borrows = || array.push(5);
    //println!("array after closure={:?}", array);
    borrows();
    println!("array after call={:?}", array);

    // Capture by move
    println!("move");
    let mut array = vec![1, 2, 3, 4];
    println!("array before closure={:?}", array);
    let mut borrows = move || array.push(5);
    //println!("array after closure={:?}", array);
    borrows();
    //println!("array after call={:?}", array);

    let s = String::from("abc");
    let fs_once = || {
        println!("fs_once({s})");
        s
    };
    f_once(fs_once);
    let mut s = String::from("X");
    let mut fs_mut = || {
        println!("fs_mut({s})");
        s.push('X');
        s.clone()
    };
    f_mut(&mut fs_mut);
    f_mut(fs_mut);
    let s = String::from("X");
    let fs = || {
        println!("fs({s})");
        s.clone()
    };
    f_fn(fs);
    f_fn(fs);
}
