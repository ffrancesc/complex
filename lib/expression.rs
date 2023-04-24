use crate::{algebra::Complex, algebra::Field};

pub trait Function<T> {
    fn apply(&self, val: T) -> T;
}

pub trait Operator<T> {
    fn apply(&self, left: T, right: T) -> T;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<T, F, O> {
    Variable,
    Constant(T),
    Function(F, Box<Expr<T, F, O>>),
    Operator(O, Box<Expr<T, F, O>>, Box<Expr<T, F, O>>),
}

impl<T, F, O> Expr<T, F, O>
where
    T: Clone,
    F: Function<T>,
    O: Operator<T>,
{
    pub fn eval(&self, t: &T) -> T {
        match &self {
            Expr::Variable => t.clone(),
            Expr::Constant(ct) => ct.clone(),
            Expr::Function(fun, exp) => {
                let val = exp.eval(t);
                fun.apply(val)
            }
            Expr::Operator(op, left, right) => {
                let left = left.eval(t);
                let right = right.eval(t);
                op.apply(left, right)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl<T> Operator<T> for FieldOperator
where
    T: Field,
{
    fn apply(&self, a: T, b: T) -> T {
        match &self {
            FieldOperator::Add => a + b,
            FieldOperator::Sub => a - b,
            FieldOperator::Mul => a * b,
            FieldOperator::Div => a / b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldFunction {
    Neg,
    Inv,
}

impl<T> Function<T> for FieldFunction
where
    T: Field,
{
    fn apply(&self, val: T) -> T {
        match &self {
            FieldFunction::Neg => -val,
            FieldFunction::Inv => T::U / val,
        }
    }
}

pub type ExprComplex = Expr<Complex<f32>, FieldFunction, FieldOperator>;

impl Default for ExprComplex {
    fn default() -> Self {
        Expr::Variable
    }
}
