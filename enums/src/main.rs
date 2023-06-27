#[derive(Debug)]
enum IpAddr {
    None,
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn to_string(&self) -> String {
        String::from(match self {
            IpAddr::None => "None".to_string(),
            IpAddr::V4(b1, b2, b3, b4) => format!("{}.{}.{}.{}", b1, b2, b3, b4),
            IpAddr::V6(v) => v.clone(),
        })
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    NotImplemented1,
    NotImplemented2{a: usize, b: i32},
    NotImplemented3(String),
    NotImplemented4(i8, i16, i32, i64),
}

impl Message {
    fn process(&self) -> bool {
        match self {
            Message::Quit => {
                println!("Quit");
                false
            }
            Message::Move { x, y } => {
                println!("Move to {}, {}", x, y);
                true
            }
            Message::Write(_) => {
                println!("Write");
                true
            }
            Message::ChangeColor(_, _, _) => {
                println!("Change color");
                true
            }
            _ => false
        }
    }
}

fn main() {
    let ip0 = IpAddr::None;
    let ip4 = IpAddr::V4(127, 0, 0, 1);
    let ip6 = IpAddr::V6(String::from("::1"));
    for ip in [&ip0, &ip4, &ip6] {
        println!("ip={}", ip.to_string());
    }

    println!("{}", Message::Quit.process());
    println!("{}", Message::Move { x: 1, y: 2 }.process());
    println!("{}", Message::Write(String::from("abc")).process());
    println!("{}", Message::ChangeColor(1, 2, 3).process());
    println!("{}", Message::NotImplemented4(1, 2, 3, 4).process());

    let a = 10;
    for b in -10..10 {
        match divide(a, b) {
            None => {}
            Some(c) => println!("{a}/{b}={c}")
        }
        if let Some(mut c) = divide(a, b) {
            c = -c;
            println!("neg={c}");
        } else {
            println!("Division by zero")
        }
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
