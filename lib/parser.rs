use std::str::FromStr;

use crate::{
    algebra::{Complex, Ring},
    expression::{ComplexFunction, Expr, FieldFunction, FieldOperator, Variable},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, peek, value},
    error::Error,
    multi::fold_many0,
    number::complete::float,
    sequence::{delimited, pair, preceded, tuple},
    Finish, IResult,
};

trait Parseable: Clone {
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parseable for FieldOperator {
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            value(FieldOperator::Add, char('+')),
            value(FieldOperator::Sub, char('-')),
            value(FieldOperator::Mul, char('*')),
            value(FieldOperator::Div, char('/')),
        ))(i)
    }
}

impl Parseable for FieldFunction {
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            value(FieldFunction::Neg, char('-')),
            value(FieldFunction::Inv, tag("inv")),
        ))(i)
    }
}

impl Parseable for ComplexFunction {
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            value(ComplexFunction::Re, tag("Re")),
            value(ComplexFunction::Im, tag("Im")),
            value(ComplexFunction::Abs, tag("abs")),
        ))(i)
    }
}

impl Parseable for f32 {
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        float(i)
    }
}

impl<T> Parseable for Complex<T>
where
    T: Parseable + Ring,
{
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((map(T::parse, Complex::from), value(Complex::I, char('i'))))(i)
    }
}

fn var<T, F, O>(i: &str) -> IResult<&str, Expr<T, F, O>>
where
    T: Clone,
    F: Clone,
    O: Clone,
{
    alt((
        value(Expr::Variable(Variable::Z), char('z')),
        value(Expr::Variable(Variable::C), char('c')),
    ))(i)
}

fn term<T, F, O>(i: &str) -> IResult<&str, Expr<T, F, O>>
where
    T: Parseable,
    F: Parseable,
    O: Parseable,
{
    alt((
        delimited(tag("("), add, tag(")")),
        var,
        map(T::parse, |ct: T| Expr::Constant(ct)),
        map(pair(F::parse, term), |(fun, t)| {
            Expr::Function(fun, Box::new(t))
        }),
    ))(i)
}

fn mul<T, F, O>(i: &str) -> IResult<&str, Expr<T, F, O>>
where
    T: Parseable,
    F: Parseable,
    O: Parseable,
{
    let (i, init) = term(i)?;
    let mul_or_div_op = preceded(peek(one_of("*/")), O::parse);
    fold_many0(
        tuple((mul_or_div_op, term)),
        move || init.clone(),
        |left, (op, right)| Expr::Operator(op, Box::new(left), Box::new(right)),
    )(i)
}

fn add<T, F, O>(i: &str) -> IResult<&str, Expr<T, F, O>>
where
    T: Parseable,
    F: Parseable,
    O: Parseable,
{
    let (i, init) = mul(i)?;
    let add_or_sub_op = preceded(peek(one_of("+-")), O::parse);
    fold_many0(
        tuple((add_or_sub_op, mul)),
        move || init.clone(),
        |left, (op, right)| Expr::Operator(op, Box::new(left), Box::new(right)),
    )(i)
}

impl<T, F, O> Parseable for Expr<T, F, O>
where
    T: Parseable,
    F: Parseable,
    O: Parseable,
{
    fn parse(i: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        add(i)
    }
}

impl<T, F, O> FromStr for Expr<T, F, O>
where
    T: Parseable,
    F: Parseable,
    O: Parseable,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, name)) => Ok(name),
            Err(Error { input, code }) => Err(format!("Bad parse '{}': {:?}", input, code)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Parseable;
    use crate::{
        algebra::{Complex, Field},
        expression::{Expr, FieldFunction, FieldOperator, Variable},
    };

    fn parse_and_eval<T>(input: &str) -> T
    where
        T: Parseable + Field,
    {
        let mut values = HashMap::new();
        values.insert(Variable::Z, T::O);
        input
            .parse::<Expr<T, FieldFunction, FieldOperator>>()
            .unwrap()
            .eval(&values)
    }

    #[test]
    fn literal() {
        assert_eq!(parse_and_eval::<f32>("1"), 1.0)
    }

    #[test]
    fn variable() {
        assert_eq!(parse_and_eval::<f32>("z"), 0.0)
    }

    #[test]
    fn parens() {
        assert_eq!(parse_and_eval::<f32>("(2)"), 2.0)
    }

    #[test]
    fn sum() {
        assert_eq!(parse_and_eval::<f32>("2+4"), 6.0)
    }

    #[test]
    fn sum_2() {
        assert_eq!(parse_and_eval::<f32>("2+4+5"), 11.0)
    }

    #[test]
    fn prod() {
        assert_eq!(parse_and_eval::<f32>("2*4"), 8.0)
    }

    #[test]
    fn prod_2() {
        assert_eq!(parse_and_eval::<f32>("3*4*5"), 60.0)
    }

    #[test]
    fn div_div() {
        assert_eq!(parse_and_eval::<f32>("2/2/2"), 0.5)
    }

    #[test]
    fn mix_1() {
        assert_eq!(parse_and_eval::<f32>("3*4+5"), 17.0)
    }

    #[test]
    fn mix_2() {
        assert_eq!(parse_and_eval::<f32>("-(3+2)"), -5.0)
    }

    #[test]
    fn complex() {
        assert_eq!(parse_and_eval::<Complex<f32>>("1*i"), Complex::I)
    }

    #[test]
    fn complex_2() {
        assert_eq!(parse_and_eval::<Complex<f32>>("-1*i"), -Complex::I)
    }

    #[test]
    fn complex_3() {
        assert_eq!(
            parse_and_eval::<Complex<f32>>("(2-i)*(2+i)"),
            Complex::from(5.0)
        )
    }
}
