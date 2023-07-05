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

fn main() {
    let list = BoxList::Node(String::from("Hello"),
        Box::new(BoxList::Node(String::from(" "),
            Box::new(BoxList::Node(String::from("world"),
                Box::new(BoxList::Node(String::from("!"),
                    Box::new(BoxList::Null)
                ))
            ))
        ))
    );
    println!("list={}", list.to_string());
}
