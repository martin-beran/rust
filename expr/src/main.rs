use expr::{op1::Op1Common, op_minus::OpMinus};
use expr::{op2::Op2Common, op_add::OpAdd, op_sub::OpSub, op_mul::OpMul, op_div::OpDiv};
use std::env;
use std::io;
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

fn run<T: std::fmt::Display>() -> ExitCode where
    OpMinus<T>: Op1Common<T>,
    OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
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
    pub trait Expr<T> {
        fn eval(&self) -> Option<T>;
        fn display(&self);
    }

    pub mod value {
        use super::Expr;

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

    pub mod op1 {
        use super::Expr;

        pub(super) trait Op1<T>: Expr<T> + Op1Common<T> {
            type Eval: Op1Evaluator<T>;
            fn op1_eval(&self) -> Option<T> {
                Self::Eval::eval_op(&self.child().eval())
            }
            fn op1_display(&self) {
                print!("(");
                self.display_op();
                self.child().display();
                print!(")");
            }
        }
        pub trait Op1Common<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>>;
            fn child(&self) -> &dyn Expr<T>;
            fn set_child(&mut self, c: Box<dyn Expr<T>>);
            fn display_op(&self);
        }
        pub(super) trait Op1Evaluator<T> {
            fn eval_op(v: &Option<T>) -> Option<T>;
        }
    }

    pub mod op_minus {
        use super::Expr;
        use super::op1::{Op1, Op1Common, Op1Evaluator};

        pub struct OpMinus<T> {
            child: Option<Box<dyn Expr<T>>>,
        }
        impl<T> OpMinus<T> {
            pub fn new() -> OpMinus<T> {
                OpMinus {
                    child: None,
                }
            }
        }
        impl<T> Expr<T> for OpMinus<T> where OpMinus<T>: Op1<T> {
            fn eval(&self) -> Option<T> {
                self.op1_eval()
            }
            fn display(&self) {
                self.op1_display()
            }
        }
        pub(super) struct OpWithMinus;
        impl<T: Copy + std::ops::Neg<Output = T>> Op1Evaluator<T> for OpWithMinus {
            fn eval_op(v: &Option<T>) -> Option<T> {
                match v {
                    None => None,
                    Some(n) => Some(-*n),
                }
            }
        }
        pub(super) struct OpNoMinus;
        impl<T> Op1Evaluator<T> for OpNoMinus {
            fn eval_op(_: &Option<T>) -> Option<T> {
                None
            }
        }
        impl Op1<i32> for OpMinus<i32> { type Eval = OpWithMinus; }
        impl Op1<u32> for OpMinus<u32> { type Eval = OpNoMinus; }
        impl Op1<f64> for OpMinus<f64> { type Eval = OpWithMinus; }
        impl Op1<String> for OpMinus<String> { type Eval = OpNoMinus; }
        impl<T: 'static> Op1Common<T> for OpMinus<T> where OpMinus<T>: Op1<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>> {
                self
            }
            fn child(&self) -> &dyn Expr<T> {
                self.child.as_ref().unwrap().as_ref()
            }
            fn set_child(&mut self, c: Box<dyn Expr<T>>) {
                self.child = Some(c);
            }
            fn display_op(&self) {
                print!("-");
            }
        }
    }

    pub mod op2 {
        use super::Expr;

        pub(super) trait Op2<T>: Expr<T> + Op2Common<T> {
            type Eval: Op2Evaluator<T>;
            fn op2_eval(&self) -> Option<T> {
                Self::Eval::eval_op(&self.left().eval(), &self.right().eval())
            }
            fn op2_display(&self) {
                print!("(");
                self.left().display();
                self.display_op();
                self.right().display();
                println!(")");
            }
        }
        // https://articles.bchlr.de/traits-dynamic-dispatch-upcasting
        pub trait Op2Common<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>>;
            fn left(&self) -> &dyn Expr<T>;
            fn set_left(&mut self, l: Box<dyn Expr<T>>);
            fn right(&self) -> &dyn Expr<T>;
            fn set_right(&mut self, l: Box<dyn Expr<T>>);
            fn display_op(&self);
        }
        pub(super) trait Op2Evaluator<T> {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T>;
        }
    }

    pub mod op_add {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};

        pub struct OpAdd<T> {
            left: Option<Box<dyn Expr<T>>>,
            right: Option<Box<dyn Expr<T>>>,
        }
        impl<T> OpAdd<T> {
            pub fn new() -> OpAdd<T> {
                OpAdd {
                    left: None,
                    right: None,
                }
            }
        }
        impl<T> Expr<T> for OpAdd<T> where OpAdd<T>: Op2<T> {
            fn eval(&self) -> Option<T> {
                self.op2_eval()
            }
            fn display(&self) {
                self.op2_display()
            }
        }
        pub(super) struct OpWithAdd;
        impl<T: Copy + std::ops::Add<Output = T>> Op2Evaluator<T> for OpWithAdd {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T> {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(*a + *b)
                } else {
                    None
                }
            }
        }
        pub(super) struct OpStringAdd;
        impl Op2Evaluator<String> for OpStringAdd {
            fn eval_op(a: &Option<String>, b: &Option<String>) -> Option<String> {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a.clone() + b)
                } else {
                    None
                }
            }
        }
        impl Op2<i32> for OpAdd<i32> { type Eval = OpWithAdd; }
        impl Op2<u32> for OpAdd<u32> { type Eval = OpWithAdd; }
        impl Op2<f64> for OpAdd<f64> { type Eval = OpWithAdd; }
        impl Op2<String> for OpAdd<String> { type Eval = OpStringAdd; }
        impl<T: 'static> Op2Common<T> for OpAdd<T> where OpAdd<T>: Op2<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>> {
                self
            }
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref().unwrap().as_ref()
            }
            fn set_left(&mut self, l: Box<dyn Expr<T>>) {
                self.left = Some(l);
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref().unwrap().as_ref()
            }
            fn set_right(&mut self, l: Box<dyn Expr<T>>) {
                self.right = Some(l);
            }
            fn display_op(&self) {
                print!("+");
            }
        }
    }

    pub mod op_sub {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};

        pub struct OpSub<T> {
            left: Option<Box<dyn Expr<T>>>,
            right: Option<Box<dyn Expr<T>>>,
        }
        impl<T> OpSub<T> {
            pub fn new() -> OpSub<T> {
                OpSub {
                    left: None,
                    right: None,
                }
            }
        }
        impl<T> Expr<T> for OpSub<T> where OpSub<T>: Op2<T> {
            fn eval(&self) -> Option<T> {
                self.op2_eval()
            }
            fn display(&self) {
                self.op2_display()
            }
        }
        pub(super) struct OpWithSub;
        impl<T: Copy + std::ops::Sub<Output = T>> Op2Evaluator<T> for OpWithSub {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T> {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(*a - *b)
                } else {
                    None
                }
            }
        }
        pub(super) struct OpNoOp2;
        impl Op2Evaluator<String> for OpNoOp2 {
            fn eval_op(_: &Option<String>, _: &Option<String>) -> Option<String> {
                None
            }
        }
        impl Op2<i32> for OpSub<i32> { type Eval = OpWithSub; }
        impl Op2<u32> for OpSub<u32> { type Eval = OpWithSub; }
        impl Op2<f64> for OpSub<f64> { type Eval = OpWithSub; }
        impl Op2<String> for OpSub<String> { type Eval = OpNoOp2; }
        impl<T: 'static> Op2Common<T> for OpSub<T> where OpSub<T>: Op2<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>> {
                self
            }
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref().unwrap().as_ref()
            }
            fn set_left(&mut self, l: Box<dyn Expr<T>>) {
                self.left = Some(l);
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref().unwrap().as_ref()
            }
            fn set_right(&mut self, l: Box<dyn Expr<T>>) {
                self.right = Some(l);
            }
            fn display_op(&self) {
                print!("-");
            }
        }
    }

    pub mod op_mul {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};
        use super::op_sub::OpNoOp2;

        pub struct OpMul<T> {
            left: Option<Box<dyn Expr<T>>>,
            right: Option<Box<dyn Expr<T>>>,
        }
        impl<T> OpMul<T> {
            pub fn new() -> OpMul<T> {
                OpMul {
                    left: None,
                    right: None,
                }
            }
        }
        impl<T> Expr<T> for OpMul<T> where OpMul<T>: Op2<T> {
            fn eval(&self) -> Option<T> {
                self.op2_eval()
            }
            fn display(&self) {
                self.op2_display()
            }
        }
        pub(super) struct OpWithMul;
        impl<T: Copy + std::ops::Mul<Output = T>> Op2Evaluator<T> for OpWithMul {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T> {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(*a * *b)
                } else {
                    None
                }
            }
        }
        impl Op2<i32> for OpMul<i32> { type Eval = OpWithMul; }
        impl Op2<u32> for OpMul<u32> { type Eval = OpWithMul; }
        impl Op2<f64> for OpMul<f64> { type Eval = OpWithMul; }
        impl Op2<String> for OpMul<String> { type Eval = OpNoOp2; }
        impl<T: 'static> Op2Common<T> for OpMul<T> where OpMul<T>: Op2<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>> {
                self
            }
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref().unwrap().as_ref()
            }
            fn set_left(&mut self, l: Box<dyn Expr<T>>) {
                self.left = Some(l);
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref().unwrap().as_ref()
            }
            fn set_right(&mut self, l: Box<dyn Expr<T>>) {
                self.right = Some(l);
            }
            fn display_op(&self) {
                print!("*");
            }
        }
    }

    pub mod op_div {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};
        use super::op_sub::OpNoOp2;

        pub struct OpDiv<T> {
            left: Option<Box<dyn Expr<T>>>,
            right: Option<Box<dyn Expr<T>>>,
        }
        impl<T> OpDiv<T> {
            pub fn new() -> OpDiv<T> {
                OpDiv {
                    left: None,
                    right: None,
                }
            }
        }
        impl<T> Expr<T> for OpDiv<T> where OpDiv<T>: Op2<T> {
            fn eval(&self) -> Option<T> {
                self.op2_eval()
            }
            fn display(&self) {
                self.op2_display()
            }
        }
        pub(super) struct OpWithDiv;
        impl<T: Copy + std::ops::Div<Output = T> + Default + PartialEq> Op2Evaluator<T> for OpWithDiv {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T> {
                if let (Some(a), Some(b)) = (a, b) {
                    if *b == T::default() {
                        None
                    } else {
                        Some(*a / *b)
                    }
                } else {
                    None
                }
            }
        }
        impl Op2<i32> for OpDiv<i32> { type Eval = OpWithDiv; }
        impl Op2<u32> for OpDiv<u32> { type Eval = OpWithDiv; }
        impl Op2<f64> for OpDiv<f64> { type Eval = OpWithDiv; }
        impl Op2<String> for OpDiv<String> { type Eval = OpNoOp2; }
        impl<T: 'static> Op2Common<T> for OpDiv<T> where OpDiv<T>: Op2<T> {
            fn as_dyn_expr(self: Box<Self>) -> Box<dyn Expr<T>> {
                self
            }
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref().unwrap().as_ref()
            }
            fn set_left(&mut self, l: Box<dyn Expr<T>>) {
                self.left = Some(l);
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref().unwrap().as_ref()
            }
            fn set_right(&mut self, l: Box<dyn Expr<T>>) {
                self.right = Some(l);
            }
            fn display_op(&self) {
                print!("/");
            }
        }
    }
}

mod parser {
    use super::expr::Expr;
    use super::expr::{op1::Op1Common, op2::Op2Common};
    use super::expr::{value::Value, op_minus::OpMinus};
    use super::expr::{op_add::OpAdd, op_sub::OpSub, op_mul::OpMul, op_div::OpDiv};

    pub fn parse<T>(s: &str) -> Option<Box<dyn Expr<T>>> where
        OpMinus<T>: Op1Common<T>,
        OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
    {
        expression::<T>(s.trim()).0
    }

    fn expression<T>(s: &str) -> (Option<Box<dyn Expr<T>>>, &str) where
        OpMinus<T>: Op1Common<T>,
        OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
    {
        if let (Some(mut t1), mut s) = term::<T>(s) {
            loop {
                s = s.trim_start();
                if s.is_empty() {
                    return (Some(t1), &s);
                }
                let mut op: Option<Box<dyn Op2Common<T>>>;
                match s.chars().next().unwrap() {
                    '+' => op = Some(Box::new(OpAdd::<T>::new())),
                    '-' => op = Some(Box::new(OpSub::<T>::new())),
                    _ => return (Some(t1), &s)
                }
                s = &s[1..];
                if let(Some(t2), s2) = term::<T>(s) {
                    s = &s2;
                    op.as_deref_mut().unwrap().set_left(t1);
                    op.as_deref_mut().unwrap().set_right(t2);
                    t1 = op.unwrap().as_dyn_expr();
                } else {
                    return (None, s)
                }
            }
        } else {
            (None, &s)
        }
    }

    fn term<T>(s: &str) -> (Option<Box<dyn Expr<T>>>, &str) where
        OpMinus<T>: Op1Common<T>,
        OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
    {
        if let (Some(mut f1), mut s) = factor::<T>(s) {
            loop {
                s = s.trim_start();
                if s.is_empty() {
                    return (Some(f1), &s);
                }
                let mut op: Option<Box<dyn Op2Common<T>>>;
                match s.chars().next().unwrap() {
                    '*' => op = Some(Box::new(OpMul::<T>::new())),
                    '/' => op = Some(Box::new(OpDiv::<T>::new())),
                    _ => return (Some(f1), &s)
                }
                s = &s[1..];
                if let(Some(f2), s2) = factor::<T>(s) {
                    s = &s2;
                    op.as_deref_mut().unwrap().set_left(f1);
                    op.as_deref_mut().unwrap().set_right(f2);
                    f1 = op.unwrap().as_dyn_expr();
                } else {
                    return (None, s);
                }
            }
        } else {
            (None, &s)
        }
    }

    fn factor<T>(s: &str) -> (Option<Box<dyn Expr<T>>>, &str) where
        OpMinus<T>: Op1Common<T>,
        OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
    {
        let mut s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let mut m: Option<Box<dyn Op1Common<T>>> = None;
        match s.chars().next().unwrap() {
            '-' => {
                m = Some(Box::new(OpMinus::<T>::new()));
                s = &s[1..];
            }
            _ => {}
        }
        s = s.trim_start();
        if s.is_empty() {
            return (None, s);
        }
        let mut e: Option<Box<dyn Expr<T>>>;
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
        if m.is_some() {
            m.as_deref_mut().unwrap().set_child(e.unwrap());
            e = Some(m.unwrap().as_dyn_expr());
        }
        (e, s)
    }

    fn terminal<T>(s: &str) -> (Option<Box<dyn Expr<T>>>, &str) where
        OpMinus<T>: Op1Common<T>,
        OpAdd<T>: Op2Common<T>, OpSub<T>: Op2Common<T>, OpMul<T>: Op2Common<T>, OpDiv<T>: Op2Common<T>
    {
        (None, s)
    }
}
