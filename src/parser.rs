#![allow(dead_code, unused_imports)]
use nom::ParseTo;
use nom::character::complete::{digit0, multispace0, digit1, multispace1};
use nom::combinator::map;
use nom::multi::{separated_list0, many0};
use nom::sequence::{delimited, tuple};
use nom::{character::complete::char, IResult};
use nom::branch::alt;

fn plus_operator(input: &str) -> IResult<&str, Op> {

    map(char('+'), |s| Op::new(s))(input)
}
fn minus_operator(input: &str) -> IResult<&str, Op> {
    map(char('-'), |s| Op::new(s))(input)
}
fn multiply_operator(input: &str) -> IResult<&str, Op> {
    map(char('*'), |s| Op::new(s))(input)
}
fn divide_operator(input: &str) -> IResult<&str, Op> {
    map(char('/'), |s| Op::new(s))(input)
}

fn operator(input: &str) -> IResult<&str, Op> {
    alt((plus_operator, minus_operator, multiply_operator, divide_operator))(input)

}

fn number(input: &str) -> IResult<&str, Expression> {
    map(digit1, |s: &str| Expression::Number(s.parse::<u64>().unwrap()))(input)
}

pub fn numbers0(input: &str) -> IResult<&str, Vec<Expression>> {
    separated_list0(multispace1, number)(input)
}


pub fn test_input(input: &str) -> IResult<&str, Expression> {
    println!("input: {:#?}", input);
    map(
        delimited(
            tuple((char('('), multispace0)), 
            tuple((
                operator,
                multispace0,
                separated_list0(
                    multispace1,
                    alt((
                        number,
                        test_input
                    ))
                ))
            ), 
            tuple((multispace0, char(')')))
        ),
        Expression::from_tuple
    )(input)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Op {
    Plus,
    Minus,
    Divide,
    Multiply,
    Other
}

impl Op {
    fn new(input: char) -> Self {
        match input {
            '+' => Op::Plus,
            '-' => Op::Minus,
            '*' => Op::Multiply,
            '/' => Op::Divide,
            _ => Op::Other
        }
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Number(u64),
    Expr(Expr)
}

impl Expression {

    fn from_tuple(t: (Op, &str, Vec<Expression>)) -> Self {
        println!("tuple: {:#?}", t);
        Expression::Expr(Expr::from_tuple(t))
    }
}
impl From<u64> for Expression {

    fn from(number: u64) -> Self {
        Expression::Number(number)
    }

}
impl From<Expr> for Expression {

    fn from(expr: Expr) -> Self {
        Expression::Expr(expr)
    }

}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Expr {
    operator: Option<Op>,
    operand1: Vec<Expression>
}

impl Expr {
    fn new() -> Self {
        Expr { operator: None, operand1: Vec::new() }
    }

    fn from_tuple(t: (Op, &str, Vec<Expression>)) -> Self {
        let (op, _, exprs) = t;
        Expr { operator: Some(op), operand1: exprs }
    }

}

#[cfg(test)]
mod test{
use super::*;

#[test]
fn test_plus_operator() {
    assert_eq!(plus_operator("+"), Ok(("", Op::Plus)));
    assert_eq!(minus_operator("-"), Ok(("", Op::Minus)));
    assert_eq!(multiply_operator("*"), Ok(("", Op::Multiply)));
    assert_eq!(divide_operator("/"), Ok(("", Op::Divide)));
}

#[test]
fn test_operator() {
    assert_eq!(operator("+"), Ok(("", Op::Plus)));
    assert_eq!(operator("-"), Ok(("", Op::Minus)));
    assert_eq!(operator("*"), Ok(("", Op::Multiply)));
    assert_eq!(operator("/"), Ok(("", Op::Divide)));
}
#[test]
fn test_number() {
    assert_eq!(number("12"), Ok(("", Expression::Number(12))))
}

#[test]
fn test_numbers0() {
    let test_data = vec![Expression::Number(1), Expression::Number(12), Expression::Number(345)];
    assert_eq!(numbers0("1 12 345").unwrap().1.iter().count(), 3);
    assert_eq!(numbers0("1 12 345").unwrap().1, test_data)

}}