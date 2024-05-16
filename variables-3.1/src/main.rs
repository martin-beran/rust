use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("3h={THREE_HOURS_IN_SECONDS}s");
    let mut x = 5;
    println!("x={x}");
    x = 6;
    println!("x={x}");

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _z) = tup;
    println!("x={x} y={}", y);
    println!(".0={} .1={} .2={}", tup.0, tup.1, tup.2);

    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Index?");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: i32 = index.trim().parse().expect("Bad number");
    let index: usize = index.try_into().unwrap();
    let element = a[index];
    println!("array[{}]={}", index, element);
}
