#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Square {
    side: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
    fn square(side: i32) -> Rectangle {
        Rectangle { width: side, height: side }
    }
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

impl Square {
    fn new(side: i32) -> Square {
        Square { side }
    }
    fn area(&self) -> i32 {
        self.side * self.side
    }
    fn double(&mut self) {
        self.side *= 2;
    }
    fn to_rectangle(self) -> Rectangle {
        Rectangle { width: self.side, height: self.side }
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width != 0
    }
    fn height(&self) -> bool {
        self.height != 0
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height
    }
}

impl Square {
    fn side(&self) -> bool {
        self.side != 0
    }
    fn can_hold(&self, s: &Square) -> bool {
        self.side >= s.side
    }
}

fn main() {
    let mut sq1 = Square::new(5);
    println!("sq1 area={}", sq1.area());
    sq1.double();
    dbg!(sq1.can_hold(&Square::new(6)));
    let rect0 = sq1.to_rectangle();
    println!("rect0 area={}", rect0.area());

    let mut rect1 = Rectangle::new(30, 50);
    println!("rect1 area={}", rect1.area());
    rect1.double();
    println!("rect1 area={}", rect1.area());
    dbg!(rect0.can_hold(&rect1));
    dbg!(rect1.can_hold(&rect0));

    dbg!((Square::new(0).side(), rect1.width(), rect1.height()));
    dbg!(Rectangle::square(3));
}
