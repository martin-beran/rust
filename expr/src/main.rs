use crate::value::Value;
use crate::op_minus::OpMinus;
use std::env;
use std::process::{ExitCode, Termination};

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
        "s" => run::<&str>(),
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

fn run<T>() -> ExitCode {
    let v: Box<dyn Expr<i32>> = Box::new(Value::new(1));
    let m: Box<dyn Expr<i32>> = Box::new(OpMinus::new(v));
    m.display();
    println!("");
    match m.eval() {
        None => println!("no value"),
        Some(r) => println!("{}", r),
    }
    ExitCode::SUCCESS
}

trait Expr<T> {
    fn eval(&self) -> Option<T>;
    fn display(&self);
}

mod value {
    use crate::Expr;

    pub struct Value<T> {
        v: T,
    }
    impl<T> Value<T> {
        pub fn new(v: T) -> Value<T> {
            Value {
                v,
            }
        }
    }
    impl<T: Clone + std::fmt::Display> Expr<T> for Value<T> {
        fn eval(&self) -> Option<T> {
            Some(self.v.clone())
        }
        fn display(&self) {
            print!("({})", self.v)
        }
    }
}

trait Op1<T>: Expr<T> {
    fn op1_eval(&self) -> Option<T> {
        self.eval_op(&self.child().eval())
    }
    fn op1_display(&self) {
        print!("(");
        self.display_op();
        self.child().display();
        print!(")");
    }
    fn child(&self) -> &dyn Expr<T>;
    fn eval_op(&self, v: &Option::<T>) -> Option<T>;
    fn display_op(&self);
}

mod op_minus {
    use crate::Expr;
    use crate::Op1;
    use std::ops::Neg;

    pub struct OpMinus<T: Clone + Neg<Output = T>> {
        child: Box<dyn Expr<T>>,
    }
    impl<T: Clone + Neg<Output = T>> OpMinus<T> {
        pub fn new(child: Box<dyn Expr<T>>) -> OpMinus<T> {
            OpMinus {
                child,
            }
        }
    }
    impl<T: Clone + Neg<Output = T>> Expr<T> for OpMinus<T> {
        fn eval(&self) -> Option<T> {
            self.op1_eval()
        }
        fn display(&self) {
            self.op1_display()
        }
    }
    impl<T: Clone + Neg<Output = T>> Op1<T> for OpMinus<T> {
        fn child(&self) -> &dyn Expr<T> {
            self.child.as_ref()
        }
        fn eval_op(&self, v: &Option::<T>) -> Option<T> {
            match v {
                None => None,
                Some(n) => Some(-n.clone()),
            }
        }
        fn display_op(&self) {
            print!("-");
        }
    }
}
