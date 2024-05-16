// comment
/* another comment */

fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(51.0, 'm');
    let i2 = times2(12);
    println!("{}", i2);
    let _u = unit_type(123);
}

fn another_function(x: i32) {
    println!("x={x}");
}

fn print_labeled_measurement(v: f64, unit: char) {
    println!("v={}{}", v, unit);
}

fn times2(i: i32) -> i32 {
    2 * i
}

fn unit_type(_i: i32) -> () {
}
