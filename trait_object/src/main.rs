trait Shape {
    fn draw(&self) {
        println!("Shape");
    }
}

struct Point {
    x: f64,
    y: f64,
}
impl Shape for Point {
    fn draw(&self) {
        println!("Point x={} y={}", self.x, self.y);
    }
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

fn draw_shape(s: &dyn Shape) {
    s.draw();
}

fn main() {
    let p = Point{x: 1.2, y:2.3};
    let c = Circle{x: -1.2, y: -2.3, r: 4.5};
    let v: Vec<Box<dyn Shape>> = vec![Box::new(p), Box::new(c)];
    for s in v {
        draw_shape(&*s);
        s.draw();
    }
}
