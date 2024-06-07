use parser::{TerminalEnd, TerminalEndImpl};
use std::env;
use std::io;
use std::process::{ExitCode, Termination};
use std::ops::{Neg, Add, Sub, Mul, Div};

fn main() -> impl Termination {
    let mut argv = env::args();
    let argv0 = argv.next().unwrap();
    if argv.len() != 1 {
        return usage(&argv0);
    }
    let argv1 = argv.next().unwrap();
    match argv1.as_str() {
        "i" => run::<i32>(),
        //"u" => run::<u32>(),
        "d" => run::<f64>(),
        //"s" => run::<String>(),
        _ => usage(&argv0),
    }
}

fn usage(argv0: &str) -> ExitCode {
    eprintln!("usage: {} {}", argv0, "{i|u|d|s}

Evaluates an expression read from stdin consisting of:
- values
- unary operator -
- binary operators +, -, *, /
- parentheses
- whitespace (ignored)

Selection of type:

i = i32
u = u32
d = f64
s = str (only binary +, whitespace needed around operators and parentheses)
");
    ExitCode::FAILURE
}

fn run<T: 'static + Clone + Default + PartialEq + std::fmt::Display + std::str::FromStr>() -> ExitCode where
    T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
    TerminalEndImpl<T>: TerminalEnd
{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("Cannot read expression: {error}");
            return ExitCode::FAILURE;
        }
    }
    if let Some(e) = parser::parse::<T>(&input) {
        e.display();
        println!("");
        if let Some(v) = e.eval() {
            println!("{}", v);
        } else {
            println!("no value");
        }
    } else {
        println!("invalid expression");
    }
    ExitCode::SUCCESS
}

mod expr {
    use std::ops::{Neg, Add, Sub, Mul, Div};
    pub enum Op1Kind {
        Minus,
    }
    impl Op1Kind {
        pub fn eval<T>(&self, c: T) -> Option<T> where
            T: Neg<Output = T>
        {
            match *self {
                Op1Kind::Minus => Some(-c),
            }
        }
        pub fn display(&self) {
            match *self {
                Op1Kind::Minus => print!("-"),
            }
        }
    }
    pub enum Op2Kind {
        Add,
        Sub,
        Mul,
        Div,
    }
    impl Op2Kind {
        pub fn eval<T: Default + PartialEq>(&self, l: T, r: T) -> Option<T> where
            T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>
        {
            match *self {
                Op2Kind::Add => Some(l + r),
                Op2Kind::Sub => Some(l - r),
                Op2Kind::Mul => Some(l * r),
                Op2Kind::Div =>
                    if r != T::default() {
                        Some(l / r)
                    } else {
                        None
                    },
            }
        }
        pub fn display(&self) {
            match *self {
                Op2Kind::Add => print!("+"),
                Op2Kind::Sub => print!("-"),
                Op2Kind::Mul => print!("*"),
                Op2Kind::Div => print!("/"),
            }
        }
    }
    pub enum Expr<T> {
        Value {
            v: T,
        },
        Op1 {
            op: Op1Kind,
            child: Box<Expr<T>>
        },
        Op2 {
            op: Op2Kind,
            l: Box<Expr<T>>,
            r: Box<Expr<T>>,
        },
    }
    impl<T: Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr> Expr<T> where
        T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>
    {
        pub fn new_value(v: T) -> Expr<T> {
            Expr::Value { v }
        }
        pub fn new_op1(op: Op1Kind, child: Box<Expr<T>>) -> Expr<T> {
            Expr::Op1 { op, child, }
        }
        pub fn new_op2(op: Op2Kind, l: Box<Expr<T>>, r: Box<Expr<T>>) -> Expr<T> {
            Expr::Op2 { op, l, r, }
        }
        pub fn eval(&self) -> Option<T> {
            match self {
                Expr::Value{v} => Some((*v).clone()),
                Expr::Op1{op, child} =>
                    if let Some(cv) = child.eval() {
                        op.eval(cv)
                    } else {
                        None
                    },
                Expr::Op2{op, l, r} =>
                    if let (Some(lv), Some(rv)) = (l.eval(), r.eval()) {
                        op.eval(lv, rv)
                    } else {
                        None
                    }
            }
        }
        pub fn display(&self) {
            print!("(");
            match self {
                Expr::Value{v} => print!("{}", v),
                Expr::Op1{op, child} => {
                    op.display();
                    child.display();
                }
                Expr::Op2{op, l, r} => {
                    l.display();
                    op.display();
                    r.display();
                }
            }
            print!(")");
        }
    }
}

mod parser {
    use std::ops::{Neg, Add, Sub, Mul, Div};
    use super::expr::{Op1Kind, Op2Kind, Expr};

    pub fn parse<T: 'static + Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr>(s: &str) ->
        Option<Box<Expr<T>>>
        where T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
        TerminalEndImpl<T>: TerminalEnd
    {
        expression::<T>(s.trim()).0
    }

    fn expression<T: 'static + Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr>(s: &str) ->
        (Option<Box<Expr<T>>>, &str)
        where T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
        TerminalEndImpl<T>: TerminalEnd
    {
        if let (Some(mut t1), mut s) = term::<T>(s) {
            loop {
                s = s.trim_start();
                if s.is_empty() {
                    return (Some(t1), &s);
                }
                let op: Op2Kind;
                match s.chars().next().unwrap() {
                    '+' => op = Op2Kind::Add,
                    '-' => op = Op2Kind::Sub,
                    _ => return (Some(t1), &s)
                }
                s = &s[1..];
                if let(Some(t2), s2) = term::<T>(s) {
                    s = &s2;
                    t1 = Box::new(Expr::new_op2(op, t1, t2));
                } else {
                    return (None, s)
                }
            }
        } else {
            (None, &s)
        }
    }

    fn term<T: 'static + Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr>(s: &str) ->
        (Option<Box<Expr<T>>>, &str)
        where T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
        TerminalEndImpl<T>: TerminalEnd
    {
        if let (Some(mut f1), mut s) = factor::<T>(s) {
            loop {
                s = s.trim_start();
                if s.is_empty() {
                    return (Some(f1), &s);
                }
                let op: Op2Kind;
                match s.chars().next().unwrap() {
                    '*' => op = Op2Kind::Mul,
                    '/' => op = Op2Kind::Div,
                    _ => return (Some(f1), &s)
                }
                s = &s[1..];
                if let(Some(f2), s2) = factor::<T>(s) {
                    s = &s2;
                    f1 = Box::new(Expr::new_op2(op, f1, f2));
                } else {
                    return (None, s);
                }
            }
        } else {
            (None, &s)
        }
    }

    fn factor<T: 'static + Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr>(s: &str) ->
        (Option<Box<Expr<T>>>, &str)
        where T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
        TerminalEndImpl<T>: TerminalEnd
    {
        let mut s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let mut m: Option<Op1Kind> = None;
        match s.chars().next().unwrap() {
            '-' => {
                m = Some(Op1Kind::Minus);
                s = &s[1..];
            }
            _ => {}
        }
        s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let mut e: Option<Box<Expr<T>>>;
        match s.chars().next().unwrap() {
            '(' => {
                s = &s[1..];
                (e, s) = expression::<T>(s);
                if e.is_some() {
                    s = s.trim_start();
                    if s.is_empty() {
                        return (None, s);
                    }
                    match s.chars().next().unwrap() {
                        ')' => s = &s[1..],
                        _ => return (None, s),
                    }
                }
            }
            _ => (e, s) = terminal::<T>(s),
        }
        if let Some(op) = m {
            e = Some(Box::new(Expr::new_op1(op, e.unwrap())));
        }
        (e, s)
    }

    fn terminal<T: 'static + Default + Clone + PartialEq + std::fmt::Display + std::str::FromStr>(s: &str) ->
        (Option<Box<Expr<T>>>, &str)
        where T: Neg<Output = T>, T: Add<Output = T>, T: Sub<Output = T>, T: Mul<Output = T>, T: Div<Output = T>,
        TerminalEndImpl<T>: TerminalEnd
    {
        let s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let i = s.find(TerminalEndImpl::<T>::pattern).unwrap_or(s.len());
        if i == 0 {
            return (None, s)
        }
        if let Ok(v) = s[..i].parse::<T>() {
            (Some(Box::new(Expr::new_value(v))), &s[i..])
        } else {
            (None, s)
        }
    }

    pub trait TerminalEnd {
        fn pattern(c: char) -> bool;
    }
    pub struct TerminalEndImpl<T> {
        dummy: std::marker::PhantomData<T>,
    }
    fn pattern_int(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    impl TerminalEnd for TerminalEndImpl<i32> {
        fn pattern(c: char) -> bool {
            !pattern_int(c)
        }
    }
    impl TerminalEnd for TerminalEndImpl<u32> {
        fn pattern(c: char) -> bool {
            !pattern_int(c)
        }
    }
    impl TerminalEnd for TerminalEndImpl<f64> {
        fn pattern(c: char) -> bool {
            !(pattern_int(c) || c == '.')
        }
    }
    impl TerminalEnd for TerminalEndImpl<String> {
        fn pattern(c: char) -> bool {
            c == ' '
        }
    }
}
