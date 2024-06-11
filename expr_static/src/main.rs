use parser::TerminalEnd;
use std::env;
use std::io;
use std::process::{ExitCode, Termination};
use ops::*;
use std::fmt::Display;
use std::str::FromStr;

fn main() -> impl Termination {
    let mut argv = env::args();
    let argv0 = argv.next().unwrap();
    if argv.len() != 1 {
        return usage(&argv0);
    }
    let argv1 = argv.next().unwrap();
    match argv1.as_str() {
        "i" => run::<i32>(),
        "u" => run::<u32>(),
        "d" => run::<f64>(),
        "s" => run::<String>(),
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

pub trait TBound<T>: 'static + Clone + Default + PartialEq + Display + FromStr + Ops<T> + TerminalEnd {}
impl<T> TBound<T> for T where
    T: 'static + Clone + Default + PartialEq + Display + FromStr + Ops<T> + TerminalEnd {}

fn run<T: TBound<T>>() -> ExitCode {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("Cannot read expression: {error}");
            return ExitCode::FAILURE;
        }
    }
    if let Some(e) = parser::parse::<T>(&input) {
        println!("{}", e);
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

mod ops {
    pub trait Ops<T> {
        fn neg(&self) -> Option<T>;
        fn add(&self, r: &T) -> Option<T>;
        fn sub(&self, r: &T) -> Option<T>;
        fn mul(&self, r: &T) -> Option<T>;
        fn div(&self, r: &T) -> Option<T>;
    }
    impl Ops<i32> for i32 {
        fn neg(&self) -> Option<i32> {
            Some(-self)
        }
        fn add(&self, r: &i32) -> Option<i32> {
            Some(self + *r)
        }
        fn sub(&self, r: &i32) -> Option<i32> {
            Some(self - *r)
        }
        fn mul(&self, r: &i32) -> Option<i32> {
            Some(self * *r)
        }
        fn div(&self, r: &i32) -> Option<i32> {
            Some(self / *r)
        }
    }
    impl Ops<u32> for u32 {
        fn neg(&self) -> Option<u32> {
            None
        }
        fn add(&self, r: &u32) -> Option<u32> {
            Some(self + *r)
        }
        fn sub(&self, r: &u32) -> Option<u32> {
            Some(self - *r)
        }
        fn mul(&self, r: &u32) -> Option<u32> {
            Some(self * *r)
        }
        fn div(&self, r: &u32) -> Option<u32> {
            Some(self / *r)
        }
    }
    impl Ops<f64> for f64 {
        fn neg(&self) -> Option<f64> {
            Some(-self)
        }
        fn add(&self, r: &f64) -> Option<f64> {
            Some(self + *r)
        }
        fn sub(&self, r: &f64) -> Option<f64> {
            Some(self - *r)
        }
        fn mul(&self, r: &f64) -> Option<f64> {
            Some(self * *r)
        }
        fn div(&self, r: &f64) -> Option<f64> {
            Some(self / *r)
        }
    }
    impl Ops<String> for String {
        fn neg(&self) -> Option<String> {
            None
        }
        fn add(&self, r: &String) -> Option<String> {
            Some((self).clone() + &*r)
        }
        fn sub(&self, _: &String) -> Option<String> {
            None
        }
        fn mul(&self, _: &String) -> Option<String> {
            None
        }
        fn div(&self, _: &String) -> Option<String> {
            None
        }
    }
}

mod expr {
    use crate::TBound;
    use crate::ops::*;
    use std::fmt::{Display, Error, Formatter};

    pub enum Op1Kind {
        Minus,
    }
    impl Op1Kind {
        pub fn eval<T: Ops<T>>(&self, c: T) -> Option<T> {
            match *self {
                Op1Kind::Minus => Ops::<T>::neg(&c),
            }
        }
    }
    impl Display for Op1Kind {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self {
                Op1Kind::Minus => write!(f, "-"),
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
        pub fn eval<T: Default + PartialEq + Ops<T>>(&self, l: T, r: T) -> Option<T> {
            match *self {
                Op2Kind::Add => Ops::<T>::add(&l, &r),
                Op2Kind::Sub => Ops::<T>::sub(&l, &r),
                Op2Kind::Mul => Ops::<T>::mul(&l, &r),
                Op2Kind::Div =>
                    if r != T::default() {
                        Ops::<T>::div(&l, &r)
                    } else {
                        None
                    },
            }
        }
    }
    impl Display for Op2Kind {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self {
                Op2Kind::Add => write!(f, "+"),
                Op2Kind::Sub => write!(f, "-"),
                Op2Kind::Mul => write!(f, "*"),
                Op2Kind::Div => write!(f, "/"),
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
    impl<T: TBound<T>> Expr<T> {
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
    }
    impl<T: Display> Display for Expr<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self {
                Expr::Value{v} => write!(f, "({v})"),
                Expr::Op1{op, child} => write!(f, "({op}{child})"),
                Expr::Op2{op, l, r} => write!(f, "({l}{op}{r})"),
            }
        }
    }
}

mod parser {
    use crate::TBound;
    use super::expr::{Op1Kind, Op2Kind, Expr};

    pub fn parse<T: TBound<T>>(s: &str) -> Option<Expr<T>> {
        expression::<T>(s.trim()).0
    }

    fn expression<T: TBound<T>>(s: &str) -> (Option<Expr<T>>, &str) {
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
                    t1 = Expr::Op2 {op, l: Box::new(t1), r: Box::new(t2)};
                } else {
                    return (None, s)
                }
            }
        } else {
            (None, &s)
        }
    }

    fn term<T: TBound<T>>(s: &str) -> (Option<Expr<T>>, &str) {
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
                    f1 = Expr::Op2 {op, l: Box::new(f1), r: Box::new(f2)};
                } else {
                    return (None, s);
                }
            }
        } else {
            (None, &s)
        }
    }

    fn factor<T: TBound<T>>(s: &str) -> (Option<Expr<T>>, &str) {
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
        let mut e: Option<Expr<T>>;
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
            e = Some(Expr::Op1 {op, child: Box::new(e.unwrap())});
        }
        (e, s)
    }

    fn terminal<T: TBound<T>>(s: &str) -> (Option<Expr<T>>, &str) {
        let s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let i = s.find(T::pattern).unwrap_or(s.len());
        if i == 0 {
            return (None, s)
        }
        if let Ok(v) = s[..i].parse::<T>() {
            (Some(Expr::Value {v}), &s[i..])
        } else {
            (None, s)
        }
    }

    pub trait TerminalEnd {
        fn pattern(c: char) -> bool;
    }
    fn pattern_int(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    impl TerminalEnd for i32 {
        fn pattern(c: char) -> bool {
            !pattern_int(c)
        }
    }
    impl TerminalEnd for u32 {
        fn pattern(c: char) -> bool {
            !pattern_int(c)
        }
    }
    impl TerminalEnd for f64 {
        fn pattern(c: char) -> bool {
            !(pattern_int(c) || c == '.')
        }
    }
    impl TerminalEnd for String {
        fn pattern(c: char) -> bool {
            c == ' '
        }
    }
}
