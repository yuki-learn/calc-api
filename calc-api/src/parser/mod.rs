#[cfg(test)]
mod tests;

use core::panic;

use nom::branch::alt;
use nom::character::complete::{digit1, char, space0};
use nom::IResult;
use nom::combinator::{map, opt, map_res};
use nom::error::{Error};
use nom::sequence::{delimited};

use crate::ast::{Const, Expr};

/// 空白含む数字をパース
pub fn const_val_parser(s: &str) -> IResult<&str, Const> {
    let (s, val): (&str, i32) = map_res(delimited(space0, digit1, space0), |x: &str| x.parse::<i32>())(s)?;
    Ok((s, Const::new(val)))
}

fn atom_parser(s: &str) -> IResult<&str, Expr> {
    let parentheses_parser = delimited(char('('), expr_parser, char(')'));

    alt((
        delimited(space0, parentheses_parser, space0),
        map(
            const_val_parser, 
            |const_val| Expr::Const(const_val)
        ),
    ))(s)
}

fn more_factors((s, a): (&str, Expr)) -> IResult<(&str, Expr), Expr, &str> {
    let (s, opt_op_char) = opt::<&str, char, Error<&str>, _>(alt((char('*'), char('/'))))(s).unwrap_or(("", None));

    // 演算子がなければ引数の式をそのまま返す
    if opt_op_char.is_none() {
        return Ok(((s, Expr::Const(Const::new(0))), a));
    }

    // 演算子があれば次の式
    let atom_result = atom_parser(s);
    if atom_result.is_err() {
        return Err(nom::Err::Error("atom_error"));
    }

    let (s, b) = atom_result.unwrap();

    // 引数の式とパースした次の式を足し算 or 引き算
    let op_char = opt_op_char.unwrap();
    match op_char {
        '*' => more_factors((s, Expr::Mul(Box::new(a), Box::new(b)))),
        '/' => more_factors((s, Expr::Div(Box::new(a), Box::new(b)))),
        _ => panic!("error")
    }
}

fn more_terms((s, a): (&str, Expr)) -> IResult<(&str, Expr), Expr, &str> {
    let (s, opt_op_char) = opt::<&str, char, Error<&str>, _>(alt((char('+'), char('-'))))(s).unwrap_or(("", None));

    // 演算子がなければ引数の式をそのまま返す
    if opt_op_char.is_none() {
        return Ok(((s, Expr::Const(Const::new(0))), a));
    }

    // 演算子があるので次のパース
    let term_result = term_parser(s);
    if term_result.is_err() {
        return Err(nom::Err::Error("term_error"));
    }

    let (s, b) = term_result.unwrap();

    // 引数の式とパースした次の式を足し算 or 引き算
    let op_char = opt_op_char.unwrap();
    match op_char {
        '+' => more_terms((s, Expr::Add(Box::new(a), Box::new(b)))),
        '-' => more_terms((s, Expr::Sub(Box::new(a), Box::new(b)))),
        _ => panic!("error")
    }
}

fn term_parser(s: &str) -> IResult<&str, Expr> {
    let (s, expr) = atom_parser(s)?;

    let more_factors_result = more_factors((s, expr));

    if more_factors_result.is_ok() {
        let ((s, _), e2) = more_factors_result.unwrap();
        Ok((s, e2))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Not)))
    }
}

pub fn expr_parser(s: &str) -> IResult<&str, Expr> {
    let (s, expr) = term_parser(s)?;

    let more_terms_result = more_terms((s, expr));
    let a = nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Not));
    if more_terms_result.is_ok() {
        let ((s, _), e2) = more_terms_result.unwrap();
        Ok((s, e2))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Not)))
    }
}