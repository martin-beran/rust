fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for v in list {
        if v > largest {
            largest = v;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    fn format(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}
impl Point<i32> {
    fn is_float(&self) -> bool {
        false
    }
}
impl Point<f64> {
    fn is_float(&self) -> bool {
        true
    }
}

struct PointXY<X, Y> {
    x: X,
    y: Y,
}
impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, p: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY { x: self.x, y: p.y }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number={}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("largest char={}", result);

    let integer = Point { x: 3, y: 5 };
    let float = Point { x: 1.0, y: -2.3 };
    println!(
        "integer={} is_float={}",
        integer.format(),
        integer.is_float()
    );
    println!("float={} is_float={}", float.format(), float.is_float());
    let int16 = Point { x: 3i16, y: 5i16 };
    println!("int16={}", int16.format()); // no is_float() for Point<i16>

    let p1 = PointXY { x: 1, y: 2 };
    let p1x = p1.x;
    let p2 = PointXY { x: 3.4, y: 5.6 };
    let p2y = p2.y;
    let p3 = p1.mixup(p2);
    assert!(p3.x == p1x);
    assert!(p3.y == p2y);
}
