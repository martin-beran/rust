trait Shape {
    fn draw(&self) {
        println!("Shape");
    }
}

trait Num {
    fn val(&self) -> f64 {
        0.0
    }
}

trait NumShape: Shape + Num {}

struct Point {
    x: f64,
    y: f64,
}
impl Shape for Point {
    fn draw(&self) {
        println!("Point x={} y={}", self.x, self.y);
    }
}
impl Num for Point {
    fn val(&self) -> f64 {
        self.x + self.y
    }
}
impl NumShape for Point {
}

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}
impl Shape for Circle {
    fn draw(&self) {
        println!("Circle x={} y={} r={}", self.x, self.y, self.r);
    }
}
impl Num for Circle {
    fn val(&self) -> f64 {
        self.x + self.y + self.r
    }
}
impl NumShape for Circle {
}

fn draw_shape(s: &dyn NumShape) {
    s.draw();
}

fn main() {
    let p = Point{x: 1.2, y:2.3};
    let c = Circle{x: -1.2, y: -2.3, r: 4.5};
    let v: Vec<Box<dyn NumShape>> = vec![Box::new(p), Box::new(c)];
    for s in v {
        draw_shape(&*s);
        println!("val={}", s.val());
    }
}
