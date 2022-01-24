use std::rc::Rc;

struct Data<'a> {
    i: i32,
    r: &'a i32,
}

struct Data2 {
    i: i32,
}

fn pc<T>(p: Rc<T>) -> Rc<T> {
    return p;
}

fn main() {
    let i = 11;
    let mut o: Data = Data{i:22, r:&i};
    o.i = 1;
    {
        let j = 12;
        o.r = &j;
        println!("o: i={} r={}", o.i, o.r);
    }
    let mut p2: Rc<Data> = Rc::new(Data{i:0, r:&i});
    {
        let mi = 123;
        let p = Rc::new(Data{i:1, r:&i});
        p2 = pc(p);
        //p2 = p;
        //(*p2).r = &i;
        println!("v: i={} r={}", (*p2).i, (*p2).r);
        p2 = Rc::new(Data{i:2, r:&i});
    }
    println!("v: i={} r={}", (*p2).i, (*p2).r);
}
