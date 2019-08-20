fn main() {
    let x = 5;
    let r = &x;
    println!("x={} r={}", x, r);
    let x = 6;
    println!("x={} r={}", x, r);

    let t = (1, 2, 3, 4);
    println!("t2={}", t.2);
}
