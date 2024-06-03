use expr::Expr;
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

fn run<T: std::fmt::Display>() -> ExitCode {
    if let Some(e) = parse::<T>() {
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

fn parse<T>() -> Option<Box<dyn Expr<T>>> {
    let e = parser::expression::<T>();
    
    e
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

    mod op1 {
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
        pub(super) trait Op1Common<T> {
            fn child(&self) -> &dyn Expr<T>;
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
            child: Box<dyn Expr<T>>,
        }
        impl<T> OpMinus<T> {
            pub fn new(child: Box<dyn Expr<T>>) -> OpMinus<T> {
                OpMinus {
                    child,
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
        impl<T> Op1Common<T> for OpMinus<T> {
            fn child(&self) -> &dyn Expr<T> {
                self.child.as_ref()
            }
            fn display_op(&self) {
                print!("-");
            }
        }
    }

    mod op2 {
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
        pub(super) trait Op2Common<T> {
            fn left(&self) -> &dyn Expr<T>;
            fn right(&self) -> &dyn Expr<T>;
            fn display_op(&self);
        }
        pub(super) trait Op2Evaluator<T> {
            fn eval_op(a: &Option<T>, b: &Option<T>) -> Option<T>;
        }
    }

    mod op_add {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};

        pub struct OpAdd<T> {
            left: Box<dyn Expr<T>>,
            right: Box<dyn Expr<T>>,
        }
        impl<T> OpAdd<T> {
            pub fn new(left: Box<dyn Expr<T>>, right: Box<dyn Expr<T>>) -> OpAdd<T> {
                OpAdd {
                    left,
                    right,
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
        impl<T> Op2Common<T> for OpAdd<T> {
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref()
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref()
            }
            fn display_op(&self) {
                print!("+");
            }
        }
    }

    mod op_sub {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};

        pub struct OpSub<T> {
            left: Box<dyn Expr<T>>,
            right: Box<dyn Expr<T>>,
        }
        impl<T> OpSub<T> {
            pub fn new(left: Box<dyn Expr<T>>, right: Box<dyn Expr<T>>) -> OpSub<T> {
                OpSub {
                    left,
                    right,
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
        impl<T> Op2Common<T> for OpSub<T> {
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref()
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref()
            }
            fn display_op(&self) {
                print!("-");
            }
        }
    }

    mod op_mul {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};
        use super::op_sub::OpNoOp2;

        pub struct OpMul<T> {
            left: Box<dyn Expr<T>>,
            right: Box<dyn Expr<T>>,
        }
        impl<T> OpMul<T> {
            pub fn new(left: Box<dyn Expr<T>>, right: Box<dyn Expr<T>>) -> OpMul<T> {
                OpMul {
                    left,
                    right,
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
        impl<T> Op2Common<T> for OpMul<T> {
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref()
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref()
            }
            fn display_op(&self) {
                print!("*");
            }
        }
    }

    mod op_div {
        use super::Expr;
        use super::op2::{Op2, Op2Common, Op2Evaluator};
        use super::op_sub::OpNoOp2;

        pub struct OpDiv<T> {
            left: Box<dyn Expr<T>>,
            right: Box<dyn Expr<T>>,
        }
        impl<T> OpDiv<T> {
            pub fn new(left: Box<dyn Expr<T>>, right: Box<dyn Expr<T>>) -> OpDiv<T> {
                OpDiv {
                    left,
                    right,
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
        impl<T> Op2Common<T> for OpDiv<T> {
            fn left(&self) -> &dyn Expr<T> {
                self.left.as_ref()
            }
            fn right(&self) -> &dyn Expr<T> {
                self.right.as_ref()
            }
            fn display_op(&self) {
                print!("/");
            }
        }
    }
}

mod parser {
    use std::io;

    pub fn expression<T>() -> Option<Box<dyn Expr<T>>> {
        None
    }
}
