use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    val: T,
    next: RefCell<Option<Rc<Node<T>>>>,
    prev: RefCell<Option<Rc<Node<T>>>>,
}

struct LinkedList<T> {
    first: Option<Rc<Node<T>>>,
    last: Option<Rc<Node<T>>>,
}
impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList{
            first: None,
            last: None,
        }
    }
    fn append(&mut self, val: T) {
        let node = Rc::new(Node {
            val: val,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });
        match &self.last {
            None => {
                self.first = Some(node.clone());
            },
            Some(pnode) => {
                *pnode.next.borrow_mut() = Some(node.clone());
                *node.prev.borrow_mut() = Some(pnode.clone());
            },
        }
        self.last = Some(node);
    }
    fn prepend(&mut self, val: T) {
        let node = Rc::new(Node {
            val: val,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });
        match &self.first {
            None => {
                self.last = Some(node.clone());
            },
            Some(pnode) => {
                *pnode.prev.borrow_mut() = Some(node.clone());
                *node.next.borrow_mut() = Some(pnode.clone());
            },
        }
        self.first = Some(node);
    }
}
impl LinkedList<String> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        match &self.first {
            None => {},
            Some(rc) => {
                let mut pnode = rc.clone();
                let mut pnode2;
                loop {
                    s.push_str(&pnode.val);
                    {
                        let pnext = pnode.next.borrow();
                        match &*pnext {
                            None => break,
                            Some(rc) => pnode2 = rc.clone(),
                        }
                    }
                    pnode = pnode2;
                }
            },
        }
        s
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("world"));
    list.append(String::from("!"));
    list.prepend(String::from(" "));
    list.prepend(String::from("Hello"));
    println!("{}", list.to_string());
}
