use std::rc::{Rc, Weak};

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
                }
            };
        }
        s
    }
}

enum RcList<T> {
    Null,
    Node(T, Rc<RcList<T>>),
}
impl RcList<String> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut p = self;
        loop {
            match p {
                RcList::Null => break,
                RcList::Node(v, l) => {
                    s.push_str(v);
                    p = l;
                }
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

struct DropSmartPtr {
    data: String,
}
impl Drop for DropSmartPtr {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let mut list = BoxList::Node(
        String::from("Hello"),
        Box::new(BoxList::Node(
            String::from(" "),
            Box::new(BoxList::Node(
                String::from("world"),
                Box::new(BoxList::Node(String::from("!"), Box::new(BoxList::Null))),
            )),
        )),
    );
    println!("list={}", list.to_string());
    let mut p = &list;
    for _ in 0..2 {
        match p {
            BoxList::Null => {}
            BoxList::Node(_, pp) => p = pp,
        }
    }
    println!("p={}", p.to_string());
    let mut p = &mut list;
    for _ in 0..3 {
        match p {
            BoxList::Null => {}
            BoxList::Node(_, pp) => p = pp,
        }
    }
    *p = BoxList::Null;
    println!("list={}", list.to_string());

    let mut list = Rc::new(RcList::<String>::Null);
    list = Rc::new(RcList::Node(String::from("!"), list));
    list = Rc::new(RcList::Node(String::from("worlds"), list));
    list = Rc::new(RcList::Node(String::from(" "), list));
    list = Rc::new(RcList::Node(String::from("Hi"), list));
    let list2 = list.clone();
    println!("rc_list={}", list.to_string());
    println!("rc_list2={}", list2.to_string());
    let mut p = &*list;
    let mut p2 = &*list2;
    if let RcList::Node(_, pp) = p {
        p = pp;
    }
    if let RcList::Node(_, pp) = p2 {
        p2 = pp;
    }
    if let RcList::Node(_, pp) = p {
        p = pp;
    }
    println!("rc_list={}", p.to_string());
    println!("rc_list2={}", p2.to_string());
    println!("rc count strong={} weak={}", Rc::strong_count(&list), Rc::weak_count(&list));
    let w = Rc::downgrade(&list2);
    println!("rc count strong={} weak={}", Rc::strong_count(&list), Rc::weak_count(&list));
    let strong = Weak::upgrade(&w);
    println!("rc count strong={} weak={}", Rc::strong_count(&list), Rc::weak_count(&list));

    let x = 5;
    let y = MyBox::new(x);
    assert!(x == 5);
    assert!(*y == 5);

    let mut o2a: DropSmartPtr;
    let mut o3a: DropSmartPtr;
    {
        let _o1 = DropSmartPtr {
            data: String::from("o1"),
        };
        let mut o2 = DropSmartPtr {
            data: String::from("o2"),
        };
        let o3 = DropSmartPtr {
            data: String::from("o3"),
        };
        let o4 = DropSmartPtr {
            data: String::from("o4"),
        };
        o2a = o2;
        o3a = o3;
        o2 = DropSmartPtr {
            data: String::from("o2b"),
        };
        o2a.data.push_str(" o2a");
        o3a.data.push_str(" o3a");
        drop(o4);
        println!("In scope");
    }
    println!("After scope");
}
