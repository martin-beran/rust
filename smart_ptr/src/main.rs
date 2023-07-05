enum BoxList<T> {
    Null,
    Node(T, Box<BoxList<T>>),
}
impl BoxList<String> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut p = self;
        loop {
            match p {
                BoxList::Null => break,
                BoxList::Node(v, l) => {
                    s.push_str(v);
                    p = l;
                },
            };
        }
        s
    }
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let mut list = BoxList::Node(String::from("Hello"),
        Box::new(BoxList::Node(String::from(" "),
            Box::new(BoxList::Node(String::from("world"),
                Box::new(BoxList::Node(String::from("!"),
                    Box::new(BoxList::Null)
                ))
            ))
        ))
    );
    println!("list={}", list.to_string());
    let mut p = &list;
    for _ in 0..2 {
        match p {
            BoxList::Null => {},
            BoxList::Node(_, pp) => p = pp,
        }
    }
    println!("p={}", p.to_string());
    let mut p = &mut list;
    for _ in 0..3 {
        match p {
            BoxList::Null => {},
            BoxList::Node(_, pp) => p = pp,
        }
    }
    *p = BoxList::Null;
    println!("list={}", list.to_string());

    let x = 5;
    let y = MyBox::new(x);
    assert!(x == 5);
    assert!(*y == 5);
}
